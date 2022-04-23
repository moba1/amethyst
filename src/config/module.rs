use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum Module {
    File(String),
    Inline(super::scriptlet::ScriptLet),
}
