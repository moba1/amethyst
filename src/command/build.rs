use crate::config::{self, scriptlet, image::attribute};
use crate::result;
use std::convert;
use std::path;
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct Image {
    scriptlets: Vec<scriptlet::Scriptlet>,
    attribute: attribute::Attribute,
}

pub fn build<P>(config_directory: P) -> result::Result<()>
where
    P: convert::AsRef<path::Path>,
{
    let config = match config::load(&config_directory) {
        Ok(config) => config,
        Err(err) => return Err(err),
    };
    let mut images = vec![];
    for image in dbg!(config).images() {
        let scriptlets = match image.clone().slurp_scriptlets() {
            Ok(scriptlets) => scriptlets,
            Err(err) => return Err(err),
        };
        let attribute = image.attribute();
        let image = Image {
            scriptlets,
            attribute: attribute::Attribute {
                base_image: attribute.base_image,
                name: attribute.name,
            }
        };
        images.push(image);
    }
    Ok(())
}
