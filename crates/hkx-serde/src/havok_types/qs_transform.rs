use super::{Quaternion, Vector3, Vector4};
use core::{fmt, str::FromStr};
use ordered_float::FloatCore;
use serde::{de::IntoDeserializer, Deserialize, Serialize, Serializer};

/// `QsTransform` with (De)serialization for havok.
///
/// In XML, it would be as follows.
/// ```xml
/// <!-- < - vector4 transition - - >< - - - - quaternion rotation - - - - > < - vector4 scale - - - - >    -->
/// <tag>(0.000000 0.000000 0.000000)(-0.000000 0.000000 -0.000000 1.000000)(1.000000 1.000000 1.000000)</tag>
/// ```
///
/// # Note
/// The `transition` and `scale` types are [`Vector4`], but only three are actually used, so [`Vector4::w`] is always 0.0.
///
/// This is probably to facilitate affine transformations(Calculation to change the coordinates by multiplying the coordinates by a matrix).
#[repr(C)]
#[derive(Debug, PartialEq, Default, Eq, Copy, Clone, Hash)]
pub struct QsTransform<S: FloatCore> {
    /// Representing transitions.
    ///
    /// # Note
    /// `Vector4::w`(4th) isn't used.
    pub transition: Vector4<S>,
    /// Representing rotation.
    pub rotation: Quaternion<S>,
    /// Representing scale.
    ///
    /// # Note
    /// `Vector4::w`(4th) isn't used.
    pub scale: Vector4<S>,
}

impl<S: FloatCore> QsTransform<S> {
    /// Create a new matrix, providing values for each index.
    #[allow(clippy::too_many_arguments)]
    #[inline]
    #[rustfmt::skip]
    pub const fn new(
        c0r0: S, c0r1: S, c0r2: S, c0r3: S,
        c1r0: S, c1r1: S, c1r2: S, c1r3: S,
        c2r0: S, c2r1: S, c2r2: S, c2r3: S,
    ) -> QsTransform<S>  {
        QsTransform::from_cols(
            Vector4::new(c0r0, c0r1, c0r2, c0r3),
            Quaternion::new(c1r0, c1r1, c1r2, c1r3),
            Vector4::new(c2r0, c2r1, c2r2, c2r3),
        )
    }

    /// Create a new matrix, providing columns.
    #[inline]
    pub const fn from_cols(c0: Vector4<S>, c1: Quaternion<S>, c2: Vector4<S>) -> QsTransform<S> {
        QsTransform {
            transition: c0,
            rotation: c1,
            scale: c2,
        }
    }
}

//# Attention
// Since [`Serialize`] of [`Matrix4`] is implemented manually and [`to_string`] is called inside it, the format of [`Display`]trait is reflected in the string format of [`Serialize`].
impl<S: fmt::Display + FloatCore> fmt::Display for QsTransform<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let t: Vector3<S> = self.transition.into();
        let r = self.rotation;
        let s: Vector3<S> = self.scale.into();
        write!(f, "{t}{r}{s}",)
    }
}

impl<T: FloatCore> Serialize for QsTransform<T>
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

impl<'de, T> Deserialize<'de> for QsTransform<T>
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
            type Value = QsTransform<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string representing a Matrix4")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                // Vector4 is enclosed in `()` in XML, so extract an array of Vector4 separated by `)`.
                let parts: Vec<_> = s.split(')').filter(|s| !s.is_empty()).collect();
                let parts_len = parts.len();
                if parts_len < 3 {
                    let err_msg = format!("QsTransform is expected 3 Vector str. But got len: {parts_len} & content: {parts:?}");
                    return Err(serde::de::Error::custom(err_msg));
                }

                let mut iter = parts.into_iter();
                Ok(
                    //? Safety: It is safe to use [`unwrap`] because we confirmed above that four exist when [`str::split`].
                    QsTransform {
                        transition: Vector3::deserialize(iter.next().unwrap().into_deserializer())?
                            .into(),
                        rotation: Quaternion::deserialize(
                            iter.next().unwrap().into_deserializer(),
                        )?,
                        scale: Vector3::deserialize(iter.next().unwrap().into_deserializer())?
                            .into(),
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
        let root = TestRoot(QsTransform::from_cols(
            Vector4::from([0.000000, 0.000000, f32::NEG_INFINITY, 0.000000]),
            Quaternion::from([f32::NAN, 0.000000, f32::NEG_INFINITY, 0.000000]),
            Vector4::from([f32::NAN, 0.000000, f32::NEG_INFINITY, 0.000000]),
        ));

        assert_eq!(
            quick_xml::se::to_string(&root).unwrap(),
            "\
            <TestRoot>\
                (0.000000 0.000000 0.000000)(-0.000000 0.000000 -0.000000 1.000000)(1.000000 1.000000 1.000000)\
            </TestRoot>"
        );
    }

    #[test]
    fn should_deserialize_qs_transform() {
        let xml = "
        <TestRoot>
            (0.000000 0.000000 0.000000)(-0.000000 0.000000 -0.000000 1.000000)(1.000000 1.000000 1.000000)
        </TestRoot>";
        let deserialized: TestRoot<QsTransform<f32>> = quick_xml::de::from_str(xml).unwrap();

        let expected_qstransform = QsTransform::from_cols(
            Vector4::from([0.0, 0.0, 0.0, 0.0]),
            Quaternion::from([-0.0, 0.0, -0.0, 1.0]),
            Vector4::from([1.0, 1.0, 1.0, 0.0]),
        );
        assert_eq!(deserialized, TestRoot(expected_qstransform));
    }
}
