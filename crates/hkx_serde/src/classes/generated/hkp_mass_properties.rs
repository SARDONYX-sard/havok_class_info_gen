//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpMassProperties`
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

/// `hkpMassProperties`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 80
/// -    vtable: false
/// - signature: `0x68a56834`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpMassProperties {
    /// # C++ Class Fields Info
    /// -   name:`"volume"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub volume: f32,
    /// # C++ Class Fields Info
    /// -   name:`"mass"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub mass: f32,
    /// # C++ Class Fields Info
    /// -   name:`"centerOfMass"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub center_of_mass: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"inertiaTensor"`
    /// -   type: `hkMatrix3`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub inertia_tensor: Matrix3<f32>,
}

impl Serialize for HkpMassProperties {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpMassPropertiesVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpMassProperties {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpMassPropertiesVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpMassPropertiesVisitor>> for HkpMassProperties {
    fn from(_values: Vec<HkpMassPropertiesVisitor>) -> Self {
            let mut volume = None;
            let mut mass = None;
            let mut center_of_mass = None;
            let mut inertia_tensor = None;


        for _value in _values {
            match _value {
                HkpMassPropertiesVisitor::Volume(m) => volume = Some(m),
                HkpMassPropertiesVisitor::Mass(m) => mass = Some(m),
                HkpMassPropertiesVisitor::CenterOfMass(m) => center_of_mass = Some(m),
                HkpMassPropertiesVisitor::InertiaTensor(m) => inertia_tensor = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            volume: volume.unwrap_or_default().into_inner(),
            mass: mass.unwrap_or_default().into_inner(),
            center_of_mass: center_of_mass.unwrap_or_default().into_inner(),
            inertia_tensor: inertia_tensor.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpMassProperties> for Vec<HkpMassPropertiesVisitor> {
    fn from(data: &HkpMassProperties) -> Self {
        vec![
            HkpMassPropertiesVisitor::Volume(data.volume.into()),
            HkpMassPropertiesVisitor::Mass(data.mass.into()),
            HkpMassPropertiesVisitor::CenterOfMass(data.center_of_mass.into()),
            HkpMassPropertiesVisitor::InertiaTensor(data.inertia_tensor.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpMassProperties {
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
enum HkpMassPropertiesVisitor {
    /// Visitor fields
    #[serde(rename = "volume")]
    Volume(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "mass")]
    Mass(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "centerOfMass")]
    CenterOfMass(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "inertiaTensor")]
    InertiaTensor(Primitive<Matrix3<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpMassPropertiesVisitor, "@name",
    ("volume" => Volume(Primitive<f32>)),
    ("mass" => Mass(Primitive<f32>)),
    ("centerOfMass" => CenterOfMass(Primitive<Vector4<f32>>)),
    ("inertiaTensor" => InertiaTensor(Primitive<Matrix3<f32>>)),
}
