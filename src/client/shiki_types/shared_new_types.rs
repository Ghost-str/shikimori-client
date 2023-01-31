use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(transparent)]
pub struct UserId(u64);

impl Display for UserId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(transparent)]
pub struct Page(u64);

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(transparent)]
pub struct Limit(u64);
