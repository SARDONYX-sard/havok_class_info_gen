//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkSweptTransform`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#![allow(
  clippy::clone_on_copy,
  clippy::unit_arg
)]

#[allow(unused)]
use super::*;
#[allow(unused)]
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkSweptTransform`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 80
/// -    vtable: false
/// - signature: `0xb4e5770`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkSweptTransform {
    /// # C++ Class Fields Info
    /// -   name:`"centerOfMass0"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub center_of_mass_0: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"centerOfMass1"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub center_of_mass_1: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"rotation0"`
    /// -   type: `hkQuaternion`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub rotation_0: Quaternion<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"rotation1"`
    /// -   type: `hkQuaternion`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub rotation_1: Quaternion<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"centerOfMassLocal"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub center_of_mass_local: Vector4<f32>,
}

impl Serialize for HkSweptTransform {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkSweptTransformVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkSweptTransform {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkSweptTransformVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkSweptTransformVisitor>> for HkSweptTransform {
    fn from(_values: Vec<HkSweptTransformVisitor>) -> Self {
            let mut center_of_mass_0 = None;
            let mut center_of_mass_1 = None;
            let mut rotation_0 = None;
            let mut rotation_1 = None;
            let mut center_of_mass_local = None;


        for _value in _values {
            match _value {
                HkSweptTransformVisitor::CenterOfMass0(m) => center_of_mass_0 = Some(m),
                HkSweptTransformVisitor::CenterOfMass1(m) => center_of_mass_1 = Some(m),
                HkSweptTransformVisitor::Rotation0(m) => rotation_0 = Some(m),
                HkSweptTransformVisitor::Rotation1(m) => rotation_1 = Some(m),
                HkSweptTransformVisitor::CenterOfMassLocal(m) => center_of_mass_local = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            center_of_mass_0: center_of_mass_0.unwrap_or_default().into_inner(),
            center_of_mass_1: center_of_mass_1.unwrap_or_default().into_inner(),
            rotation_0: rotation_0.unwrap_or_default().into_inner(),
            rotation_1: rotation_1.unwrap_or_default().into_inner(),
            center_of_mass_local: center_of_mass_local.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkSweptTransform> for Vec<HkSweptTransformVisitor> {
    fn from(data: &HkSweptTransform) -> Self {
        vec![
            HkSweptTransformVisitor::CenterOfMass0(data.center_of_mass_0.into()),
            HkSweptTransformVisitor::CenterOfMass1(data.center_of_mass_1.into()),
            HkSweptTransformVisitor::Rotation0(data.rotation_0.clone().into()),
            HkSweptTransformVisitor::Rotation1(data.rotation_1.clone().into()),
            HkSweptTransformVisitor::CenterOfMassLocal(data.center_of_mass_local.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkSweptTransform {
    fn from_bytes<B>(
        _bytes: &'bytes [u8],
        _de: &mut PackFileDeserializer,
    ) -> Result<Self>
    where
        B: ByteOrder,
        Self: Sized + 'de
    {
        todo!()
    }
}


/// # Why use Visitor pattern?
/// Since the C++ field must be deserialized from the `name` attribute name of the `hkparam` in the XML,
/// this is accomplished by having the Visitor process the internally tagged enum and convert it.
/// Leakage of field items may occur if Vec<enum> is left as it is.
///
/// struct -> (De)serialize by visitor -> struct
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
enum HkSweptTransformVisitor {
    /// Visitor fields
    #[serde(rename = "centerOfMass0")]
    CenterOfMass0(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "centerOfMass1")]
    CenterOfMass1(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "rotation0")]
    Rotation0(Primitive<Quaternion<f32>>),
    /// Visitor fields
    #[serde(rename = "rotation1")]
    Rotation1(Primitive<Quaternion<f32>>),
    /// Visitor fields
    #[serde(rename = "centerOfMassLocal")]
    CenterOfMassLocal(Primitive<Vector4<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkSweptTransformVisitor, "@name",
    ("centerOfMass0" => CenterOfMass0(Primitive<Vector4<f32>>)),
    ("centerOfMass1" => CenterOfMass1(Primitive<Vector4<f32>>)),
    ("rotation0" => Rotation0(Primitive<Quaternion<f32>>)),
    ("rotation1" => Rotation1(Primitive<Quaternion<f32>>)),
    ("centerOfMassLocal" => CenterOfMassLocal(Primitive<Vector4<f32>>)),
}
