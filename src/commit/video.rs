use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Video {
    #[serde(rename = "$type")]
    pub type_: String,
    #[serde(rename = "ref")]
    pub ref_: Ref,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    pub size: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ref {
    #[serde(rename = "$link")]
    pub link: String,
}
