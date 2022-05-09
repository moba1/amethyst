pub mod image;
pub mod module;
pub mod scriptlet;

use crate::result;
use serde::{Deserialize, Deserializer, Serialize};
use std::convert;
use std::error;
use std::fmt;
use std::fs;
use std::io::Read;
use std::path;

const CONFIG_FILE_NAME: &str = "amethyst.yaml";

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Config<Script> {
    #[serde(rename = "image")]
    #[serde(deserialize_with = "deserialize_images")]
    #[serde(bound(deserialize = "Script: Deserialize<'de>"))]
    pub images: Vec<image::Image<Script>>,
}

fn deserialize_images<'de, Script, D>(
    deserializer: D,
) -> Result<Vec<image::Image<Script>>, D::Error>
where
    D: Deserializer<'de>,
    Script: Deserialize<'de>,
{
    Deserialize::deserialize(deserializer)
}

#[derive(Debug)]
struct LoadError {
    path: String,
    original_error: result::BoxedError,
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "cannot load configuration file: {} [{}]",
            self.path, self.original_error
        )
    }
}

impl error::Error for LoadError {}

fn load_error<P>(path: P, original_error: result::BoxedError) -> result::BoxedError
where
    P: convert::AsRef<path::Path>,
{
    Box::new(LoadError {
        path: format!("{:?}", path.as_ref()),
        original_error,
    })
}

fn load<Path>(config_directory: Path) -> result::Result<Config<module::Module>>
where
    Path: convert::AsRef<std::path::Path>,
{
    let entrypoint = config_directory.as_ref().join(CONFIG_FILE_NAME);
    let mut file = match fs::File::open(&entrypoint) {
        Ok(file) => file,
        Err(err) => return Err(load_error(entrypoint, Box::new(err))),
    };
    let mut raw_config = String::new();
    if let Err(err) = file.read_to_string(&mut raw_config) {
        return Err(load_error(entrypoint, Box::new(err)));
    }

    match serde_yaml::from_str(&raw_config) {
        Ok(config) => Ok(config),
        Err(err) => Err(load_error(entrypoint, Box::new(err))),
    }
}

pub fn build<P>(config_directory: P) -> result::Result<Config<scriptlet::Scriptlet>>
where
    P: convert::AsRef<path::Path>,
{
    let config = match load(&config_directory) {
        Ok(config) => config,
        Err(err) => return Err(err),
    };
    let mut images = vec![];
    for image in config.images {
        let scriptlets = match image.slurp_scriptlets() {
            Ok(scriptlets) => scriptlets,
            Err(err) => return Err(err),
        };
        let image = image::Image {
            scripts: scriptlets,
            base_image: image.base_image,
            name: image.name,
            tag: image.tag,
        };
        images.push(image);
    }
    Ok(Config { images })
}

#[cfg(test)]
mod tests {
    mod load_error_function {
        use super::super::load_error;
        use std::error;
        use std::fmt;

