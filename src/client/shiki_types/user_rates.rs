use crate::client::shiki_types::UserId;
use crate::shiki_types::AnimeId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum UserRateStatus {
    Planned,
    Watching,
    Rewatching,
    Completed,
    OnHold,
    Dropped,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UserRateId(u64);

#[derive(Serialize, Deserialize, Debug)]
pub enum TargetType {
    Anime,
    Manga,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(transparent)]
pub struct TargetId(u64);

impl Into<u64> for TargetId {
    fn into(self) -> u64 {
        self.0
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserRate {
    pub id: UserRateId,
    pub user_id: UserId,
    pub target_id: TargetId,
    pub target_type: TargetType,
    pub score: u64,
    pub status: UserRateStatus,
    pub rewatches: u64,
    pub episodes: u64,
    pub volumes: u64,
    pub chapters: u64,
    pub text: Option<String>,
    pub text_html: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserRate {
    pub fn anime_id(&self) -> Option<AnimeId> {
        match self.target_type {
            TargetType::Anime => {
                let raw_id: u64 = self.target_id.clone().into();
                Some(AnimeId::from(raw_id))
            }
            TargetType::Manga => None,
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub struct UserRatesParams {
    pub user_id: Option<UserId>,
    pub target_id: Option<TargetId>,
    pub target_type: Option<TargetType>,
    pub status: Option<UserRateStatus>,
    pub page: Option<u64>,
    pub limit: Option<u64>,
}

impl From<UserId> for UserRatesParams {
    fn from(user_id: UserId) -> Self {
        UserRatesParams {
            user_id: Some(user_id),
            ..UserRatesParams::default()
        }
    }
}

pub type UserRateResponse = Vec<UserRate>;
