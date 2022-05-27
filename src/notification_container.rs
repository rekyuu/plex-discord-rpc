use serde::Deserialize;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationContainerRoot {
    #[serde(rename = "NotificationContainer")]
    pub notification_container: NotificationContainer
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationContainer {
    #[serde(rename = "type")]
    pub type_field: String,
    pub size: i64,
    #[serde(rename = "PlaySessionStateNotification")]
    pub play_session_state_notification: Vec<PlaySessionStateNotification>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaySessionStateNotification {
    pub session_key: String,
    pub client_identifier: String,
    pub guid: String,
    pub rating_key: String,
    pub url: String,
    pub key: String,
    pub view_offset: u64,
    #[serde(rename = "playQueueItemID")]
    pub play_queue_item_id: i64,
    #[serde(rename = "playQueueID")]
    pub play_queue_id: i64,
    pub state: String,
    pub transcode_session: String,
}
