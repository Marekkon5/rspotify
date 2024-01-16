//! Image object

use serde::{Deserialize, Serialize};

/// Image object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Image {
    pub height: Option<f32>,
    pub url: String,
    pub width: Option<f32>,
}
