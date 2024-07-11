//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkxLight`
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

/// `hkxLight`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x81c86d42`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkxLight {
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub mem_size_and_flags: u16,
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub reference_count: i16,

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// # C++ Class Fields Info
    /// -   name:`"type"`
    /// -   type: `enum LightType`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub _type: LightType,
    /// # C++ Class Fields Info
    /// -   name:`"position"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub position: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"direction"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub direction: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"color"`
    /// -   type: `hkUint32`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub color: u32,
    /// # C++ Class Fields Info
    /// -   name:`"angle"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub angle: f32,
}

impl Serialize for HkxLight {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkxLightVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkxLight {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkxLightVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkxLightVisitor>> for HkxLight {
    fn from(_values: Vec<HkxLightVisitor>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut _type = None;
            let mut position = None;
            let mut direction = None;
            let mut color = None;
            let mut angle = None;


        for _value in _values {
            match _value {
                HkxLightVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkxLightVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkxLightVisitor::Type(m) => _type = Some(m),
                HkxLightVisitor::Position(m) => position = Some(m),
                HkxLightVisitor::Direction(m) => direction = Some(m),
                HkxLightVisitor::Color(m) => color = Some(m),
                HkxLightVisitor::Angle(m) => angle = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            _type: _type.unwrap_or_default().into_inner(),
            position: position.unwrap_or_default().into_inner(),
            direction: direction.unwrap_or_default().into_inner(),
            color: color.unwrap_or_default().into_inner(),
            angle: angle.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkxLight> for Vec<HkxLightVisitor> {
    fn from(data: &HkxLight) -> Self {
        vec![
            HkxLightVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkxLightVisitor::ReferenceCount(data.reference_count.into()),
            HkxLightVisitor::Type(data._type.clone().into()),
            HkxLightVisitor::Position(data.position.into()),
            HkxLightVisitor::Direction(data.direction.into()),
            HkxLightVisitor::Color(data.color.into()),
            HkxLightVisitor::Angle(data.angle.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkxLight {
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
enum HkxLightVisitor {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "type")]
    Type(Primitive<LightType>),
    /// Visitor fields
    #[serde(rename = "position")]
    Position(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "direction")]
    Direction(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "color")]
    Color(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "angle")]
    Angle(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxLightVisitor, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("type" => Type(Primitive<LightType>)),
    ("position" => Position(Primitive<Vector4<f32>>)),
    ("direction" => Direction(Primitive<Vector4<f32>>)),
    ("color" => Color(Primitive<u32>)),
    ("angle" => Angle(Primitive<f32>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum LightType {
    #[serde(rename = "POINT_LIGHT")]
    #[default]
    PointLight = 0,
    #[serde(rename = "DIRECTIONAL_LIGHT")]
    DirectionalLight = 1,
    #[serde(rename = "SPOT_LIGHT")]
    SpotLight = 2,
}
