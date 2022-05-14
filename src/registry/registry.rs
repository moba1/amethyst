use crate::result;
use std::path;

pub trait Registry {
    fn download_base_image(&self, image_name: &str, tag: &str) -> result::Result<path::PathBuf>;
    fn image_storage() -> String;
}
