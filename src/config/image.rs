mod tag;
mod typ;

use crate::result;
use serde::{de, Deserialize, Deserializer, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Image<Script> {
    pub scripts: Vec<Script>,
    #[serde(default)]
    pub base_image: typ::ImageType,
    #[serde(deserialize_with = "deserialize_image_name")]
    pub name: String,
    #[serde(default = "default_tag")]
    pub tag: String,
}

impl Image<super::module::Module> {
    pub fn slurp_scriptlets(&self) -> result::Result<Vec<super::scriptlet::Scriptlet>> {
        let scriptlets = (&self.scripts)
            .iter()
            .map(|module| module.to_scriptlets())
            .collect::<Result<Vec<_>, _>>()?
            .concat();
        Ok(scriptlets)
    }
}

fn deserialize_image_name<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let image_name = String::deserialize(deserializer)?;
    if image_name == typ::SCRATCH_IMAGE_NAME {
        return Err(de::Error::invalid_value(
            de::Unexpected::Str(typ::SCRATCH_IMAGE_NAME),
            &"other than scratch",
        ));
    }
    Ok(image_name)
}

fn default_tag() -> String {
    tag::LATEST_TAG.to_string()
}

#[cfg(test)]
mod tests {
    use super::{tag, typ};

    mod deserializability {
        mod undeserializable {
            use super::super::super::Image;

            #[test]
            fn no_name_attribute() {
                let scripts: Vec<i32> = vec![];
                let original_string = format!(
                    r#"
                    scripts: {}
                    "#,
                    format_args!(
                        "[{}]",
                        scripts
                            .iter()
                            .map(|v| format!("{}", v))
                            .collect::<Vec<_>>()
                            .join(",")
                    ),
                );
                assert!(serde_yaml::from_str::<Image<i32>>(&original_string).is_err());
            }

            #[test]
            fn no_scripts_attribute() {
                let name = "amethyst";
                let original_string = format!(
                    r#"
                    name: {}
                    "#,
                    name
                );
                assert!(serde_yaml::from_str::<Image<i32>>(&original_string).is_err());
            }
        }

        mod deserializable {
            use super::super::super::{tag, typ, Image};

            #[test]
            fn minimum_set() {
                let scripts: Vec<i32> = vec![1];
                let image_name = "amethyst";
                let original_string = format!(
                    r#"
                    scripts: {}
                    name: {}
                    "#,
                    format_args!(
                        "[{}]",
                        scripts
                            .iter()
                            .map(|v| format!("{}", v))
                            .collect::<Vec<_>>()
                            .join(",")
                    ),
                    image_name,
                );
                let image = Image::<_> {
                    base_image: typ::ImageType::Scratch,
                    name: image_name.to_string(),
                    scripts,
                    tag: tag::LATEST_TAG.to_string(),
                };
                let deserialized_image = serde_yaml::from_str::<Image<i32>>(&original_string);

                assert!(deserialized_image.is_ok());
                assert_eq!(image, deserialized_image.unwrap());
            }

            #[test]
            fn full_set() {
                let scripts: Vec<i32> = vec![1, 2];
                let image_name = "amethyst";
                let image_tag = "tag";
                let base_image_name = "base_image";
                let original_string = format!(
                    r#"
                    scripts: {}
                    name: {}
                    tag: {}
                    base_image:
                      name: {}
                    "#,
                    format_args!(
                        "[{}]",
                        scripts
                            .iter()
                            .map(|v| format!("{}", v))
                            .collect::<Vec<_>>()
                            .join(",")
                    ),
                    image_name,
                    image_tag,
                    base_image_name,
                );
                let image = Image::<_> {
                    base_image: typ::ImageType::BaseImage {
                        name: base_image_name.to_string(),
                        tag: tag::LATEST_TAG.to_string(),
                    },
                    name: image_name.to_string(),
                    scripts,
                    tag: image_tag.to_string(),
                };
                let deserialized_image = serde_yaml::from_str::<Image<i32>>(&original_string);

                assert!(deserialized_image.is_ok());
                assert_eq!(image, deserialized_image.unwrap());
            }
        }
    }

    #[test]
    fn serializable_minimum_attributes() {
        let image_name = "amethyst";
        let image_tag = tag::LATEST_TAG;
        let original_image = super::Image::<_> {
            base_image: typ::ImageType::Scratch,
            scripts: vec![1],
            name: image_name.to_string(),
            tag: image_tag.to_string(),
        };
        let expected_string = format!(
            r#"---
scripts:
  - 1
base_image:
  name: {}
name: {}
tag: {}
"#,
            typ::SCRATCH_IMAGE_NAME,
            image_name,
            image_tag,
        );
        let serialized_string = serde_yaml::to_string(&original_image);

        assert!(serialized_string.is_ok());
        assert_eq!(serialized_string.unwrap(), expected_string);
    }
}
