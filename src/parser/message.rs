use serde::{Deserialize, Serialize};

use crate::Account;
use crate::Commit;
use crate::Identity;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub did: String,
    pub time_us: u64,
    #[serde(flatten)]
    pub kind: Kind,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Kind {
    #[serde(rename = "commit")]
    Commit { commit: Commit },
    #[serde(rename = "identity")]
    Identity { identity: Identity },
    #[serde(rename = "account")]
    Account { account: Account },
}
