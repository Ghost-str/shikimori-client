use crate::client::shiki_types::UserId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WhoAmIResponce {
    pub id: UserId,
    pub nickname: String,
    pub avatar: String,
}
