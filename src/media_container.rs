use serde::Deserialize;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaContainer {
    #[serde(rename = "Video")]
    pub video: Option<Vec<Video>>,
    pub size: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    #[serde(rename = "Media")]
    pub media: Option<Vec<Media>>,
    #[serde(rename = "Genre")]
    pub genre: Option<Vec<Genre>>,
    #[serde(rename = "Director")]
    pub director: Option<Vec<Director>>,
    #[serde(rename = "Writer")]
    pub writer: Option<Vec<Writer>>,
    #[serde(rename = "Producer")]
    pub producer: Option<Vec<Producer>>,
    #[serde(rename = "Country")]
    pub country: Option<Vec<Country>>,
    #[serde(rename = "Rating")]
    pub rating: Option<Vec<Rating>>,
    #[serde(rename = "Role")]
    pub role: Option<Vec<Role>>,
    #[serde(rename = "User")]
    pub user: User,
    #[serde(rename = "Player")]
    pub player: Player,
    #[serde(rename = "Session")]
    pub session: Session,
    #[serde(rename = "TranscodeSession")]
    pub transcode_session: TranscodeSession,
    pub added_at: Option<String>,
    pub art: Option<String>,
    pub audience_rating: Option<String>,
    pub audience_rating_image: Option<String>,
    pub chapter_source: Option<String>,
    pub content_rating: Option<String>,
    pub duration: Option<u64>,
    pub grandparent_art: Option<String>,
    pub grandparent_guid: Option<String>,
    pub grandparent_key: Option<String>,
    pub grandparent_rating_key: Option<String>,
    pub grandparent_theme: Option<String>,
    pub grandparent_thumb: Option<String>,
    pub grandparent_title: Option<String>,
    pub guid: Option<String>,
    pub index: Option<i32>,
    pub key: Option<String>,
    #[serde(rename = "librarySectionID")]
    pub library_section_id: Option<String>,
    pub library_section_key: Option<String>,
    pub library_section_title: Option<String>,
    pub originally_available_at: Option<String>,
    pub parent_guid: Option<String>,
    pub parent_index: Option<i32>,
    pub parent_key: Option<String>,
    pub parent_rating_key: Option<String>,
    pub parent_thumb: Option<String>,
    pub parent_title: Option<String>,
    pub primary_extra_key: Option<String>,
    #[serde(rename = "rating")]
    pub rating2: Option<String>,
    pub rating_image: Option<String>,
    pub rating_key: Option<String>,
    pub session_key: Option<String>,
    pub studio: Option<String>,
    pub summary: Option<String>,
    pub tagline: Option<String>,
    pub thumb: Option<String>,
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub updated_at: Option<String>,
    pub view_offset: Option<String>,
    pub year: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    #[serde(rename = "Part")]
    pub part: Part,
    pub id: Option<String>,
    pub video_profile: Option<String>,
    pub audio_channels: Option<String>,
    pub audio_codec: Option<String>,
    pub bitrate: Option<String>,
    pub container: Option<String>,
    pub duration: Option<i32>,
    pub height: Option<String>,
    pub protocol: Option<String>,
    pub video_codec: Option<String>,
    pub video_frame_rate: Option<String>,
    pub video_resolution: Option<String>,
    pub width: Option<String>,
    pub selected: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Part {
    #[serde(rename = "Stream")]
    pub stream: Option<Vec<Stream>>,
    pub id: Option<String>,
    pub video_profile: Option<String>,
    pub bitrate: Option<String>,
    pub container: Option<String>,
    pub duration: Option<String>,
    pub height: Option<String>,
    pub protocol: Option<String>,
    pub width: Option<String>,
    pub decision: Option<String>,
    pub selected: Option<String>
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stream {
    pub bit_depth: Option<String>,
    pub bitrate: Option<String>,
    pub chroma_location: Option<String>,
    pub chroma_subsampling: Option<String>,
    pub codec: Option<String>,
    pub coded_height: Option<String>,
    pub coded_width: Option<String>,
    pub display_title: Option<String>,
    pub extended_display_title: Option<String>,
    pub frame_rate: Option<String>,
    pub has_scaling_matrix: Option<String>,
    pub height: Option<String>,
    pub id: Option<String>,
    pub language: Option<String>,
    pub language_code: Option<String>,
    pub language_tag: Option<String>,
    pub level: Option<String>,
    pub profile: Option<String>,
    pub ref_frames: Option<String>,
    pub scan_type: Option<String>,
    pub stream_type: Option<String>,
    pub title: Option<String>,
    pub width: Option<String>,
    pub decision: Option<String>,
    pub location: Option<String>,
    pub bitrate_mode: Option<String>,
    pub channels: Option<String>,
    pub default: Option<String>,
    pub selected: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Genre {
    pub count: Option<String>,
    pub filter: Option<String>,
    pub id: Option<String>,
    pub tag: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Director {
    pub count: Option<String>,
    pub filter: Option<String>,
    pub id: Option<String>,
    pub tag: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Writer {
    pub count: Option<String>,
    pub filter: Option<String>,
    pub id: Option<String>,
    pub tag: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Producer {
    pub count: Option<String>,
    pub filter: Option<String>,
    pub id: Option<String>,
    pub tag: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Country {
    pub count: Option<String>,
    pub filter: Option<String>,
    pub id: Option<String>,
    pub tag: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rating {
    pub count: Option<String>,
    pub image: Option<String>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    pub count: Option<String>,
    pub filter: Option<String>,
    pub id: Option<String>,
    pub role: Option<String>,
    pub tag: Option<String>,
    pub thumb: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Option<String>,
    pub thumb: Option<String>,
    pub title: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub address: Option<String>,
    pub device: Option<String>,
    pub machine_identifier: Option<String>,
    pub model: Option<String>,
    pub platform: Option<String>,
    pub platform_version: Option<String>,
    pub product: Option<String>,
    pub profile: Option<String>,
    pub remote_public_address: Option<String>,
    pub state: Option<String>,
    pub title: Option<String>,
    pub version: Option<String>,
    pub local: Option<String>,
    pub relayed: Option<String>,
    pub secure: Option<String>,
    #[serde(rename = "userID")]
    pub user_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub id: Option<String>,
    pub bandwidth: Option<String>,
    pub location: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranscodeSession {
    pub key: Option<String>,
    pub throttled: Option<String>,
    pub complete: Option<String>,
    pub progress: Option<String>,
    pub size: Option<String>,
    pub speed: Option<String>,
    pub error: Option<String>,
    pub duration: Option<String>,
    pub remaining: Option<String>,
    pub context: Option<String>,
    pub source_video_codec: Option<String>,
    pub source_audio_codec: Option<String>,
    pub video_decision: Option<String>,
    pub audio_decision: Option<String>,
    pub protocol: Option<String>,
    pub container: Option<String>,
    pub video_codec: Option<String>,
    pub audio_codec: Option<String>,
    pub audio_channels: Option<String>,
    pub transcode_hw_requested: Option<String>,
    pub transcode_hw_full_pipeline: Option<String>,
    pub time_stamp: Option<String>,
    pub max_offset_available: Option<String>,
    pub min_offset_available: Option<String>,
}

