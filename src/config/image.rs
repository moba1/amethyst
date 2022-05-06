mod typ;

use crate::result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Image<Script> {
    #[serde(rename(deserialize = "modules"))]
    pub scripts: Vec<Script>,
    pub base_image: typ::ImageType,
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
