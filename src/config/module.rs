use crate::result;
use serde::Deserialize;
use std::convert;
use std::error;
use std::fmt;
use std::path;

#[derive(Debug, Deserialize)]
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

#[cfg(test)]
mod tests {
    use std::error;
    use std::fmt;
    use std::io::Write;

    use crate::config::scriptlet;

    #[test]
    fn test_scriptlet_load_error() {
        const ORIGINAL_ERROR_MESSAGE: &str = "original error";
        #[derive(Debug)]
        struct OriginalError;
        impl fmt::Display for OriginalError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", ORIGINAL_ERROR_MESSAGE)
            }
        }
        impl error::Error for OriginalError {}

        const PATH: &str = "./";
        const ORIGINAL_ERROR: OriginalError = OriginalError;

        assert_eq!(
            format!(
                "cannot load scriptlet file: {:?} [{}]",
                PATH, ORIGINAL_ERROR
            ),
            format!(
                "{}",
                super::scriptlet_load_error(PATH, Box::new(ORIGINAL_ERROR))
            )
        )
    }

    #[test]
    fn test_scriptlet_load_format() {
        const ORIGINAL_ERROR_MESSAGE: &str = "original error";
        #[derive(Debug)]
        struct OriginalError;
        impl fmt::Display for OriginalError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", ORIGINAL_ERROR_MESSAGE)
            }
        }
        impl error::Error for OriginalError {}

        const PATH: &str = "./";

        let error = super::ScriptletLoadError {
            original_error: Box::new(OriginalError),
            path: PATH.to_string(),
        };
        assert_eq!(
            format!("{}", error),
            format!(
                "cannot load scriptlet file: {} [{}]",
                PATH, ORIGINAL_ERROR_MESSAGE
            ),
        );
    }

    #[test]
    fn test_to_scriptlets_with_file_module() {
        let mut module_file = tempfile::NamedTempFile::new().expect("temporary file created");
        let original_scriptlets = vec![
            scriptlet::Scriptlet::Add {
                source: "./source1.yaml".to_string(),
                destination: "./destination1.yaml".to_string(),
            },
            scriptlet::Scriptlet::Add {
                source: "./source2.yaml".to_string(),
                destination: "./destination2.yaml".to_string(),
            },
        ];
        let content =
            serde_yaml::to_string(&original_scriptlets).expect("cannot initialize module file");
        write!(&mut module_file, "{}", content).expect("initialize module file");

        let module = super::Module::File(module_file.path().to_string_lossy().to_string());
        let parsed_scriptlets = module.to_scriptlets();
        assert!(parsed_scriptlets.is_ok());
        let parsed_scriptlets = parsed_scriptlets.unwrap();
        assert_eq!(parsed_scriptlets, original_scriptlets);

        assert!(super::Module::File("./abcd".to_string())
            .to_scriptlets()
            .is_err());
    }

    #[test]
    fn test_to_scriptlets_with_inline_module() {
        let original_scriptlet = scriptlet::Scriptlet::Add {
            source: "./source.yaml".to_string(),
            destination: "./destination.yaml".to_string(),
        };
        let module = super::Module::Inline(original_scriptlet.clone());
        let parsed_scriptlets = module.to_scriptlets();
        assert!(parsed_scriptlets.is_ok());
        assert_eq!(parsed_scriptlets.unwrap(), vec![original_scriptlet]);
    }
}
