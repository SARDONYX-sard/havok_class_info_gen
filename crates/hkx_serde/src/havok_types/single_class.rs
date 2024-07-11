//! Structure for use in a `hkparam` field containing only one class (`hkobject`)
use serde::{Deserialize, Serialize};

/// # Single Class
/// In C++, this is the case when the type of the in-class field is a class.
///
/// In XML, A type to hold another class of `hkparam` within `hkparam`.
///
/// e.g. `variableInfos` field of `hkbBehaviorGraphData` class
///
/// # XML Example
/// ```xml
/// <hkparam name="variantVariableValues" numelements="2">
///     <hkobject>
///         <hkparam name="class_field">#0063</hkparam>
///     </hkobject>
/// </hkparam>
/// ```
///
/// # Note
/// The `name` attribute is required for `hkparam` but is not included in this structure.
/// This is because the value of the `name` attribute corresponds to a C++ field name,
/// and the processing must be changed according to the value.
/// And to do that, we need the parent enum that wraps this structure.
///
/// In summary, the parent enum determines and retrieves the `name` attribute, so it is not included in this structure.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkparam")]
pub struct SingleClass<T> {
    /// Receives `hkparam` of a certain class.
    ///
    /// The XML for this class does not require `name` and `signature`, but only encloses `hkobject` tags without attributes.
    #[serde(rename = "hkobject")]
    pub class: SingleClassParam<T>,
}

impl<T> SingleClass<T> {
    /// Take inner value.
    pub fn into_inner(self) -> T {
        self.class.hkparam
    }
}

impl<T> From<SingleClassParam<T>> for SingleClass<T> {
    fn from(class: SingleClassParam<T>) -> Self {
        Self { class }
    }
}

impl<T> From<T> for SingleClass<T> {
    fn from(values: T) -> Self {
        Self {
            class: SingleClassParam { hkparam: values },
        }
    }
}

/// One class of `HkArray`
///
/// ```xml
/// <hkobject>
///     <hkparam>#0063</hkparam>
///     <hkparam>#0064</hkparam>
/// </hkobject>
/// ````
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SingleClassParam<T> {
    /// Expected [`Vec<&str>`], [`i16`], etc.
    #[serde(rename = "hkparam")]
    pub hkparam: T,
}

impl<T> From<T> for SingleClassParam<T> {
    fn from(value: T) -> Self {
        Self { hkparam: value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize() {
        let data: SingleClass<Vec<i32>> = vec![1045220557, 0].into();
        let serialized = quick_xml::se::to_string(&data).unwrap();

        let expected_xml = "\
            <hkparam>\
                <hkobject>\
                    <hkparam>\
                        1045220557\
                    </hkparam>\
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
                    <hkparam>#0063</hkparam>
                </hkobject>
            </hkparam>
        "###;
        let deserialized: SingleClass<Vec<&str>> = quick_xml::de::from_str(xml).unwrap();

        let expected = SingleClass {
            class: SingleClassParam {
                hkparam: vec!["#0063", "#0063"],
            },
        };

        assert_eq!(deserialized, expected);
    }
}
