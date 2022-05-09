use crate::result;
use serde::Deserialize;
use std::convert;
use std::error;
use std::fmt;
use std::path;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
#[cfg_attr(test, derive(PartialEq))]
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
    mod script_load_error_function {
        use super::super::scriptlet_load_error;
        use std::error;
        use std::fmt;

        #[test]
        fn create_script_load_error() {
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
                format!("{}", scriptlet_load_error(PATH, Box::new(ORIGINAL_ERROR)))
            );
        }
    }

    mod to_scriptlets_method {
        mod file_module {
            use std::io::Write;

            use super::super::super::super::scriptlet;
            use super::super::super::Module;

            #[test]
            fn to_scriptlets() {
                let mut module_file =
                    tempfile::NamedTempFile::new().expect("temporary file created");
                let original_scriptlets = vec![
                    scriptlet::Scriptlet::Add {
                        source: "source1.yaml".to_string(),
                        destination: "destination1.yaml".to_string(),
                    },
                    scriptlet::Scriptlet::Add {
                        source: "source2.yaml".to_string(),
                        destination: "destination2.yaml".to_string(),
                    },
                ];
                let content = serde_yaml::to_string(&original_scriptlets)
                    .expect("cannot initialize module file");
                write!(&mut module_file, "{}", content).expect("initialize module file");

                let module = Module::File(module_file.path().to_string_lossy().to_string());
                let parsed_scriptlets = module.to_scriptlets();

                assert!(parsed_scriptlets.is_ok());
            }

            #[test]
            fn unloadable() {
                let module_file = tempfile::NamedTempFile::new().expect("temporary module file");
                let reserved_file_path = module_file
                    .path()
                    .join("abcd")
                    .to_string_lossy()
                    .to_string();

                assert!(Module::File(reserved_file_path).to_scriptlets().is_err());
            }

            #[test]
            fn cannot_deserialize_scriptlets_from_file() {
                let mut non_module_file =
                    tempfile::NamedTempFile::new().expect("temporary module file");

                write!(non_module_file, "non scriptlets string").unwrap();

                let non_module_file_path = non_module_file.path().to_string_lossy().to_string();
                assert!(Module::File(non_module_file_path).to_scriptlets().is_err());
            }
        }

        mod inline_module {
            use super::super::super::super::scriptlet;
            use super::super::super::Module;

            #[test]
            fn to_scriptlets() {
                let original_scriptlet = scriptlet::Scriptlet::Add {
                    source: "source.yaml".to_string(),
                    destination: "destination.yaml".to_string(),
                };
                let module = Module::Inline(original_scriptlet.clone());
                let parsed_scriptlets = module.to_scriptlets();

                assert!(parsed_scriptlets.is_ok());
                assert_eq!(parsed_scriptlets.unwrap(), vec![original_scriptlet]);
            }
        }
    }

    mod deserializable {
        use super::super::super::scriptlet;
        use super::super::Module;

        #[test]
        fn inline_module() {
            let source_path = "source_file";
            let destination_path = "destination_file";
            let original_string = format!(
                r#"
                type: add
                source: {}
                destination: {}
                "#,
                source_path, destination_path,
            );
            let deserialized_module = serde_yaml::from_str::<Module>(&original_string);

            assert!(deserialized_module.is_ok());
            assert_eq!(
                deserialized_module.unwrap(),
                Module::Inline(scriptlet::Scriptlet::Add {
                    source: source_path.to_string(),
                    destination: destination_path.to_string(),
                })
            );
        }

        #[test]
        fn file_module() {
            let original_string = "test";
            let deserialized_module = serde_yaml::from_str::<Module>(original_string);

            assert!(deserialized_module.is_ok());
            assert_eq!(
                deserialized_module.unwrap(),
                Module::File(original_string.to_string())
            );
        }
    }
}
