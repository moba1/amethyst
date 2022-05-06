use crate::config;
use crate::result;
use std::convert;
use std::path;

pub fn build<P>(config_directory: P) -> result::Result<()>
where
    P: convert::AsRef<path::Path>,
{
    let config = config::build(config_directory)?;
    match serde_yaml::to_string(&config) {
        Ok(config) => println!("{}", config),
        Err(err) => eprintln!("error occured: {:?}", err),
    }
    Ok(())
}
