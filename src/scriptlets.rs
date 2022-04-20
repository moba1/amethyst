use std::io::Read;

#[derive(Debug)]
pub enum ScriptLet {
    Add {
        source: String,
        destination: String,
    }
}

pub mod parser;

pub fn load<Path>(config_directory: Path) -> Result<(), std::io::Error>
    where Path: std::convert::AsRef<std::path::Path>
{
    let entrypoint = config_directory.as_ref().join("amethyst.yaml");
    let mut file = std::fs::File::open(&entrypoint)?;
    let mut raw_config = String::new();
    file.read_to_string(&mut raw_config)?;
    println!("load contents = {:?}", raw_config);
    Ok(())
}
