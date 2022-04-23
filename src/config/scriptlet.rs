use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ScriptLet {
    #[serde(rename(deserialize = "add"))]
    Add { source: String, destination: String },
}
