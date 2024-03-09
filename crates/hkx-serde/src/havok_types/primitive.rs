use serde::{Deserialize, Serialize};

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
    #[serde(rename = "$text")]
    value: T,
}

/* impl<T> Serialize for Primitive<T>
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
} */

/* impl<'de, T> Deserialize<'de> for Primitive<T>
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
                let mut value = None;

                while let Some(key) = map.next_key::<Cow<'_, str>>()? {
                    match key.as_bytes() {
                        b"$text" => {
                            let text: Cow<'_, str> = map.next_value()?;
                            value = Some(text.parse().map_err(|_err| {
                                serde::de::Error::custom("Failed to parse text as u64")
                            })?);
                        }
                        unknown => {
                            let key = String::from_utf8_lossy(unknown);
                            let content: Cow<'_, str> = map.next_value()?;
                            tracing::warn!("Got unknown field. key: {key}, value: {content}",);
                        }
                    }
                }

                let value = value.unwrap_or_default();

                Ok(Primitive { value })
            }
        }

        deserializer.deserialize_map(InnerVisitor(std::marker::PhantomData))
    }
} */

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
