use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum Module {
    File(String),
    Inline(super::scriptlet::Scriptlet),
}

#[derive(Debug, Deserialize)]
struct Scriptlets {
    scriptlets: Vec<super::scriptlet::Scriptlet>,
}

impl Module {
    pub fn to_scriptlets(self) -> Result<Vec<super::scriptlet::Scriptlet>, Box<dyn std::error::Error>> {
        match self {
            Self::File(path) => {
                let raw_scriptlets = std::fs::read_to_string(path)?;
                let scriptlets: Scriptlets = toml::from_str(&raw_scriptlets)?;
                Ok(scriptlets.scriptlets)
            },
            Self::Inline(scriptlet) => Ok(vec![scriptlet]),
        }
    }
}
