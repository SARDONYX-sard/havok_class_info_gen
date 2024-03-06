use serde::{Deserialize, Serialize};

/// A structure for parsing XML that takes a pointer to a C++ structure.
///
/// In XML, it is just an array of space-delimited class name strings.
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkparam")]
pub struct HkArrayRef<T> {
    /// Length of the pointer array stored in its own structure.
    #[serde(rename = "@numelements")]
    pub numelements: usize,
    /// A vector in structure for parsing XML that takes a pointer to a C++ structure.
    ///
    /// In XML, it is just an array of space-delimited class name strings.
    #[serde(rename = "$text")]
    pub value: Vec<T>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize() {
        let data = HkArrayRef {
            numelements: 2,
            value: vec!["#0063", "#0064"],
        };
        let serialized = quick_xml::se::to_string(&data).unwrap();

        let expected_xml = "\
            <hkparam numelements=\"2\">\
                #0063 #0064\
            </hkparam>\
        ";

        assert_eq!(serialized, expected_xml);
    }

    #[test]
    fn should_deserialize() {
        let xml = r###"
            <hkparam name="variantVariableValues" numelements="2">
                    #0063 #0064
            </hkparam>
        "###;
        let deserialized: HkArrayRef<&str> = quick_xml::de::from_str(xml).unwrap();

        let expected = HkArrayRef {
            numelements: 2,
            value: vec!["#0063", "#0064"],
        };

        assert_eq!(deserialized, expected);
    }
}
