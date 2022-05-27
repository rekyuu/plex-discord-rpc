#[derive(Debug)]
pub struct CacheObject {
    pub session_key: String,
    pub title: String,
    pub episode_title: Option<String>,
    pub season: Option<i32>,
    pub current_episode: Option<i32>,
    pub duration: u64,
    pub media_kind: MediaKind
}

#[derive(Debug)]
pub enum MediaKind {
    Movie,
    TV
}