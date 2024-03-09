use serde::{Deserialize, Serialize};

/// A type to hold another class of `hkparam` as an array within `hkparam` in XML.
///
/// e.g. `wordVariableValues` field of `hkbVariableValueSet` class
///
/// ```xml
/// <hkparam name="variantVariableValues" numelements="2">
///     <hkobject>
///         <hkparam name="class_field">#0063</hkparam>
///     </hkobject>
///     <hkobject>
///         <hkparam name="another_class_field">#0064</hkparam>
///     </hkobject>
/// </hkparam>
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkparam")]
pub struct HkArrayClass<T> {
    /// Length of the class
    #[serde(rename = "@numelements")]
    pub numelements: usize,
    /// An array that receives `hkparam` of a certain class.
    ///
    /// The XML for this class does not require `name` and `signature`, but only encloses `hkobject` tags without attributes.
    #[serde(rename = "hkobject")]
    pub classes: Vec<HkArrayClassParam<T>>,
}

impl<T> From<Vec<HkArrayClassParam<T>>> for HkArrayClass<T> {
    fn from(classes: Vec<HkArrayClassParam<T>>) -> Self {
        Self {
            numelements: classes.len(),
            classes,
        }
    }
}

/// One class of `HkArray`
///
/// ```xml
/// <hkobject>
///     <hkparam>#0063</hkparam>
/// </hkobject>
/// ````
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct HkArrayClassParam<T> {
    #[serde(rename = "hkparam")]
    pub hkparam: T,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize() {
        let data: HkArrayClass<_> = vec![
            HkArrayClassParam {
                hkparam: 1045220557,
            },
            HkArrayClassParam { hkparam: 0 },
        ]
        .into();
        let serialized = quick_xml::se::to_string(&data).unwrap();

        let expected_xml = "\
            <hkparam numelements=\"2\">\
                <hkobject>\
                    <hkparam>\
                        1045220557\
                    </hkparam>\
                </hkobject>\
                <hkobject>\
                    <hkparam>\
                        0\
                    </hkparam>\
                </hkobject>\
            </hkparam>\
        ";

        assert_eq!(serialized, expected_xml);
    }

    #[test]
    fn should_deserialize() {
        let xml = r###"
            <hkparam name="variantVariableValues" numelements="2">
                <hkobject>
                    <hkparam>#0063</hkparam>
                </hkobject>
                <hkobject>
                    <hkparam>#0064</hkparam>
                </hkobject>
            </hkparam>
        "###;
        let deserialized: HkArrayClass<String> = quick_xml::de::from_str(xml).unwrap();

        let expected = HkArrayClass {
            numelements: 2,
            classes: vec![
                HkArrayClassParam {
                    hkparam: "#0063".into(),
                },
                HkArrayClassParam {
                    hkparam: "#0064".into(),
                },
            ],
        };

        assert_eq!(deserialized, expected);
    }
}
