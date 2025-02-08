use serde::{Deserialize, Serialize};

use crate::ImageAlt;

#[derive(Debug, Serialize, Deserialize)]
pub struct External {
    pub description: String,
    pub thumb: Option<ImageAlt>,
    pub title: String,
    pub uri: String,
}
