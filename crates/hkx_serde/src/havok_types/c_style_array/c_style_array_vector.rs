use serde::{Deserialize, Serialize};

/// Implement `From<Vec<T>>` & [`serde::Serialize`]/[`serde::Deserialize`]  for `HkArrayCStyle`
macro_rules! impl_serde_for_c_style_array {
    ($struct_name:ident, $sep:literal, $chunk_size:literal) => {
        /// C style arrays(Limit-Size) such as `Vector4` and `Matrix3`.
        ///
        /// This is almost the same as for `hkArray<Matrix3>`, etc., but with the following differences
        /// - There is no `numelements` attribute.
        /// - The number of elements is limited.
        ///
        /// # Note
        /// Extra values are ignored.(e.g. `[Vector4<f32>; 10]` => `[[Vector4<f32>; 5]`)
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $struct_name<T, const N: usize>
        where
            T: Default + Copy + Serialize + core::fmt::Display,
        {
            /// Limit Array which stores [`[Matrix3; 4`], etc.
            pub value: [T; N],
        }

        impl<T, const N: usize> Default for $struct_name<T, N>
        where
            T: Default + Copy + Serialize + core::fmt::Display,
        {
            fn default() -> Self {
                Self {
                    value: [Default::default(); N],
                }
            }
        }

        impl<T, const N: usize> $struct_name<T, N>
        where
            T: Default + Copy + Serialize + core::fmt::Display,
        {
            /// Take inner value.
            pub fn into_inner(self) -> [T; N] {
                self.value
            }
        }

        impl<T, const N: usize> From<[T; N]> for $struct_name<T, N>
        where
            T: Default + Copy + Serialize + core::fmt::Display,
        {
            fn from(value: [T; N]) -> Self {
                Self { value }
            }
        }

        impl<T, const N: usize> Serialize for $struct_name<T, N>
        where
            T: Default + Copy + Serialize + core::fmt::Display,
        {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(&format!(
                    "{}",
                    self.value
                        .iter()
                        .map(|&x| x.to_string())
                        .collect::<String>()
                ))
            }
        }

        //Vec using quick_xml's default special behaviors such as $value and $text does not support arbitrary deserialization with `()`.
        // Therefore, manual implementation is required.
        impl<'de, T, const N: usize> Deserialize<'de> for $struct_name<T, N>
        where
            T: Deserialize<'de> + Default + Copy + Serialize + core::fmt::Display,
        {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct HkArrayValueVisitor<T, const N: usize> {
                    marker: std::marker::PhantomData<[T; N]>,
                }

                impl<'de, T, const N: usize> serde::de::Visitor<'de> for HkArrayValueVisitor<T, N>
                where
                    T: Deserialize<'de> + Default + Copy + Serialize + core::fmt::Display,
                {
                    type Value = $struct_name<T, N>;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str(stringify!("struct "$struct_name))
                    }

                    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                    where
                        A: serde::de::MapAccess<'de>,
                    {
                        use crate::helpers::str_ext::SplitExt as _;
                        use serde::de::{Error, IntoDeserializer as _};
                        use std::borrow::Cow;

                        let mut value = None;

                        while let Some(key) = map.next_key::<Cow<'_, str>>()? {
                            match key.as_bytes() {
                                b"$text" => {
                                    if value.is_some() {
                                        return Err(Error::duplicate_field("$text"));
                                    }

                                    let mut value_inner = [T::default(); N];

                                    let text: Cow<'_, str> = map.next_value()?;
                                    let text = text.as_ref();
                                    let chunks: Vec<_> = text
                                        .chunk($sep, $chunk_size)
                                        .into_iter()
                                        .filter(|s| !s.is_empty())
                                        .collect();

                                    let chunk_size = chunks.len();
                                    if chunk_size != N {
                                        let err_msg = format!(
                                            "Expected {N} chunks, but got {chunk_size} in {}",
                                            stringify!($struct_name)
                                        );
                                        return Err(Error::custom(err_msg));
                                    }

                                    // Expect a string like "(1.0 0.0 0.0 0.0)" for the `unit` variable.
                                    for (i, unit) in chunks.iter().enumerate() {
                                        value_inner[i] = T::deserialize(unit.into_deserializer())?;
                                    }

                                    value = Some(value_inner);
                                }
                                _unknown => {
                                    // ! The key and value calls must be called in pairs or else **it will error**.
                                    let _ = map.next_value()?;
                                }
                            }
                        }

                        let value = value.unwrap_or([T::default(); N]);
                        Ok($struct_name { value })
                    }
                }

                deserializer.deserialize_map(HkArrayValueVisitor {
                    marker: std::marker::PhantomData,
                })
            }
        }
    };
}

impl_serde_for_c_style_array!(CStyleArrayVector, ')', 1);
impl_serde_for_c_style_array!(CStyleArrayMatrix3, ')', 3);
impl_serde_for_c_style_array!(CStyleArrayMatrix4, ')', 4);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::havok_types::Vector4;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize_vector4() {
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(rename = "hkparam")]
        struct Root(CStyleArrayVector<Vector4<f32>, 2>);

        let data: CStyleArrayVector<Vector4<f32>, 2> = [
            Vector4::from((0.0, 0.0, f32::NAN, 0.0)),
            Vector4::from((0.0, 1.0, 0.0, 0.0)),
        ]
        .into();
        let serialized = quick_xml::se::to_string(&Root(data)).unwrap();

        let expected_xml = "\
            <hkparam>\
                (0.000000 0.000000 -1.#IND00 0.000000)\
                (0.000000 1.000000 0.000000 0.000000)\
            </hkparam>\
        ";

        assert_eq!(serialized, expected_xml);
    }

    #[test]
    fn should_deserialize_vector4() {
        let xml = r###"
            <hkparam name="variantVariableValues" numelements="2">
                (0.000000 0.000000 0.000000 0.000000)
                (0.000000 1.000000 0.000000 0.000000)
                (0.000000 0.000000 1.000000 0.000000)
            </hkparam>
        "###;
        let deserialized: CStyleArrayVector<Vector4<f32>, 3> =
            quick_xml::de::from_str(xml).unwrap();

        let expected = CStyleArrayVector {
            value: [
                Vector4::from((0.0, 0.0, 0.0, 0.0)),
                Vector4::from((0.0, 1.0, 0.0, 0.0)),
                Vector4::from((0.0, 0.0, 1.0, 0.0)),
            ],
        };

        assert_eq!(deserialized, expected);
    }
}
