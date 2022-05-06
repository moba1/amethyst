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

#[derive(Debug, Deserialize, Serialize, Clone)]
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
    let entrypoint = config_directory.as_ref().join("amethyst.yaml");
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
        };
        images.push(image);
    }
    Ok(Config { images })
}
