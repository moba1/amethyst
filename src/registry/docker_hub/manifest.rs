use crate::storage;
use serde::Deserialize;
use std::path;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "mediaType")]
    pub media_type: String,
    pub size: usize,
    pub digest: String,
}

#[derive(Debug, Deserialize)]
pub struct Layer {
    #[serde(rename = "mediaType")]
    pub media_type: String,
    pub size: usize,
    pub digest: String,
}

#[derive(Debug, Deserialize)]
pub struct Manifest {
    #[serde(rename = "schemaVersion")]
    pub schema_version: u32,
    #[serde(rename = "mediaType")]
    pub media_type: String,
    pub config: Config,
    pub layers: Vec<Layer>,
}

pub fn storage() -> path::PathBuf {
    storage::storage().join("docker")
}

pub const MANIFEST_FILENAME: &str = "manifest.json";
