use serde::{Deserialize, Serialize};

// # Note
// The fields are named because XML parsing failed in the form of `StructName(T)`.

/// Wrapper for retrieving the value that resides directly under the XML tag identified by the tagged enum.
///
/// Without it, it cannot be retrieved properly.
///
/// # XML Example
///
/// We would like to get this value `2`
/// ```xml
/// <tag>2</tag>
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Primitive<T> {
    /// Value inner tag
    #[serde(rename = "$text")]
    value: T,
}

impl<T: core::fmt::Display> core::fmt::Display for Primitive<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<T> From<T> for Primitive<T> {
    fn from(value: T) -> Self {
        Self { value }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize() {
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        struct TestRoot<T>(T);

        assert_eq!(
            quick_xml::se::to_string(&TestRoot(Primitive::<i32> { value: 42 })).unwrap(),
            "<TestRoot>42</TestRoot>"
        );
    }

    #[test]
    fn should_deserialize() {
        let xml = r#"<AnyTagName>42</AnyTagName>"#;
        let deserialized_primitive: Primitive<i32> = quick_xml::de::from_str(xml).unwrap();

        assert_eq!(deserialized_primitive, Primitive::<i32> { value: 42 });
    }

    #[test]
    fn should_serialize_with_space_value() {
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        struct TestRoot<T>(T);

        assert_eq!(
            quick_xml::se::to_string(&TestRoot(Primitive { value: "text text" })).unwrap(),
            "<TestRoot>text text</TestRoot>"
        );
    }

    #[test]
    fn should_deserialize_with_space_value() {
        let xml = r#"<AnyTagName>text text</AnyTagName>"#;
        let deserialized: Primitive<Cow<'_, str>> = quick_xml::de::from_str(xml).unwrap();

        assert_eq!(
            deserialized,
            Primitive {
                value: "text text".into()
            }
        );
    }
}
