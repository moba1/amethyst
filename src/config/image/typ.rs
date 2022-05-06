use serde::{Deserialize, Serialize, de, ser::SerializeStruct};
use std::default;

#[derive(Debug, Clone)]
pub enum ImageType {
    Scratch,
    BaseImage {
        name: String,
        tag: String,
    }
}

const NAME_ATTRIBUTE_NAME: &'static str = "name";
const TAG_ATTRIBUTE_NAME: &'static str = "tag";

impl<'de> Deserialize<'de> for ImageType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
            D: serde::Deserializer<'de> {
        #[derive(Debug)]
        enum Field { Name, Tag }
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                    D: serde::Deserializer<'de> {
                struct FieldVisitor;

                impl<'de> de::Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("`name` required")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                            E: de::Error, {
                        match v {
                            NAME_ATTRIBUTE_NAME => Ok(Field::Name),
                            TAG_ATTRIBUTE_NAME => Ok(Field::Tag),
                            _ => Err(de::Error::unknown_field(v, &["name", "tag"]))
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
                    A: de::MapAccess<'de>, {
                let mut name = None;
                let mut tag = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Name => {
                            if name.is_some() {
                                return Err(de::Error::duplicate_field(NAME_ATTRIBUTE_NAME))
                            }
                            name = map.next_value::<Option<String>>()?;
                        }
                        Field::Tag => {
                            if tag.is_some() {
                                return Err(de::Error::duplicate_field(TAG_ATTRIBUTE_NAME))
                            }
                            tag = map.next_value::<Option<String>>()?;
                        }
                    }
                }
                let name = name.ok_or_else(|| de::Error::missing_field(NAME_ATTRIBUTE_NAME))?;
                let name = match name.as_str() {
                    "scratch" => return Ok(ImageType::Scratch),
                    image => image.to_string(),
                };
                let tag = tag.unwrap_or_else(|| "latest".to_string());
                Ok(ImageType::BaseImage { name, tag })
            }
        }

        deserializer.deserialize_struct("ImageType", &[NAME_ATTRIBUTE_NAME, TAG_ATTRIBUTE_NAME], ImageTypeVisitor)
    }
}

impl Serialize for ImageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
            S: serde::Serializer {
        match self {
            ImageType::Scratch => {
                let mut state = serializer.serialize_struct("ImageType", 1)?;
                state.serialize_field(NAME_ATTRIBUTE_NAME, "scratch")?;
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
