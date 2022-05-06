pub mod typ;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    pub base_image: typ::ImageType,
    pub name: String,
}
