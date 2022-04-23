use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum ScriptLet {
    #[serde(rename(deserialize = "add"))]
    Add { source: String, destination: String },
}
