use std::io::Read;
use serde::Deserialize;

pub mod scriptlets;

#[derive(Debug, Deserialize)]
pub struct Config {
    scriptlets: Vec<scriptlets::ScriptLet>,
}

pub fn load<Path>(config_directory: Path) -> Result<Config, Box<dyn std::error::Error>>
    where Path: std::convert::AsRef<std::path::Path>
{
    let entrypoint = config_directory.as_ref().join("amethyst.toml");
    let mut file = std::fs::File::open(&entrypoint)?;
    let mut raw_config = String::new();
    file.read_to_string(&mut raw_config)?;

    Ok(toml::from_str(&raw_config)?)
}
