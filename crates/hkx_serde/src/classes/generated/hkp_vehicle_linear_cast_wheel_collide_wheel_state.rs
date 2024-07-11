//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleLinearCastWheelCollideWheelState`
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

/// `hkpVehicleLinearCastWheelCollideWheelState`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 96
/// -    vtable: false
/// - signature: `0x2a9acf98`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpVehicleLinearCastWheelCollideWheelState<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"phantom"`
    /// -   type: `struct hkpAabbPhantom*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub phantom: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"shape"`
    /// -   type: `struct hkpShape*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub shape: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"transform"`
    /// -   type: `hkTransform`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub transform: Transform<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"to"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub to: Vector4<f32>,
}

impl Serialize for HkpVehicleLinearCastWheelCollideWheelState<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpVehicleLinearCastWheelCollideWheelStateVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpVehicleLinearCastWheelCollideWheelState<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpVehicleLinearCastWheelCollideWheelStateVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpVehicleLinearCastWheelCollideWheelStateVisitor<'a>>> for HkpVehicleLinearCastWheelCollideWheelState<'a> {
    fn from(_values: Vec<HkpVehicleLinearCastWheelCollideWheelStateVisitor<'a>>) -> Self {
            let mut phantom = None;
            let mut shape = None;
            let mut transform = None;
            let mut to = None;


        for _value in _values {
            match _value {
                HkpVehicleLinearCastWheelCollideWheelStateVisitor::Phantom(m) => phantom = Some(m),
                HkpVehicleLinearCastWheelCollideWheelStateVisitor::Shape(m) => shape = Some(m),
                HkpVehicleLinearCastWheelCollideWheelStateVisitor::Transform(m) => transform = Some(m),
                HkpVehicleLinearCastWheelCollideWheelStateVisitor::To(m) => to = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            phantom: phantom.unwrap_or_default().into_inner(),
            shape: shape.unwrap_or_default().into_inner(),
            transform: transform.unwrap_or_default().into_inner(),
            to: to.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpVehicleLinearCastWheelCollideWheelState<'a>> for Vec<HkpVehicleLinearCastWheelCollideWheelStateVisitor<'a>> {
    fn from(data: &HkpVehicleLinearCastWheelCollideWheelState<'a>) -> Self {
        vec![
            HkpVehicleLinearCastWheelCollideWheelStateVisitor::Phantom(data.phantom.clone().into()),
            HkpVehicleLinearCastWheelCollideWheelStateVisitor::Shape(data.shape.clone().into()),
            HkpVehicleLinearCastWheelCollideWheelStateVisitor::Transform(data.transform.clone().into()),
            HkpVehicleLinearCastWheelCollideWheelStateVisitor::To(data.to.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpVehicleLinearCastWheelCollideWheelState<'de> {
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
enum HkpVehicleLinearCastWheelCollideWheelStateVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "phantom")]
    Phantom(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "shape")]
    Shape(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "transform")]
    Transform(Primitive<Transform<f32>>),
    /// Visitor fields
    #[serde(rename = "to")]
    To(Primitive<Vector4<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleLinearCastWheelCollideWheelStateVisitor<'de>, "@name",
    ("phantom" => Phantom(Primitive<Cow<'de, str>>)),
    ("shape" => Shape(Primitive<Cow<'de, str>>)),
    ("transform" => Transform(Primitive<Transform<f32>>)),
    ("to" => To(Primitive<Vector4<f32>>)),
}
