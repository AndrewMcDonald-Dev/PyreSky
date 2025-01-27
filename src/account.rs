use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub did: String,
    pub active: bool,
    pub seq: u64,
    pub time: String,
}
