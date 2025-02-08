use serde::{Deserialize, Serialize};

use crate::Ref;

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
