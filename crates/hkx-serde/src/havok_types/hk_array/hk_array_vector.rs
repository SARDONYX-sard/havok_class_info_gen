use serde::{de::IntoDeserializer, Deserialize, Serialize};
use std::borrow::Cow;

/// A structure for storing special arrays enclosed in `()`, such as Vector4, and for performing normal (De)serialization.
#[derive(Debug, Default, PartialEq, Serialize)]
#[serde(rename = "hkparam")]
pub struct HkArrayVector<T> {
    /// Length of [`Vec`] stored in its own structure.
    #[serde(rename = "@numelements")]
    pub numelements: usize,
    /// [`Vec`], which stores [`Vector4`], etc.
    #[serde(rename = "$value")]
    pub value: Vec<T>,
}

//Vec using quick_xml's default special behaviors such as $value and $text does not support arbitrary deserialization with `()`.
// Therefore, manual implementation is required.
impl<'de, T> Deserialize<'de> for HkArrayVector<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct HkArrayValueVisitor<T> {
            marker: std::marker::PhantomData<T>,
        }

        impl<'de, T> serde::de::Visitor<'de> for HkArrayValueVisitor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = HkArrayVector<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct HkArrayVector")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                use serde::de::Error;
                let mut numelements = None;
                let mut value = None;

                while let Some(key) = map.next_key::<Cow<'_, str>>()? {
                    match key.as_bytes() {
                        b"@numelements" => {
                            if numelements.is_some() {
                                return Err(Error::duplicate_field("@numelements"));
                            }
                            numelements = Some(map.next_value()?);
                        }
                        b"$text" => {
                            let text: Cow<'_, str> = map.next_value()?;

                            let mut value_inner = Vec::new();
                            for line in text.split(['(']).filter(|line| !line.is_empty()) {
                                value_inner.push(T::deserialize(line.into_deserializer())?);
                            }

                            value = Some(value_inner);
                        }
                        unknown => {
                            let key = String::from_utf8_lossy(unknown);
                            let content: Cow<'_, str> = map.next_value()?;
                            tracing::warn!("Got unknown field. key: {key}, value: {content}",);
                        }
                    }
                }

                let mut numelements =
                    numelements.ok_or_else(|| Error::missing_field("@numelements"))?;
                let value = value.unwrap_or_default();

                let vec_len = value.len();
                if numelements != vec_len {
                    tracing::warn!("XML value ({numelements}) & array length ({vec_len}) in XML do not match. Automatically correct to the length of the array.");
                    numelements = value.len();
                };

                Ok(HkArrayVector { numelements, value })
            }
        }

        deserializer.deserialize_map(HkArrayValueVisitor {
            marker: std::marker::PhantomData,
        })
    }
}

impl<T> From<Vec<T>> for HkArrayVector<T> {
    fn from(value: Vec<T>) -> Self {
        Self {
            numelements: value.len(),
            value,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::havok_types::Vector4;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize() {
        let data: HkArrayVector<_> = vec![
            Vector4::from((0.0, 0.0, f32::NAN, 0.0)),
            Vector4::from((0.0, 1.0, 0.0, 0.0)),
        ]
        .into();
        let serialized = quick_xml::se::to_string(&data).unwrap();

        let expected_xml = "\
            <hkparam numelements=\"2\">\
                (0.000000 0.000000 -1.#IND00 0.000000)\
                (0.000000 1.000000 0.000000 0.000000)\
            </hkparam>\
        ";

        assert_eq!(serialized, expected_xml);
    }

    #[test]
    fn should_deserialize() {
        let xml = r###"
            <hkparam name="variantVariableValues" numelements="2">
                (0.000000 0.000000 0.000000 0.000000)
                (0.000000 1.000000 0.000000 0.000000)
                (0.000000 0.000000 1.000000 0.000000)
            </hkparam>
        "###;
        let deserialized: HkArrayVector<Vector4<f32>> = quick_xml::de::from_str(xml).unwrap();

        let expected = HkArrayVector {
            numelements: 3,
            value: vec![
                Vector4::from((0.0, 0.0, 0.0, 0.0)),
                Vector4::from((0.0, 1.0, 0.0, 0.0)),
                Vector4::from((0.0, 0.0, 1.0, 0.0)),
            ],
        };

        assert_eq!(deserialized, expected);
    }
}
