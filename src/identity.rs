use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Identity {
    pub did: String,
    pub handle: String,
    pub seq: u64,
    pub time: String,
}
