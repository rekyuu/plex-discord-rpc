use crate::constants::{
    CONFIG_FOLDER_NAME, CONFIG_FILE_NAME,
    DEFAULT_PLEX_HOST, DEFAULT_PLEX_USERNAME, DEFAULT_PLEX_TOKEN, DEFAULT_PLEX_TLS,
    DEFAULT_DISCORD_APP_ID,
    DEFAULT_DISCORD_LARGE_IMAGE, DEFAULT_DISCORD_SMALL_IMAGE,
    DEFAULT_DISCORD_LARGE_IMAGE_TEXT, DEFAULT_DISCORD_SMALL_IMAGE_TEXT
};

use merge::Merge;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Merge, Clone)]
pub struct Plex {
    pub(crate) host: Option<String>,
    pub(crate) username: Option<String>,
    pub(crate) token: Option<String>,
    pub(crate) tls: Option<bool>,
}

#[derive(Serialize, Deserialize, Merge, Clone)]
pub struct Discord {
    pub(crate) app_id: Option<u64>,
    pub(crate) large_image: Option<String>,
    pub(crate) small_image: Option<String>,
    pub(crate) large_text: Option<String>,
    pub(crate) small_text: Option<String>,
}

#[derive(Serialize, Deserialize, Merge, Clone)]
pub struct Config {
    pub(crate) plex: Option<Plex>,
    pub(crate) discord: Option<Discord>,
}

impl Default for Plex {
    fn default() -> Self {
        Plex {
            host: Some(DEFAULT_PLEX_HOST.to_string()),
            username: Some(DEFAULT_PLEX_USERNAME.to_string()),
            token: Some(DEFAULT_PLEX_TOKEN.to_string()),
            tls: Some(DEFAULT_PLEX_TLS),
        }
    }
}

impl Default for Discord {
    fn default() -> Self {
        Discord {
            app_id: Some(DEFAULT_DISCORD_APP_ID),
            large_image: Some(DEFAULT_DISCORD_LARGE_IMAGE.to_string()),
            small_image: Some(DEFAULT_DISCORD_SMALL_IMAGE.to_string()),
            large_text: Some(DEFAULT_DISCORD_LARGE_IMAGE_TEXT.to_string()),
            small_text: Some(DEFAULT_DISCORD_SMALL_IMAGE_TEXT.to_string())
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            plex: Some(Plex::default()),
            discord: Some(Discord::default())
        }
    }
}

impl Config {
    /// Loads the configuration file for the user.
    /// If no configuration file is found, it will be created with defaults.
    pub fn load() -> Config {
        let dir = Config::get_config_dir();
        let filepath = dir.join(CONFIG_FILE_NAME);

        if !filepath.exists() {
            Config::create_default_config_file()
                .expect("Unable to create default config file.");
        }

        let config_file = fs::File::open(&filepath).unwrap_or_else(|_| {
            panic!("Unable to load config file: {}", filepath.display());
        });

        let mut reader = BufReader::new(config_file);
        let mut contents = String::new();

        reader.read_to_string(&mut contents)
            .expect("Unable to read config file. Is the encoding UTF-8?");

        return toml::from_str::<Config>(contents.as_str())
            .expect("Unable to parse config file. Is it a valid TOML?")
            .merge_custom(Config::default());
    }

    /// Initializes the default configuration file.
    fn create_default_config_file() -> Result<(), std::io::Error> {
        println!("Creating default config.");
        let dir = Config::get_config_dir();
        let filepath = dir.join(CONFIG_FILE_NAME);
        let config = Config::default();

        fs::create_dir_all(Config::get_config_dir())?;
        let mut config_file_path = fs::File::create(filepath)?;

        config_file_path.write_all(toml::to_string(&config).unwrap().as_bytes()).unwrap();

        return Ok(())
    }

    /// Returns the configuration directory.
    fn get_config_dir() -> PathBuf {
        match dirs::config_dir() {
            Some(dir) => {
                dir.join(Path::new(CONFIG_FOLDER_NAME))
            },
            None => {
                dirs::config_dir()
                    .expect("Cannot get config folder or home directory.")
                    .join(format!(".{}", CONFIG_FOLDER_NAME))
            }
        }
    }

    /// Custom merge for the Config object and it's children.
    fn merge_custom(mut self, other: Config) -> Self {
        self.merge(other.clone());

        let mut plex = self.plex.unwrap();
        plex.merge(other.plex.unwrap());
        self.plex = Some(plex);

        let mut discord = self.discord.unwrap();
        discord.merge(other.discord.unwrap());
        self.discord = Some(discord);

        return self;
    }
}