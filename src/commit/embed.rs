use serde::{Deserialize, Serialize};

use crate::commit::{
    AspectRatio, External, Image, Media, SimpleRecord, SimpleRecordWithMedia, Video,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Embed {
    #[serde(flatten)]
    pub type_: EmbedEnum,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum EmbedEnum {
    #[serde(rename = "app.bsky.embed.images")]
    Images { images: Vec<Image> },
    #[serde(rename = "app.bsky.embed.external")]
    External { external: External },
    #[serde(rename = "app.bsky.embed.record")]
    Record { record: SimpleRecord },
    #[serde(rename = "app.bsky.embed.recordWithMedia")]
    RecordWithMedia {
        record: SimpleRecordWithMedia,
        media: Media,
    },
    #[serde(rename = "app.bsky.embed.video")]
    Video {
        video: Video,
        aspect_ratio: Option<AspectRatio>,
    },
}
