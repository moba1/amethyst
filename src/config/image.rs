mod typ;

use crate::result;
use serde::{de, Deserialize, Deserializer, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Image<Script> {
    #[serde(rename(deserialize = "modules"))]
    pub scripts: Vec<Script>,
    #[serde(default)]
    pub base_image: typ::ImageType,
    #[serde(deserialize_with = "deserialize_image_name")]
    pub name: String,
    pub tag: String,
}

impl Image<super::module::Module> {
    pub fn slurp_scriptlets(&self) -> result::Result<Vec<super::scriptlet::Scriptlet>> {
        let scriptlets = self
            .scripts
            .clone()
            .into_iter()
            .map(|module| module.to_scriptlets())
            .collect::<Result<Vec<_>, _>>()?
            .concat();
        Ok(scriptlets)
    }
}

fn deserialize_image_name<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let image_name = String::deserialize(deserializer)?;
    if image_name == typ::SCRATCH_IMAGE_NAME {
        return Err(de::Error::invalid_value(
            de::Unexpected::Str(typ::SCRATCH_IMAGE_NAME),
            &"other than scratch",
        ));
    }
    Ok(image_name)
}
