pub mod image;
pub mod module;
pub mod scriptlet;

use crate::result;
use serde::Deserialize;
use std::convert;
use std::error;
use std::fmt;
use std::fs;
use std::io::Read;
use std::path;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    #[serde(rename = "image")]
    images: Vec<image::Image>,
}

impl Config {
    pub fn images(self) -> Vec<image::Image> {
        self.images
    }
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

pub fn load<Path>(config_directory: Path) -> result::Result<Config>
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
        Ok(config) => return Ok(config),
        Err(err) => Err(load_error(entrypoint, Box::new(err))),
    }
}
