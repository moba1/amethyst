use serde::Deserialize;
use std::error;

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum Module {
    File(String),
    Inline(super::scriptlet::Scriptlet),
}

impl Module {
    pub fn to_scriptlets(
        self,
    ) -> Result<Vec<super::scriptlet::Scriptlet>, Box<dyn error::Error + Send + Sync + 'static>> {
        #[derive(Debug, Deserialize)]
        struct Scriptlets {
            scriptlets: Vec<super::scriptlet::Scriptlet>,
        }

        match self {
            Self::File(path) => {
                let raw_scriptlets = std::fs::read_to_string(path)?;
                let scriptlets: Scriptlets = toml::from_str(&raw_scriptlets)?;
                Ok(scriptlets.scriptlets)
            }
            Self::Inline(scriptlet) => Ok(vec![scriptlet]),
        }
    }
}
