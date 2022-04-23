use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Image {
    modules: Vec<super::module::Module>,
    #[serde(default = "default_base_image")]
    base_image: String,
    name: String,
}

fn default_base_image() -> String {
    "scratch".to_string()
}
