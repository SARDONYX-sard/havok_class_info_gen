use super::Vector3;
use crate::helpers::{number::rust_to_cpp_float_str, vector::normalize};
use core::{fmt, str::FromStr};
use ordered_float::{FloatCore, OrderedFloat};
use serde::{Deserialize, Serialize, Serializer};

/// Vector4 with (De)serialization for havok.
///
/// In XML, it is like a tuple enclosed in parentheses, such as `(-1.#IND00 0.000000 -1.#INF00 0.000000)`.
///
/// # NOTE
/// It seems that [`Self::w`] is always 0.
#[repr(C)]
#[derive(Debug, PartialEq, Default, Eq, Copy, Clone, Hash)]
pub struct Vector4<S: FloatCore> {
    /// The x component of the vector.
    pub x: OrderedFloat<S>,
    /// The y component of the vector.
    pub y: OrderedFloat<S>,
    /// The z component of the vector.
    pub z: OrderedFloat<S>,
    /// The w component of the vector.
    pub w: OrderedFloat<S>,
}

impl<S: FloatCore> Vector4<S> {
    /// Creates a new vector4
    pub const fn new(x: S, y: S, z: S, w: S) -> Self {
        Self {
            x: OrderedFloat(x),
            y: OrderedFloat(y),
            z: OrderedFloat(z),
            w: OrderedFloat(w),
        }
    }
}

impl<S: FloatCore> From<(S, S, S, S)> for Vector4<S> {
    fn from(value: (S, S, S, S)) -> Self {
        Self {
            x: value.0.into(),
            y: value.1.into(),
            z: value.2.into(),
            w: value.3.into(),
        }
    }
}

impl<S: FloatCore> From<[S; 4]> for Vector4<S> {
    fn from(value: [S; 4]) -> Self {
        Self {
            x: value[0].into(),
            y: value[1].into(),
            z: value[2].into(),
            w: value[3].into(),
        }
    }
}

impl<S: FloatCore + Default> From<Vector3<S>> for Vector4<S> {
    fn from(vector3: Vector3<S>) -> Self {
        Self {
            x: vector3.x,
            y: vector3.y,
            z: vector3.z,
            w: OrderedFloat::default(),
        }
    }
}

//# Attention
// Since [`Serialize`] of [`Vector4`] is implemented manually and [`to_string`] is called inside it, the format of [`Display`]trait is reflected in the string format of [`Serialize`].
impl<T: fmt::Display + FloatCore> fmt::Display for Vector4<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x = rust_to_cpp_float_str(self.x);
        let y = rust_to_cpp_float_str(self.y);
        let z = rust_to_cpp_float_str(self.z);
        let w = rust_to_cpp_float_str(self.w);
        write!(f, "({x} {y} {z} {w})",)
    }
}

impl<T: FloatCore> Serialize for Vector4<T>
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

impl<'de, T> Deserialize<'de> for Vector4<T>
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
            type Value = Vector4<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string representing a Vector4")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let parts = normalize(s);
                let parts_len = parts.len();
                if parts_len != 4 {
                    return Err(E::custom(format!("Vector4 is expected 4 values. But got len: {parts_len} & content: {parts:?}")));
                }
                let values: Result<Vec<T>, _> = parts.iter().map(|p| p.parse()).collect();
                match values {
                    Ok(v) => Ok(Vector4 {
                        x: v[0].into(),
                        y: v[1].into(),
                        z: v[2].into(),
                        w: v[3].into(),
                    }),
                    Err(_) => Err(E::custom("Failed to parse values")),
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
    struct Root(Vector4<f32>);

    #[test]
    fn should_serialize() {
        let vector4 = Vector4::from([f32::NAN, -0.000000, f32::INFINITY, 0.000000]);
        let serialized = quick_xml::se::to_string(&Root(vector4)).unwrap();

        assert_eq!(
            serialized,
            "<Root>(-1.#IND00 -0.000000 1.#INF00 0.000000)</Root>"
        );
    }

    #[test]
    fn should_deserialize() {
        let xml = "
        <Root>\
          (-1.#IND00 0.000000 -1.#INF00 0.000000)\
        </Root>";
        let deserialized: Root = quick_xml::de::from_str(xml).unwrap();

        let vector4 = Vector4::from([f32::NAN, 0.000000, f32::NEG_INFINITY, 0.000000]);
        assert_eq!(deserialized, Root(vector4));
    }
}
