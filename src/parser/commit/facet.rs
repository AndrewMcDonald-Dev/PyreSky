use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Facet {
    pub features: Vec<Feature>,
    pub index: Index,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Index {
    #[serde(rename = "byteStart")]
    byte_start: u32,
    #[serde(rename = "byteEnd")]
    byte_end: u32,
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
