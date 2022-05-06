use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::default;

#[derive(Debug, Clone)]
pub enum ImageType {
    Scratch,
    BaseImage(String),
}

impl Serialize for ImageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let image_name = match self {
            ImageType::Scratch => "scratch",
            ImageType::BaseImage(base_image) => &base_image,
        };
        serializer.serialize_str(image_name)
    }
}

impl<'de> Deserialize<'de> for ImageType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
            D: Deserializer<'de> 
    {
        let image_type: Option<String> = Deserialize::deserialize(deserializer)?;
        match image_type {
            None => Ok(ImageType::Scratch),
            Some(path) => Ok(ImageType::BaseImage(path)),
        }
    }
}

impl default::Default for ImageType {
    fn default() -> Self {
        ImageType::Scratch
    }
}
