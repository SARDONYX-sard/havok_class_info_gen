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

use super::Vector3;
use core::{fmt, str::FromStr};
use ordered_float::FloatCore;
use serde::{de::IntoDeserializer, Deserialize, Serialize, Serializer};

/// Matrix 3x3 with (De)serialization for havok.
///
/// In XML, it would be as follows.
/// ```xml
/// <tag>(0.000000 0.000000 0.000000)(-0.000000 0.000000 -0.000000)(1.000000 1.000000 1.000000)</tag>
/// ```
#[repr(C)]
#[derive(Debug, PartialEq, Default, Eq, Copy, Clone, Hash)]
pub struct Matrix3<S: FloatCore> {
    /// The first column of the matrix.
    pub x: Vector3<S>,
    /// The second column of the matrix.
    pub y: Vector3<S>,
    /// The third column of the matrix.
    pub z: Vector3<S>,
}

impl<S: FloatCore> Matrix3<S> {
    /// Create a new matrix, providing values for each index.
    #[allow(clippy::too_many_arguments)]
    #[inline]
    #[rustfmt::skip]
    pub const fn new(
        c0r0: S, c0r1: S, c0r2: S,
        c1r0: S, c1r1: S, c1r2: S,
        c2r0: S, c2r1: S, c2r2: S
    ) -> Self  {
        Self::from_cols(
            Vector3::new(c0r0, c0r1, c0r2),
            Vector3::new(c1r0, c1r1, c1r2),
            Vector3::new(c2r0, c2r1, c2r2),
        )
    }

    /// Create a new matrix, providing columns.
    #[inline]
    pub const fn from_cols(c0: Vector3<S>, c1: Vector3<S>, c2: Vector3<S>) -> Self {
        Self {
            x: c0,
            y: c1,
            z: c2,
        }
    }
}

impl<S: FloatCore> From<(Vector3<S>, Vector3<S>, Vector3<S>)> for Matrix3<S> {
    fn from(vector3_tuple: (Vector3<S>, Vector3<S>, Vector3<S>)) -> Self {
        Self {
            x: vector3_tuple.0,
            y: vector3_tuple.1,
            z: vector3_tuple.2,
        }
    }
}

impl<S: FloatCore> From<[Vector3<S>; 3]> for Matrix3<S> {
    fn from(vector3_array: [Vector3<S>; 3]) -> Self {
        Self {
            x: vector3_array[0],
            y: vector3_array[1],
            z: vector3_array[2],
        }
    }
}

//# Attention
// Since [`Serialize`] of [`Matrix4`] is implemented manually and [`to_string`] is called inside it, the format of [`Display`]trait is reflected in the string format of [`Serialize`].
impl<T: fmt::Display + FloatCore> fmt::Display for Matrix3<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        write!(f, "{x}{y}{z}",)
    }
}

impl<T: FloatCore> Serialize for Matrix3<T>
where
    T: fmt::Display + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // use Display trait by `to_string`
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de, T> Deserialize<'de> for Matrix3<T>
where
    T: FloatCore + FromStr,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        /// Need to use [`std::marker::PhantomData`] because of generics in `Matrix4` for Value target.
        struct Visitor<T>(std::marker::PhantomData<T>);

        impl<'de, T> serde::de::Visitor<'de> for Visitor<T>
        where
            T: FloatCore + FromStr,
        {
            type Value = Matrix3<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string representing a Matrix3")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                // Vector4 is enclosed in `()` in XML, so extract an array of Vector4 separated by `)`.
                let parts: Vec<_> = s.split(')').filter(|s| !s.is_empty()).collect();
                let parts_len = parts.len();
                if parts_len != 3 {
                    let err_msg = format!("Matrix 3 is expected 3 Vector3 str. But got len: {parts_len} & content: {parts:?}");
                    return Err(serde::de::Error::custom(err_msg));
                }

                let values: Result<Vec<Vector3<T>>, E> = parts
                    .iter()
                    .map(|vec3_str| Vector3::deserialize(vec3_str.into_deserializer()))
                    .collect();

                values.map(|v| {
                    let mut iter = v.into_iter();
                    Matrix3::from_cols(
                        //? Safety: It is safe to use `unwrap` because we confirmed above that four exist when `str::split`.
                        iter.next().unwrap(),
                        iter.next().unwrap(),
                        iter.next().unwrap(),
                    )
                })
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
        let root = TestRoot(Matrix3::from([
            Vector3::from([0.000000, 0.000000, 0.000000]),
            Vector3::from([f32::NAN, 0.000000, 0.000000]),
            Vector3::from([f32::NAN, 0.000000, 0.000000]),
        ]));

        assert_eq!(
            quick_xml::se::to_string(&root).unwrap(),
            "\
            <TestRoot>\
                (0.000000 0.000000 0.000000)(-1.#IND00 0.000000 0.000000)(-1.#IND00 0.000000 0.000000)
            </TestRoot>"
        );
    }

    #[test]
    fn should_deserialize() {
        let xml = "
        <Root>\
            (0.000000 0.000000  0.000000)(-0.000000 0.000000 -0.000000)(1.000000 1.000000 1.000000)
        </Root>";
        let deserialized: TestRoot<Matrix3<f32>> = quick_xml::de::from_str(xml).unwrap();

        let expected = Matrix3::from([
            [0.0, 0.0, 0.0].into(),
            [-0.0, 0.0, -0.0].into(),
            [1.0, 1.0, 1.0].into(),
        ]);
        assert_eq!(deserialized, TestRoot(expected));
    }
}
