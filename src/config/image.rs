use crate::result;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize, Clone)]
pub enum ImageType {
    Scratch,
    BaseImage(String),
}

#[derive(Debug, Deserialize, Clone)]
pub struct Image {
    modules: Vec<super::module::Module>,
    #[serde(
        default = "default_base_image",
        deserialize_with = "deserialize_base_image"
    )]
    base_image: ImageType,
    name: String,
}

fn default_base_image() -> ImageType {
    ImageType::Scratch
}

fn deserialize_base_image<'de, D>(deserializer: D) -> Result<ImageType, D::Error>
where
    D: Deserializer<'de>,
{
    let image_type: Option<String> = Deserialize::deserialize(deserializer)?;
    match image_type {
        None => Ok(ImageType::Scratch),
        Some(path) => Ok(ImageType::BaseImage(path)),
    }
}

impl Image {
    pub fn slurp_scriptlets(self) -> result::Result<Vec<super::scriptlet::Scriptlet>> {
        let scriptlets = self
            .modules
            .into_iter()
            .map(|module| module.to_scriptlets())
            .collect::<Result<Vec<_>, _>>()?
            .concat();
        Ok(scriptlets)
    }

    pub fn base_image(self) -> ImageType {
        self.base_image
    }
}
