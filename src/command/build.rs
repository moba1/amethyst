use crate::config;
use crate::result;
use std::convert;
use std::path;

pub fn build<P>(config_directory: P) -> result::Result<()>
where
    P: convert::AsRef<path::Path>,
{
    let config = match config::load(&config_directory) {
        Ok(config) => config,
        Err(err) => return Err(err),
    };
    for image in config.images() {
        match image.slurp_scriptlets() {
            Ok(scriptlets) => println!("{:?}", scriptlets),
            Err(err) => return Err(err),
        }
    }
    Ok(())
}
