use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleRecord {
    pub cid: String,
    pub uri: String,
}
