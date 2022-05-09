use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(tag = "type")]
pub enum Scriptlet {
    #[serde(rename = "add")]
    Add { source: String, destination: String },
}

#[cfg(test)]
mod tests {
    mod add {
        use super::super::Scriptlet;

        #[test]
        fn serializable() {
            let source = "source";
            let destination = "destination";
            let scriptlet = Scriptlet::Add {
                source: source.to_string(),
                destination: destination.to_string(),
            };
            let expected_string = format!(
                r#"---
type: add
source: {}
destination: {}
"#,
                source, destination
            );
            let serialized_string = serde_yaml::to_string(&scriptlet);

            assert!(serialized_string.is_ok());
            assert_eq!(expected_string, serialized_string.unwrap());
        }

        mod deserializability {
            use super::super::super::Scriptlet;

            #[test]
            fn deserializable() {
                let source = "source";
                let destination = "destination";
                let original_string = format!(
                    "{{ type: add, source: {:?}, destination: {:?} }}",
                    source, destination
                );
                let deserialized_scriptlet =
                    serde_yaml::from_str::<Scriptlet>(original_string.as_str());

                assert!(deserialized_scriptlet.is_ok());
                assert_eq!(
                    Scriptlet::Add {
                        source: source.to_string(),
                        destination: destination.to_string(),
                    },
                    deserialized_scriptlet.unwrap(),
                );
            }

            #[test]
            fn undeserializable() {
                let original_string = "{{ type: add, source: source }}";
                let deserialized_scriptlet = serde_yaml::from_str::<Scriptlet>(original_string);

                assert!(deserialized_scriptlet.is_err());
            }
        }
    }
}
