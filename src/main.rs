mod cache_object;
mod config;
mod constants;
mod media_container;
mod notification_container;

use crate::constants::{ACTIVE_TIME, IDLE_TIME};
use crate::cache_object::{CacheObject, MediaKind};
use crate::config::{Config, Discord as DiscordConfig};
use crate::media_container::{MediaContainer, Video};
use crate::notification_container::NotificationContainerRoot;

use std::{thread, time};
use std::time::{SystemTime, UNIX_EPOCH};
use std::net::TcpStream;
use discord_rpc_client::Client as DiscordClient;
use discord_rpc_client::models::{Activity, ActivityAssets};
use discord_rpc_client::models::payload::Payload;
use quick_xml::de::from_str;
use websocket::client::builder::ClientBuilder;
use websocket::native_tls::TlsStream;
use websocket::sync::Client;
use websocket::url::Url;
use websocket::ws::dataframe::DataFrame;

#[tokio::main]
async fn main() {
    // Load the configuration file.
    let config = Config::load();
    let plex_config = config.plex.unwrap();
    let discord_config = config.discord.unwrap();

    let username = &plex_config.username.unwrap();
    let server = &plex_config.host.unwrap();
    let token = &plex_config.token.unwrap();
    let client_id = discord_config.app_id.unwrap();

    // Format the WebSocket URL for Plex.
    let url = get_wss_plex_url(server, "/:/websockets/notifications", token);
    let wss = Url::parse(&url).unwrap();

    // Start the Discord RPC client.
    let mut discord = DiscordClient::new(client_id);
    discord.start();

    loop {
        // Clear the cache and continuously attempt to connect to the WebSocket.
        let mut now_playing_cache: Option<CacheObject> = None;
        let mut client = websocket_idle(&wss);

        loop {
            // Wait to receive WebSocket notifications.
            let response = match client.recv_message() {
                Ok(r) => r,
                Err(_) => {
                    println!("Disconnected from websocket.");
                    break;
                }
            };

            // Deserialize the payload.
            let payload = response.take_payload();
            let json_str = std::str::from_utf8(&payload).unwrap();
            let notification = serde_json::from_str::<NotificationContainerRoot>(json_str);

            match notification {
                Ok(root) => {
                    let notification = root.notification_container;

                    // Plex sends many notifications, but we only care about "playing".
                    if notification.type_field != "playing" { continue }

                    for state in &notification.play_session_state_notification {
                        let rpc_response: Result<Payload<Activity>, _>;

                        // Clear cache and Discord activity if the media is stopped or paused.
                        if &state.state == "paused" || &state.state == "stopped" {
                            rpc_response = discord.clear_activity();
                            now_playing_cache = None;

                            if let Err(e) = rpc_response {
                                eprintln!("Issue clearing Discord activity: {}", e);
                            }

                            continue;
                        }

                        // Populate the cache for currently playing media.
                        // We do this so we don't have to hit the HTTPS endpoint for each update.
                        if now_playing_cache.is_none() {
                            println!("Updating now watching cache.");

                            // Validate that the media being played is being watched by the specified user.
                            let video = match get_session_for_username(server, token, username).await {
                                Ok(might_be_video) => match might_be_video {
                                    Some(v) => v,
                                    None => continue // no sessions
                                },
                                Err(_) => continue // issue reaching Plex or user no sessions
                            };

                            let title = video.title.unwrap();

                            // Set the cache object.
                            now_playing_cache = Some(CacheObject {
                                session_key: video.session_key.unwrap(),
                                title: match &video.grandparent_title {
                                    Some(series_title) => series_title.to_string(),
                                    None => title.to_string()
                                },
                                episode_title: match &video.grandparent_title {
                                    Some(_) => Some(title.to_string()),
                                    None => None
                                },
                                season: video.parent_index,
                                current_episode: video.index,
                                duration: video.duration.unwrap(),
                                media_kind: match &video.grandparent_title {
                                    Some(_) => MediaKind::TV,
                                    None => MediaKind::Movie
                                }
                            });
                        }

                        // Grab the current cache object and validate that the
                        // media being played is being watched by the specified user.
                        let cache = now_playing_cache.as_ref().unwrap();
                        if state.session_key != cache.session_key { continue }

                        // Do specific formatting for Discord RPC depending on the media type.
                        match cache.media_kind {
                            MediaKind::Movie => {
                                rpc_response = discord.set_activity(|activity| {
                                    activity
                                        .state(&cache.title)
                                        .assets(|assets| {
                                            format_assets(assets, &discord_config)
                                        })
                                        .timestamps(|ts| {
                                            ts.start(get_start_timestamp_secs(&state.view_offset))
                                        })
                                });
                            }
                            MediaKind::TV => {
                                let episode_title = &cache.episode_title.as_ref().unwrap().to_string();
                                let details = format!("S{} · E{} — {}", cache.season.unwrap(), cache.current_episode.unwrap(), episode_title);

                                rpc_response = discord.set_activity(|activity| {
                                    activity
                                        .details(&cache.title)
                                        .state(details)
                                        .assets(|assets| {
                                            format_assets(assets, &discord_config)
                                        })
                                        .timestamps(|ts| {
                                            ts.start(get_start_timestamp_secs(&state.view_offset))
                                        })
                                });
                            }
                        }

                        // Log Discord RPC errors.
                        if let Err(e) = rpc_response {
                            eprintln!("Error setting Discord activity: {}", e);
                        }
                    }
                }
                // Skip useless notifications.
                Err(_) => continue
            }

            // Wait a bit so we're not hammering Discord (just in case).
            thread::sleep(time::Duration::from_secs(ACTIVE_TIME));
        }
    }
}

