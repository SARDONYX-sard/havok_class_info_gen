// # NOTE
// This code is based on the cgmath crate code.
// Structures, new methods, and so on.
//
// Copyright 2013-2014 The CGMath Developers. For a full listing of the authors,
// refer to the Cargo.toml file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::havok_types::{float::rust_to_cpp_float_str, hk_array::normalize};

use super::{Vector3, Vector4};
use core::{fmt, str::FromStr};
use ordered_float::FloatCore;
use serde::{Deserialize, Serialize, Serializer};

/// Quaternion with (De)serialization for havok.
///
/// In XML, it would be as follows.
/// ```xml
/// <!-- < - - - -  vector3 - - - - > < scalar >   -->
/// <tag>(0.000000 0.000000  0.000000 0.000000)</tag>
/// ```
#[repr(C)]
#[derive(Debug, PartialEq, Default, Eq, Copy, Clone, Hash)]
pub struct Quaternion<S: FloatCore> {
    /// The vector part of the quaternion.
    pub v: Vector3<S>,
    /// The scalar part of the quaternion.
    pub s: S,
}

impl<S: FloatCore> Quaternion<S> {
    /// Construct a new quaternion from one scalar component and three
    /// imaginary components.
    #[inline]
    pub const fn new(w: S, xi: S, yj: S, zk: S) -> Quaternion<S> {
        Quaternion::from_sv(w, Vector3::new(xi, yj, zk))
    }

    /// Construct a new quaternion from a scalar and a vector.
    #[inline]
    pub const fn from_sv(s: S, v: Vector3<S>) -> Quaternion<S> {
        Quaternion { s, v }
    }
}

impl<S: FloatCore> From<[S; 4]> for Quaternion<S> {
    fn from(value: [S; 4]) -> Self {
        Self {
            v: Vector3::new(value[0], value[1], value[2]),
            s: value[3],
        }
    }
}

impl<S: FloatCore> From<Vector4<S>> for Quaternion<S> {
    fn from(value: Vector4<S>) -> Self {
        Self {
            v: Vector3::new(value.x.0, value.y.0, value.z.0),
            s: value.w.0,
        }
    }
}

//# Attention
// Since [`Serialize`] of [`Matrix4`] is implemented manually and [`to_string`] is called inside it, the format of [`Display`]trait is reflected in the string format of [`Serialize`].
impl<T: fmt::Display + FloatCore> fmt::Display for Quaternion<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x = rust_to_cpp_float_str(self.v.x);
        let y = rust_to_cpp_float_str(self.v.y);
        let z = rust_to_cpp_float_str(self.v.z);
        let s = rust_to_cpp_float_str(self.s);
        write!(f, "({x} {y} {z} {s})",)
    }
}

impl<T> Serialize for Quaternion<T>
where
    T: FloatCore + fmt::Display + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de, T> Deserialize<'de> for Quaternion<T>
where
    T: FloatCore + FromStr + Copy,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        /// Need to use [`std::marker::PhantomData`] because of generics in `Matrix4` for Value target.
        struct Visitor<T>(std::marker::PhantomData<T>);

        impl<'de, T> serde::de::Visitor<'de> for Visitor<T>
        where
            T: FloatCore + FromStr + Copy,
        {
            type Value = Quaternion<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string representing a Quaternion")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                // Vector4 is enclosed in `()` in XML, so extract an array of Vector4 separated by `)`.
                let parts = normalize(s);
                let parts_len = parts.len();
                if parts_len != 4 {
                    let err_msg = format!("Quaternion is expected 4(Vector3 & Scale) str. But got len: {parts_len} & content: {parts:?}");
                    return Err(serde::de::Error::custom(err_msg));
                }

                let values: Result<Vec<T>, _> = parts.iter().map(|p| p.parse()).collect();
                match values {
                    Ok(v) => Ok(Quaternion {
                        v: Vector3::new(v[0], v[1], v[2]),
                        s: v[3],
                    }),
                    Err(_) => Err(serde::de::Error::custom("Failed to parse values")),
                }
            }
        }

        deserializer.deserialize_str(Visitor(std::marker::PhantomData))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    /// # Why do we need this test structure?
    /// To start parsing quick_xml, a structure definition that serves as the Root is required.
    ///
    /// # Note
    /// Tests pass even with a tag with different name (e.g., `<Root>` instead of `<TestRoot>`) during XML deserialization.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct TestRoot<T>(T);

    #[test]
    fn should_serialize() {
        let root = TestRoot(Quaternion::from_sv(
            0.000000,
            Vector3::from([0.000000, 0.000000, f32::NEG_INFINITY]),
        ));

        assert_eq!(
            quick_xml::se::to_string(&root).unwrap(),
            "\
            <TestRoot>\
                (0.000000 0.000000 -1.#INF00 0.000000)\
            </TestRoot>"
        );
    }

    #[test]
    fn should_deserialize() {
        let xml = "
        <Root>\
            (1.000000 1.000000 1.000000 0.000000)\
        </Root>";

        assert_eq!(
            quick_xml::de::from_str::<TestRoot<Quaternion<f32>>>(xml).unwrap(),
            TestRoot(Quaternion::from_sv(0.0, [1.0, 1.0, 1.0].into()))
        );
    }
}
