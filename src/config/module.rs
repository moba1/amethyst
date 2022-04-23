use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Module {
    File(String),
    Inline(super::scriptlet::ScriptLet),
}
