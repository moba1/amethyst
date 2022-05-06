use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type")]
pub enum Scriptlet {
    #[serde(rename = "add")]
    Add { source: String, destination: String },
}
