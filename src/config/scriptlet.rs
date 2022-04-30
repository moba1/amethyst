use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum Scriptlet {
    #[serde(rename(deserialize = "add"))]
    Add { source: String, destination: String },
}
