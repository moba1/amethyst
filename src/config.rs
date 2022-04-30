use serde::Deserialize;
use std::io::Read;

pub mod image;
pub mod module;
pub mod scriptlet;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "image")]
    images: Vec<image::Image>,
}

pub fn load<Path>(config_directory: Path) -> Result<Config, Box<dyn std::error::Error>>
where
    Path: std::convert::AsRef<std::path::Path>,
{
    let entrypoint = config_directory.as_ref().join("amethyst.toml");
    let mut file = std::fs::File::open(&entrypoint)?;
    let mut raw_config = String::new();
    file.read_to_string(&mut raw_config)?;

    Ok(toml::from_str(&raw_config)?)
}

impl Config {
    pub fn images(self) -> Vec<image::Image> {
        self.images
    }
}