/// Gets the formatted Plex URL using the server token.
fn get_plex_url(server: &str, endpoint: &str, token: &str) -> String {
    return format!("{}{}?X-Plex-Token={}", server, endpoint, token);
}

/// Gets the formatted Plex WebSocket URL using the server token.
fn get_wss_plex_url(server: &str, endpoint: &str, token: &str) -> String {
    return format!("wss://{}", get_plex_url(server, endpoint, token));
}

/// Gets the formatted Plex HTTPS URL using the server token.
fn get_https_plex_url(server: &str, endpoint: &str, token: &str) -> String {
    return format!("https://{}", get_plex_url(server, endpoint, token));
}

/// Formats Discord RPC assets, including images and their alt text.
fn format_assets(mut assets: ActivityAssets, discord_config: &DiscordConfig) -> ActivityAssets {
    let large_image = discord_config.large_image.as_ref().unwrap();
    let small_image = discord_config.small_image.as_ref().unwrap();
    let large_text = discord_config.large_text.as_ref().unwrap();
    let small_text = discord_config.small_text.as_ref().unwrap();

    if large_image != "" {
        assets = assets.large_image(large_image)
    }

    if small_image != "" {
        assets = assets.small_image(small_image)
    }

    if large_text != "" {
        assets = assets.large_text(large_text)
    }

    if small_text != "" {
        assets = assets.small_text(small_text)
    }

    return assets;
}

/// Gets the start timestamp, considering the elapsed time. Used for the Discord RPC duration.
fn get_start_timestamp_secs(elapsed_ms: &u64) -> u64 {
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    return current_time - (elapsed_ms / 1000);
}

/// Gets XML data from the provided Plex Server API endpoint.
async fn get_plex_api(server: &str, endpoint: &str, token: &str) -> Result<String, reqwest::Error> {
    let url = get_https_plex_url(server, endpoint, token);
    let body = reqwest::get(&url)
        .await?
        .text()
        .await?;

    return Ok(body);
}

/// Gets Plex Server session data for the provided username.
async fn get_session_for_username(server: &str, token: &str, username: &str) -> Result<Option<Video>, &'static str> {
    return match get_plex_api(server, "/status/sessions", token).await {
        Ok(xml) => {
            let media_container: MediaContainer = from_str(xml.as_str()).unwrap();

            if media_container.video.is_none() { return Ok(None); }

            for video in media_container.video.unwrap() {
                if video.session_key.is_none() { continue }
                if video.user.title.is_none() { continue }
                if video.user.title.as_ref().unwrap() != username { continue }

                return Ok(Some(video));
            }

            Err("Could not find username in sessions.")
        }
        Err(_) => {
            Err("Issue requesting sessions from Plex.")
        }
    }
}

/// Attempts reconnecting to the WebSocket server every so often.
/// Issues connecting will be logged.
fn websocket_idle(websocket_url: &Url) -> Client<TlsStream<TcpStream>> {
    println!("Entering idle mode.");

    let mut last_error = String::from("");

    let mut builder = ClientBuilder::from_url(websocket_url);
    loop {
        match builder.connect_secure(None) {
            Ok(conn) => {
                println!("Exiting idle mode.");
                return conn;
            }
            Err(e) => {
                let current_error = e.to_string();

                if &last_error != &current_error {
                    last_error = current_error;
                    eprintln!("{}", last_error);
                }
            }
        }

        thread::sleep(time::Duration::from_secs(IDLE_TIME));
    }
}