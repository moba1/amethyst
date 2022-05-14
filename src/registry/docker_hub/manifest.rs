use serde::Deserialize;

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
