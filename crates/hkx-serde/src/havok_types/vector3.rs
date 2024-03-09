use crate::havok_types::hk_array::normalize;
use crate::helpers::float::rust_to_cpp_float_str;
use core::{fmt, str::FromStr};
use ordered_float::{FloatCore, OrderedFloat};
use serde::{Deserialize, Serialize, Serializer};

use super::Vector4;

/// Vector3 with (De)serialization for havok.
///
/// In XML, it is like a tuple enclosed in parentheses, such as `(-1.#IND00 0.000000 -1.#INF00)`.
#[repr(C)]
#[derive(Debug, PartialEq, Default, Eq, Copy, Clone, Hash)]
pub struct Vector3<S: FloatCore> {
    /// The x component of the vector.
    pub x: OrderedFloat<S>,
    /// The y component of the vector.
    pub y: OrderedFloat<S>,
    /// The z component of the vector.
    pub z: OrderedFloat<S>,
}

impl<S: FloatCore> Vector3<S> {
    /// Creates a new vector4
    pub const fn new(x: S, y: S, z: S) -> Self {
        Self {
            x: OrderedFloat(x),
            y: OrderedFloat(y),
            z: OrderedFloat(z),
        }
    }
}

impl<S: FloatCore> From<(S, S, S)> for Vector3<S> {
    fn from(value: (S, S, S)) -> Self {
        Self {
            x: value.0.into(),
            y: value.1.into(),
            z: value.2.into(),
        }
    }
}

impl<S: FloatCore> From<[S; 3]> for Vector3<S> {
    fn from(value: [S; 3]) -> Self {
        Self {
            x: value[0].into(),
            y: value[1].into(),
            z: value[2].into(),
        }
    }
}

impl<S: FloatCore> From<Vector4<S>> for Vector3<S> {
    fn from(value: Vector4<S>) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

//# Attention
// Since [`Serialize`] of [`Vector3`] is implemented manually and [`to_string`] is called inside it, the format of [`Display`]trait is reflected in the string format of [`Serialize`].
impl<T: fmt::Display + FloatCore> fmt::Display for Vector3<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x = rust_to_cpp_float_str(self.x);
        let y = rust_to_cpp_float_str(self.y);
        let z = rust_to_cpp_float_str(self.z);
        write!(f, "({x} {y} {z})",)
    }
}

impl<T: FloatCore> Serialize for Vector3<T>
where
    T: fmt::Display + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de, T> Deserialize<'de> for Vector3<T>
where
    T: FloatCore + FromStr + Copy,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Vector4Visitor<T>(std::marker::PhantomData<T>);

        impl<'de, T> serde::de::Visitor<'de> for Vector4Visitor<T>
        where
            T: FloatCore + FromStr + Copy,
        {
            type Value = Vector3<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string representing a Vector4")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let parts = normalize(s);
                if parts.len() != 3 {
                    let err_msg = format!("Vector3 is expected 3 values. But got {:?}", parts);
                    return Err(serde::de::Error::custom(err_msg));
                }
                let values: Result<Vec<T>, _> = parts.iter().map(|p| p.parse()).collect();
                match values {
                    Ok(v) => Ok(Vector3 {
                        x: v[0].into(),
                        y: v[1].into(),
                        z: v[2].into(),
                    }),
                    Err(_) => Err(serde::de::Error::custom("Failed to parse values")),
                }
            }
        }

        deserializer.deserialize_str(Vector4Visitor(std::marker::PhantomData))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct Root(Vector3<f32>);

    #[test]
    fn should_serialize() {
        let vector3 = Vector3::from([f32::NAN, -0.000000, f32::INFINITY]);
        let serialized = quick_xml::se::to_string(&Root(vector3)).unwrap();

        assert_eq!(serialized, "<Root>(-1.#IND00 -0.000000 1.#INF00)</Root>");
    }

    #[test]
    fn should_deserialize() {
        let xml = "
        <Root>\
          (-1.#IND00 0.000000 -1.#INF00)\
        </Root>";
        let deserialized: Root = quick_xml::de::from_str(xml).unwrap();

        let vector3 = Vector3::from([f32::NAN, 0.000000, f32::NEG_INFINITY]);
        assert_eq!(deserialized, Root(vector3));
    }
}
