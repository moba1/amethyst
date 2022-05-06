pub mod attribute;

use crate::result;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Image {
    modules: Vec<super::module::Module>,
    #[serde(flatten)]
    attribute: attribute::Attribute
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

    pub fn attribute(self) -> attribute::Attribute {
        self.attribute
    }
}
