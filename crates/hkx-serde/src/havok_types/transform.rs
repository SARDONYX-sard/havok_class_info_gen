use super::{Matrix3, Vector3, Vector4};
use crate::helpers::str_ext::SplitExt as _;
use core::{fmt, str::FromStr};
use ordered_float::FloatCore;
use serde::{de::IntoDeserializer, Deserialize, Serialize, Serializer};

/// A `Transform` type that use `Matrix3`(`rotation`) with (De)serialization for havok.
///
/// In XML, it would be as follows.
/// ```xml
/// <tag>
///     <!--   matrix3 rotation  -->
///     (0.000000 0.000000 0.000000)
///     (0.000000 0.000000 0.000000)
///     (0.000000 0.000000 0.000000)
///     <!--     vector4 transition         -->
///     (-0.000000 0.000000 -0.000000)
/// </tag>
/// ```
///
/// # Note
/// The `transition` types are [`Vector4`], but only three are actually used, so [`Vector4::w`] is always 0.0
///
/// This is probably to facilitate affine transformations(Calculation to change the coordinates by multiplying the coordinates by a matrix).
#[repr(C)]
#[derive(Debug, PartialEq, Default, Eq, Copy, Clone, Hash)]
pub struct Transform<S: FloatCore> {
    /// Representing rotation.
    pub rotation: Matrix3<S>,
    /// Representing transitions.
    ///
    /// # Note
    /// `Vector4::w`(4th) isn't used.
    pub transition: Vector4<S>,
}

impl<S: FloatCore> Transform<S> {
    /// Create a new matrix, providing values for each index.
    #[allow(clippy::too_many_arguments)]
    #[inline]
    #[rustfmt::skip]
    pub const fn new(
        c0r0: S, c0r1: S, c0r2: S,
        c1r0: S, c1r1: S, c1r2: S,
        c2r0: S, c2r1: S, c2r2: S,
        c3r0: S, c3r1: S, c3r2: S, c3r3: S,
    ) -> Transform<S>  {
        Transform::from_cols(
            Matrix3::new(c0r0, c0r1, c0r2, c1r0, c1r1, c1r2 , c2r0, c2r1, c2r2),
            Vector4::new(c3r0, c3r1, c3r2, c3r3),
        )
    }

    /// Create a new matrix, providing columns.
    #[inline]
    pub const fn from_cols(c0: Matrix3<S>, c1: Vector4<S>) -> Self {
        Self {
            rotation: c0,
            transition: c1,
        }
    }
}

//# Attention
// Since [`Serialize`] of [`Matrix4`] is implemented manually and [`to_string`] is called inside it, the format of [`Display`]trait is reflected in the string format of [`Serialize`].
impl<S: fmt::Display + FloatCore> fmt::Display for Transform<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r = self.rotation;
        let t: Vector3<S> = self.transition.into();
        write!(f, "{r}{t}",)
    }
}

impl<T: FloatCore> Serialize for Transform<T>
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

impl<'de, T> Deserialize<'de> for Transform<T>
where
    T: FloatCore + FromStr + Default,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        /// Need to use [`std::marker::PhantomData`] because of generics in `Matrix4` for Value target.
        struct Visitor<T>(std::marker::PhantomData<T>);

        impl<'de, T> serde::de::Visitor<'de> for Visitor<T>
        where
            T: FloatCore + FromStr + Default,
        {
            type Value = Transform<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string representing a Transform")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let (matrix3, vec3) = s.split_at_nth(')', 3).ok_or_else(|| {
                    let err_msg = "Transform is expected 2(Matrix3 & Vector3) str. But got None";
                    serde::de::Error::custom(err_msg)
                })?;

                Ok(
                    //? Safety: It is safe to use [`unwrap`] because we confirmed above that four exist when [`str::split`].
                    Transform {
                        rotation: Matrix3::deserialize(matrix3.into_deserializer())?,
                        transition: Vector3::deserialize(vec3.into_deserializer())?.into(),
                    },
                )
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
        let root = TestRoot(Transform::from_cols(
            Matrix3::from_cols(
                [-0.911004, 0.408057, 0.059673].into(),
                [0.409913, 0.880133, 0.239450].into(),
                [0.045189, 0.242601, -0.969073].into(),
            ),
            Vector4::from([-0.0, 0.0, 0.0, 0.0]),
        ));

        assert_eq!(
            quick_xml::se::to_string(&root).unwrap(),
            "\
            <TestRoot>\
                (-0.911004 0.408057 0.059673)\
                (0.409913 0.880133 0.239450)\
                (0.045189 0.242601 -0.969073)\
                (-0.000000 0.000000 0.000000)\
            </TestRoot>"
        );
    }

    #[test]
    fn should_deserialize_qs_transform() {
        let xml = "
        <TestRoot>
            (0.000000 1.000000 0.000796)(0.899619 -0.000347 0.436676)(0.436676 0.000716 -0.899619)(-0.000016 -0.000017 19.995882)
        </TestRoot>";
        let deserialized: TestRoot<Transform<f32>> = quick_xml::de::from_str(xml).unwrap();

        let expected = Transform::from_cols(
            Matrix3::from_cols(
                [0.000000, 1.000000, 0.000796].into(),
                [0.899619, -0.000347, 0.436676].into(),
                [0.436676, 0.000716, -0.899619].into(),
            ),
            Vector4::from([-0.000016, -0.000017, 19.995882, 0.0]),
        );
        assert_eq!(deserialized, TestRoot(expected));
    }
}
