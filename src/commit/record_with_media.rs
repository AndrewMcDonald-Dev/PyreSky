use serde::{Deserialize, Serialize};

use crate::commit::{External, Image, SimpleRecord, Video};

#[derive(Debug, Serialize, Deserialize)]
pub struct Media {
    #[serde(flatten)]
    pub type_: MedianEnum,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum MedianEnum {
    #[serde(rename = "app.bsky.embed.images")]
    Images { images: Vec<Image> },
    #[serde(rename = "app.bsky.embed.external")]
    External { external: External },
    #[serde(rename = "app.bsky.embed.video")]
    Video { video: Video },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordWithMedia {
    pub record: SimpleRecordWithMedia,
    pub media: Media,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleRecordWithMedia {
    #[serde(rename = "$type")]
    pub type_: String,
    pub record: SimpleRecord,
}
