use serde::{Deserialize, Serialize};
use std::{borrow::Cow, str::FromStr};

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
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Primitive<T> {
    /// # Note
    /// The fields are named because XML parsing failed in the form of `StructName(T)`.
    value: T,
}

// # Why do we implement manually?
// Space in $str and so on are not taken into account in $text, so you should switch to manual implementation if necessary.
impl<T> Serialize for Primitive<T>
where
    T: core::fmt::Display + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // use Display trait by `to_string`
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de, T> Deserialize<'de> for Primitive<T>
where
    T: std::default::Default + FromStr,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct InnerVisitor<T>(std::marker::PhantomData<T>);

        impl<'de, T> serde::de::Visitor<'de> for InnerVisitor<T>
        where
            T: std::default::Default + FromStr,
        {
            type Value = Primitive<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Primitive<T>")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                use serde::de::Error;
                let mut value = None;

                while let Some(key) = map.next_key::<Cow<'_, str>>()? {
                    match key.as_bytes() {
                        b"$text" => {
                            let text: Cow<'_, str> = map.next_value()?;
                            value = Some(
                                text.parse()
                                    .map_err(|_err| Error::custom("Failed to parse text as T"))?,
                            );
                        }
                        _unknown => {}
                    }
                }

                let value =
                    value.ok_or_else(|| Error::custom("Failed to get value in Primitive"))?;

                Ok(Primitive { value })
            }
        }

        deserializer.deserialize_map(InnerVisitor(std::marker::PhantomData))
    }
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
        let deserialized_primitive: Primitive<String> = quick_xml::de::from_str(xml).unwrap();

        assert_eq!(
            deserialized_primitive,
            Primitive {
                value: "text text".into()
            }
        );
    }
}
