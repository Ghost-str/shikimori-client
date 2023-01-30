use crate::shiki_types::{Limit, Page, UserId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    Inbox,
    Private,
    Sent,
    News,
    Notifications,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct UserMessagesParams {
    #[serde(skip)]
    pub user_id: UserId,
    pub page: Option<Page>,
    pub limit: Option<Limit>,
    #[serde(rename = "type")]
    pub message_type: MessageType,
}