        #[test]
        fn create_load_error() {
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
                    "cannot load configuration file: {:?} [{}]",
                    PATH, ORIGINAL_ERROR
                ),
                format!("{}", load_error(PATH, Box::new(ORIGINAL_ERROR))),
            );
        }
    }

    mod load_function {
        use std::fs;
        use std::io::Write;

        use super::super::image::{self, typ};
        use super::super::module;
        use super::super::{load, Config, CONFIG_FILE_NAME};
        use crate::config::scriptlet;
        use crate::result;

        #[test]
        fn cannot_load_config_since_config_file_not_exist() {
            let reserved_file = tempfile::NamedTempFile::new().expect("reserved file");

            assert!(load(reserved_file.path()).is_err());
        }

        #[test]
        fn cannot_load_config_from_non_config_file() {
            let config_directory = tempfile::tempdir().expect("config directory");
            let config_file_path = config_directory.path().join(CONFIG_FILE_NAME);
            let eval = || -> result::Result<Config<module::Module>> {
                let mut config_file = fs::File::create(config_file_path.clone())?;

                write!(config_file, "non config file")?;

                load(config_directory)
            };

            assert!(eval().is_err());
        }

        #[test]
        fn load_config_file() {
            let config_directory = tempfile::tempdir().expect("config directory");
            let config_file_path = config_directory.path().join(CONFIG_FILE_NAME);
            let base_image_name = typ::SCRATCH_IMAGE_NAME;
            let image1_name = "image1";
            let image1_tag = "0.1";
            let image2_name = "image2";
            let image2_tag = "0.2";
            let image2_scriptlet_source = "source";
            let image2_scriptlet_destination = "destination";
            let eval = || -> result::Result<Config<module::Module>> {
                let mut config_file =
                    fs::File::create(config_file_path.clone()).expect("config file");
                let config = format!(
                    r#"
                    image:
                      - name: {}
                        base_image:
                          name: {}
                        scripts: []
                        tag: {:?}
                      - name: {}
                        base_image:
                          name: {}
                        scripts:
                          - type: add
                            source: {}
                            destination: {}
                        tag: {:?}
                    "#,
                    image1_name,
                    base_image_name,
                    image1_tag,
                    image2_name,
                    base_image_name,
                    image2_scriptlet_source,
                    image2_scriptlet_destination,
                    image2_tag,
                );
                write!(config_file, "{}", config)?;

                load(config_directory)
            };

            let original_config: Config<module::Module> = Config {
                images: vec![
                    image::Image {
                        name: image1_name.to_string(),
                        base_image: typ::ImageType::Scratch,
                        tag: image1_tag.to_string(),
                        scripts: vec![],
                    },
                    image::Image {
                        name: image2_name.to_string(),
                        base_image: typ::ImageType::Scratch,
                        tag: image2_tag.to_string(),
                        scripts: vec![module::Module::Inline(scriptlet::Scriptlet::Add {
                            source: image2_scriptlet_source.to_string(),
                            destination: image2_scriptlet_destination.to_string(),
                        })],
                    },
                ],
            };
            let loaded_config = eval();

            assert!(loaded_config.is_ok());
            assert_eq!(loaded_config.unwrap(), original_config);
        }
    }

    mod build_function {
        use std::fs;
        use std::io::Write;

        use super::super::image::{self, typ};
        use super::super::{build, Config, CONFIG_FILE_NAME};
        use crate::config::image::tag;
        use crate::config::scriptlet;
        use crate::result;

        #[test]
        fn cannot_build_config_since_config_file_not_exist() {
            let reserved_file = tempfile::NamedTempFile::new().expect("reserved file");

            assert!(build(reserved_file.path()).is_err());
        }

        #[test]
        fn cannot_build_config_from_non_config_file() {
            let config_directory = tempfile::tempdir().expect("config directory");
            let config_file_path = config_directory.path().join(CONFIG_FILE_NAME);
            let eval = || -> result::Result<Config<scriptlet::Scriptlet>> {
                let mut config_file = fs::File::create(config_file_path.clone())?;

                write!(config_file, "non config file")?;

                build(config_directory)
            };

            assert!(eval().is_err());
        }

        #[test]
        fn build_config_from_config_file() {
            let config_directory = tempfile::tempdir().expect("config directory");
            let add_scriptlet_source = "source";
            let add_scriptlet_destination = "destination";
            let base_image_name = "base_image";
            let image_name = "image";
            let image_tag = "0.1";
            let module_file_path = config_directory.path().join("scriptlet.yaml");
            let config_file_path = config_directory.path().join("amethyst.yaml");

            let eval = || -> result::Result<Config<scriptlet::Scriptlet>> {
                let mut module_file = fs::File::create(&module_file_path)?;
                let module_file_content = format!(
                    r#"
                    - type: add
                      source: {}
                      destination: {}
                    "#,
                    add_scriptlet_source, add_scriptlet_destination
                );
                write!(module_file, "{}", module_file_content)?;

                let config_file_content = format!(
                    r#"
                image:
                  - base_image:
                      name: {}
                    name: {}
                    scripts:
                      - {}
                    tag: {:?}
                "#,
                    base_image_name,
                    image_name,
                    module_file_path.to_string_lossy(),
                    image_tag
                );
                let mut config_file = fs::File::create(config_file_path)?;

                write!(config_file, "{}", config_file_content)?;

                build(config_directory.path())
            };
            let built_config = eval();
            let original_config = Config {
                images: vec![image::Image {
                    name: image_name.to_string(),
                    tag: image_tag.to_string(),
                    base_image: typ::ImageType::BaseImage {
                        name: base_image_name.to_string(),
                        tag: tag::LATEST_TAG.to_string(),
                    },
                    scripts: vec![scriptlet::Scriptlet::Add {
                        source: add_scriptlet_source.to_string(),
                        destination: add_scriptlet_destination.to_string(),
                    }],
                }],
            };

            assert!(built_config.is_ok());
            assert_eq!(built_config.unwrap(), original_config);
        }
    }
}
