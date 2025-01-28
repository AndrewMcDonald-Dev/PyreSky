use serde::{Deserialize, Serialize};

use crate::commit::{Embed, Facet};

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    pub rev: String,
    #[serde(flatten)]
    pub operation: Operation,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "operation")]
pub enum Operation {
    #[serde(rename = "create")]
    Create {
        collection: String,
        rkey: String,
        record: Box<Record>,
        cid: String,
    },
    #[serde(rename = "update")]
    Update,
    #[serde(rename = "delete")]
    Delete,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    #[serde(rename = "$type")]
    pub type_: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub embed: Option<Embed>,
    pub facets: Option<Vec<Facet>>,
    pub langs: Option<Vec<String>>,
    pub text: String,
}
