use serde::{de, ser::SerializeStruct, Deserialize, Serialize};
use std::default;

use super::tag;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum ImageType {
    Scratch,
    BaseImage { name: String, tag: String },
}

const NAME_ATTRIBUTE_NAME: &str = "name";
const TAG_ATTRIBUTE_NAME: &str = "tag";
pub const SCRATCH_IMAGE_NAME: &str = "scratch";

impl<'de> Deserialize<'de> for ImageType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug)]
        enum Field {
            Name,
            Tag,
        }
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> de::Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("`name` required")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        match v {
                            NAME_ATTRIBUTE_NAME => Ok(Field::Name),
                            TAG_ATTRIBUTE_NAME => Ok(Field::Tag),
                            _ => Err(de::Error::unknown_field(
                                v,
                                &[NAME_ATTRIBUTE_NAME, TAG_ATTRIBUTE_NAME],
                            )),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct ImageTypeVisitor;
        impl<'de> de::Visitor<'de> for ImageTypeVisitor {
            type Value = ImageType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ImageType")
            }

            fn visit_map<A>(self, mut map: A) -> Result<ImageType, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                let mut name = None;
                let mut tag = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Name => {
                            if name.is_some() {
                                return Err(de::Error::duplicate_field(NAME_ATTRIBUTE_NAME));
                            }
                            name = map.next_value::<Option<String>>()?;
                        }
                        Field::Tag => {
                            if tag.is_some() {
                                return Err(de::Error::duplicate_field(TAG_ATTRIBUTE_NAME));
                            }
                            tag = map.next_value::<Option<String>>()?;
                        }
                    }
                }
                let name = name.ok_or_else(|| de::Error::missing_field(NAME_ATTRIBUTE_NAME))?;
                let name = match name.as_str() {
                    SCRATCH_IMAGE_NAME => return Ok(ImageType::Scratch),
                    image => image.to_string(),
                };
                let tag = tag.unwrap_or_else(|| tag::LATEST_TAG.to_string());
                Ok(ImageType::BaseImage { name, tag })
            }
        }

        deserializer.deserialize_struct(
            "ImageType",
            &[NAME_ATTRIBUTE_NAME, TAG_ATTRIBUTE_NAME],
            ImageTypeVisitor,
        )
    }
}

impl Serialize for ImageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ImageType::Scratch => {
                let mut state = serializer.serialize_struct("ImageType", 1)?;
                state.serialize_field(NAME_ATTRIBUTE_NAME, SCRATCH_IMAGE_NAME)?;
                state.end()
            }
            ImageType::BaseImage { name, tag } => {
                let mut state = serializer.serialize_struct("ImageType", 2)?;
                state.serialize_field(NAME_ATTRIBUTE_NAME, name)?;
                state.serialize_field(TAG_ATTRIBUTE_NAME, tag)?;
                state.end()
            }
        }
    }
}

impl default::Default for ImageType {
    fn default() -> Self {
        Self::Scratch
    }
}

#[cfg(test)]
mod tests {
    use std::default;

    #[test]
    fn default_value_is_scratch() {
        let default_value: super::ImageType = default::Default::default();

        assert_eq!(default_value, super::ImageType::Scratch);
    }

    mod serializability {
        mod serializable {
            use super::super::super::{ImageType, SCRATCH_IMAGE_NAME};

            #[test]
            fn scrach_image() {
                let image_type = ImageType::Scratch;
                let expected_string = format!(
                    r#"---
name: {}
"#,
                    SCRATCH_IMAGE_NAME
                );
                let serialized_string = serde_yaml::to_string(&image_type);

                assert!(serialized_string.is_ok());
                assert_eq!(expected_string, serialized_string.unwrap());
            }

            #[test]
            fn non_scrach_image() {
                let name = "base_image_name";
                let tag = "tag";
                let image_type = ImageType::BaseImage {
                    name: name.to_string(),
                    tag: tag.to_string(),
                };
                let expected_string = format!(
                    r#"---
name: {}
tag: {}
"#,
                    name, tag
                );
                let serialized_string = serde_yaml::to_string(&image_type);

                assert!(serialized_string.is_ok());
                assert_eq!(expected_string, serialized_string.unwrap(),);
            }
        }
    }

    mod deserializability {
        mod deserializable {
            use super::super::super::{tag, ImageType, SCRATCH_IMAGE_NAME};

            #[test]
            fn scrach_image() {
                let original_string = format!(
                    r#"---
                    name: {}
                    "#,
                    SCRATCH_IMAGE_NAME
                );
                let image_type = ImageType::Scratch;
                let deserialized_image_type = serde_yaml::from_str(original_string.as_str());

                assert!(deserialized_image_type.is_ok());
                assert_eq!(image_type, deserialized_image_type.unwrap());
            }

            #[test]
            fn non_scrach_image_with_custom_tag() {
                let name = "base_image_name";
                let tag = "tag";
                let image_type = ImageType::BaseImage {
                    name: name.to_string(),
                    tag: tag.to_string(),
                };
                let original_string = format!(
                    r#"---
                    name: {}
                    tag: {}
                    "#,
                    name, tag
                );
                let deserialized_image_type = serde_yaml::from_str(original_string.as_str());

                assert!(deserialized_image_type.is_ok());
                assert_eq!(image_type, deserialized_image_type.unwrap());
            }

            #[test]
            fn latest_non_scrach_image() {
                let name = "base_image_name";
                let original_string = format!(
                    r#"---
                    name: {}
                    "#,
                    name,
                );
                let image_type = ImageType::BaseImage {
                    name: name.to_string(),
                    tag: tag::LATEST_TAG.to_string(),
                };
                let deserialized_image_type = serde_yaml::from_str(original_string.as_str());

                assert!(deserialized_image_type.is_ok());
                assert_eq!(image_type, deserialized_image_type.unwrap(),);
            }
        }
    }
}
