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
    #[test]
    fn test_add_scriptlet_serialize() {
        let source = "source";
        let destination = "destination";
        let scriptlet = super::Scriptlet::Add {
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
        assert_eq!(expected_string, serialized_string.unwrap(),);
    }

    #[test]
    fn test_add_scriptlet_deserialize() {
        let source = "source";
        let destination = "destination";
        let original_string = format!(
            "{{ type: add, source: {:?}, destination: {:?} }}",
            source, destination
        );
        assert_eq!(
            super::Scriptlet::Add {
                source: source.to_string(),
                destination: destination.to_string(),
            },
            serde_yaml::from_str(original_string.as_str()).unwrap_or_else(|_| panic!(
                "add type (source: {}, destination: {})",
                source, destination
            ))
        );
    }
}
