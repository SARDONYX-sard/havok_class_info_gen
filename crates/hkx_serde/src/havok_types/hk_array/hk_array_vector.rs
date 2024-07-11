use serde::{de::IntoDeserializer, Deserialize, Serialize};
use std::borrow::Cow;

/// Implement `From<Vec<T>>` & [`serde::Serialize`]/[`serde::Deserialize`]  for `HkArray`
macro_rules! impl_serde_for_hk_array {
  ($struct_name:ident, $sep:literal, $chunk_size:literal) => {
impl<T> $struct_name<T> {
    /// Take inner value.
    pub fn into_inner(self) -> Vec<T> {
        self.values
    }
}

impl<T> From<Vec<T>> for $struct_name<T> {
    fn from(values: Vec<T>) -> Self {
        Self {
            numelements: values.len(),
            values,
        }
    }
}

//Vec using quick_xml's default special behaviors such as $value and $text does not support arbitrary deserialization with `()`.
// Therefore, manual implementation is required.
impl<'de, T> Deserialize<'de> for $struct_name<T>
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
            type Value = $struct_name<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(stringify!("struct "$struct_name))
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                use crate::helpers::str_ext::SplitExt as _;
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
                            if value.is_some() {
                                return Err(Error::duplicate_field("$text"));
                            }

                            let mut value_inner = Vec::new();

                            let text: Cow<'_, str> = map.next_value()?;
                            for line in text.as_ref().chunk($sep, $chunk_size).into_iter().filter(|s| !s.is_empty()) {
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
                let values = value.unwrap_or_default();

                let vec_len = values.len();
                if numelements != vec_len {
                    tracing::warn!("XML value ({numelements}) & array length ({vec_len}) in XML do not match. Automatically correct to the length of the array.");
                    numelements = values.len();
                };

                Ok($struct_name { numelements, values })
            }
        }

        deserializer.deserialize_map(HkArrayValueVisitor {
            marker: std::marker::PhantomData,
        })
    }
}

  }
}

impl_serde_for_hk_array!(HkArrayVector, ')', 1);
impl_serde_for_hk_array!(HkArrayMatrix3, ')', 3);
impl_serde_for_hk_array!(HkArrayMatrix4, ')', 4);

/// A structure for storing special arrays enclosed in `()`, such as Vector4, and for performing normal (De)serialization.
///
/// # Note
/// The `name` attribute is required for `hkparam` but is not included in this structure.
/// This is because the value of the `name` attribute corresponds to a C++ field name,
/// and the processing must be changed according to the value.
/// And to do that, we need the parent enum that wraps this structure.
///
/// In summary, the parent enum determines and retrieves the `name` attribute, so it is not included in this structure.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[serde(rename = "hkparam")]
pub struct HkArrayVector<T> {
    /// Length of [`Vec`] stored in its own structure.
    #[serde(rename = "@numelements")]
    pub numelements: usize,
    /// [`Vec`], which stores [`Vector4`], etc.
    #[serde(rename = "$value", default)]
    pub values: Vec<T>,
}

/// Array type with [`serde::Serialize`]/[`serde::Deserialize`] implemented for Havok for `Matrix3`.
///
/// # Note
/// The `name` attribute is required for `hkparam` but is not included in this structure.
/// This is because the value of the `name` attribute corresponds to a C++ field name,
/// and the processing must be changed according to the value.
/// And to do that, we need the parent enum that wraps this structure.
///
/// In summary, the parent enum determines and retrieves the `name` attribute, so it is not included in this structure.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[serde(rename = "hkparam")]
pub struct HkArrayMatrix3<T> {
    /// Length of [`Vec`] stored in its own structure.
    #[serde(rename = "@numelements")]
    pub numelements: usize,
    /// [`Vec`], which stores `Matrix3`, etc.
    ///
    /// # Info
    /// The reason we use `Vec<T>` instead of `Vec<Matrix3<T>>` is that we can include other structures besides `hkMatrix3`,
    /// such as `hkRotation`, which are equivalent in structure.
    #[serde(rename = "$value", default)]
    pub values: Vec<T>,
}

