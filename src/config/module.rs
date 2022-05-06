use crate::result;
use serde::Deserialize;
use std::convert;
use std::error;
use std::fmt;
use std::path;

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum Module {
    File(String),
    Inline(super::scriptlet::Scriptlet),
}

impl Module {
    pub fn to_scriptlets(&self) -> result::Result<Vec<crate::config::scriptlet::Scriptlet>> {
        match self {
            Self::File(path) => {
                let raw_scriptlets = match std::fs::read_to_string(&path) {
                    Ok(raw_scriptlets) => raw_scriptlets,
                    Err(err) => return Err(scriptlet_load_error(path, Box::new(err))),
                };
                match serde_yaml::from_str::<Vec<super::scriptlet::Scriptlet>>(&raw_scriptlets) {
                    Ok(scriptlets) => Ok(scriptlets),
                    Err(err) => Err(scriptlet_load_error(path, Box::new(err))),
                }
            }
            Self::Inline(scriptlet) => Ok(vec![scriptlet.clone()]),
        }
    }
}

fn scriptlet_load_error<P>(path: P, original_error: result::BoxedError) -> result::BoxedError
where
    P: convert::AsRef<path::Path>,
{
    Box::new(ScriptletLoadError {
        path: format!("{:?}", path.as_ref()),
        original_error,
    })
}

#[derive(Debug)]
struct ScriptletLoadError {
    original_error: result::BoxedError,
    path: String,
}

impl fmt::Display for ScriptletLoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "cannot load scriptlet file: {} [{}]",
            self.path, self.original_error
        )
    }
}

impl error::Error for ScriptletLoadError {}
