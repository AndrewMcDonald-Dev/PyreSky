use serde::{Deserialize, Serialize};

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
pub struct SimpleRecord {
    pub cid: String,
    pub uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct External {
    pub description: String,
    pub thumb: Option<ImageAlt>,
    pub title: String,
    pub uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub alt: String,
    #[serde(rename = "aspectRatio")]
    pub aspect_ratio: Option<AspectRatio>,
    pub image: ImageAlt,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AspectRatio {
    pub height: u32,
    pub width: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageAlt {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Facet {
    pub features: Vec<Feature>,
    pub index: Index,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Feature {
    #[serde(flatten)]
    pub type_: FeatureEnum,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum FeatureEnum {
    #[serde(rename = "app.bsky.richtext.facet#tag")]
    Tag { tag: String },
    #[serde(rename = "app.bsky.richtext.facet#link")]
    Link { uri: String },
    #[serde(rename = "app.bsky.richtext.facet#mention")]
    Mention { did: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Index {
    #[serde(rename = "byteStart")]
    pub byte_start: u32,
    #[serde(rename = "byteEnd")]
    pub byte_end: u32,
}