/// Array type with [`serde::Serialize`]/[`serde::Deserialize`] implemented for Havok for `Matrix4`.
///
/// # Note
/// The `name` attribute is required for `hkparam` but is not included in this structure.
/// This is because the value of the `name` attribute corresponds to a C++ field name,
/// and the processing must be changed according to the value.
/// And to do that, we need the parent enum that wraps this structure.
///
/// In summary, the parent enum determines and retrieves the `name` attribute, so it is not included in this structure.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[serde(rename = "hkparam")]
pub struct HkArrayMatrix4<T> {
    /// Length of [`Vec`] stored in its own structure.
    #[serde(rename = "@numelements")]
    pub numelements: usize,
    /// [`Vec`], which stores `Matrix4`, etc.
    #[serde(rename = "$value", default)]
    pub values: Vec<T>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::havok_types::{Matrix3, Vector3, Vector4};
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize_vector4() {
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
    fn should_deserialize_vector4() {
        let xml = r###"
            <hkparam name="variantVariableValues" numelements="2">
                (0.000000 0.000000 0.000000 0.000000)
                (0.000000 1.000000 0.000000 0.000000)
                (0.000000 0.000000 1.000000 0.000000)
            </hkparam>
        "###;
        let deserialized: HkArrayVector<Vector4<f32>> = quick_xml::de::from_str(xml).unwrap();

        let expected = HkArrayVector {
            numelements: 3, // Auto fix numelements
            values: vec![
                Vector4::from((0.0, 0.0, 0.0, 0.0)),
                Vector4::from((0.0, 1.0, 0.0, 0.0)),
                Vector4::from((0.0, 0.0, 1.0, 0.0)),
            ],
        };

        assert_eq!(deserialized, expected);
    }

    #[test]
    fn should_serialize_hk_array_matrix3() {
        let matrix3 = Matrix3::from([
            Vector3::from([0.000000, 0.000000, 0.000000]),
            Vector3::from([f32::NAN, 0.000000, 0.000000]),
            Vector3::from([f32::NAN, 0.000000, 0.000000]),
        ]);
        let hk_array_matrix3: HkArrayMatrix3<Matrix3<f32>> = vec![matrix3, matrix3, matrix3].into();

        assert_eq!(
            quick_xml::se::to_string(&hk_array_matrix3).unwrap(),
            "\
            <hkparam numelements=\"3\">\
                (0.000000 0.000000 0.000000)(-1.#IND00 0.000000 0.000000)(-1.#IND00 0.000000 0.000000)\
                (0.000000 0.000000 0.000000)(-1.#IND00 0.000000 0.000000)(-1.#IND00 0.000000 0.000000)\
                (0.000000 0.000000 0.000000)(-1.#IND00 0.000000 0.000000)(-1.#IND00 0.000000 0.000000)\
            </hkparam>"
        );
    }

    #[test]
    fn should_deserialize_hk_array_matrix3() {
        let xml = "
        <hkparam numelements=\"3\">
            (0.000000 0.000000 0.000000)(-0.000000 0.000000 -0.000000)(1.000000 1.000000 1.000000)
            (0.000000 0.000000 0.000000)(-0.000000 0.000000 -0.000000)(1.000000 1.000000 1.000000)
            (0.000000 0.000000 0.000000)(-0.000000 0.000000 -0.000000)(1.000000 1.000000 1.000000)
        </hkparam>";
        let deserialized: HkArrayMatrix3<Matrix3<f32>> = quick_xml::de::from_str(xml).unwrap();

        let expected = Matrix3::from([
            [0.0, 0.0, 0.0].into(),
            [-0.0, 0.0, -0.0].into(),
            [1.0, 1.0, 1.0].into(),
        ]);
        let expected_array = vec![expected, expected, expected].into();
        assert_eq!(deserialized, expected_array);
    }
}
