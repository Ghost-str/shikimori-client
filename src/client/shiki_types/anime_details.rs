use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy)]
#[serde(transparent)]
pub struct AnimeId(u64);

impl From<u64> for AnimeId {
    fn from(val: u64) -> Self {
        AnimeId(val)
    }
}

impl Into<u64> for AnimeId {
    fn into(self) -> u64 {
        self.0
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AnimeKind {
    Tv,
    Movie,
    Ova,
    Ona,
    Special,
    Music,
    Tv13,
    Tv24,
    Tv48,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeDetails {
    pub id: AnimeId,
    pub name: String,
    pub russian: String,
    pub kind: AnimeKind,
}
