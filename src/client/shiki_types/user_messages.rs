use crate::shiki_types::{Limit, Page, UserId};
use chrono::{DateTime, Utc};
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
pub struct UserMessagesParams {
    #[serde(skip)]
    pub user_id: UserId,
    pub page: Option<Page>,
    pub limit: Option<Limit>,
    #[serde(rename = "type")]
    pub message_type: MessageType,
}

#[derive(Deserialize, Debug)]
pub struct UserMessageResponce {
    pub id: u64,
    pub kind: String,
    pub read: bool,
    pub body: String,
    pub html_body: String,
    pub created_at: DateTime<Utc>,
    pub linked: Linked,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "linked_type")]
pub enum Linked {
    Topic(Topic),
}

#[derive(Deserialize, Debug)]
pub struct Topic {
    pub id: u64,
    pub topic_url: String,
    pub thread_id: u64,
    pub topic_id: u64,
    #[serde(rename = "type")]
    pub topic_type: String,
}
