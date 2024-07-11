//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbParticleSystemEventPayload`
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

/// `hkbParticleSystemEventPayload`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: true
/// -    parent: `hkbEventPayload`/`0xda8c7d7d`
/// - signature: `0x9df46cd6`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbParticleSystemEventPayload {
    // C++ Parent class(`hkbEventPayload` => parent: `hkReferencedObject`) has no fields
    //
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
    /// -   type: `enum SystemType`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub _type: SystemType,
    /// # C++ Class Fields Info
    /// -   name:`"emitBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 10
    /// -  flags: `FLAGS_NONE`
    pub emit_bone_index: i16,
    /// # C++ Class Fields Info
    /// -   name:`"offset"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub offset: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"direction"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub direction: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"numParticles"`
    /// -   type: `hkInt32`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub num_particles: i32,
    /// # C++ Class Fields Info
    /// -   name:`"speed"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub speed: f32,
}

impl Serialize for HkbParticleSystemEventPayload {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbParticleSystemEventPayloadVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbParticleSystemEventPayload {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbParticleSystemEventPayloadVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkbParticleSystemEventPayloadVisitor>> for HkbParticleSystemEventPayload {
    fn from(_values: Vec<HkbParticleSystemEventPayloadVisitor>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut _type = None;
            let mut emit_bone_index = None;
            let mut offset = None;
            let mut direction = None;
            let mut num_particles = None;
            let mut speed = None;


        for _value in _values {
            match _value {
                HkbParticleSystemEventPayloadVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbParticleSystemEventPayloadVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbParticleSystemEventPayloadVisitor::Type(m) => _type = Some(m),
                HkbParticleSystemEventPayloadVisitor::EmitBoneIndex(m) => emit_bone_index = Some(m),
                HkbParticleSystemEventPayloadVisitor::Offset(m) => offset = Some(m),
                HkbParticleSystemEventPayloadVisitor::Direction(m) => direction = Some(m),
                HkbParticleSystemEventPayloadVisitor::NumParticles(m) => num_particles = Some(m),
                HkbParticleSystemEventPayloadVisitor::Speed(m) => speed = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            _type: _type.unwrap_or_default().into_inner(),
            emit_bone_index: emit_bone_index.unwrap_or_default().into_inner(),
            offset: offset.unwrap_or_default().into_inner(),
            direction: direction.unwrap_or_default().into_inner(),
            num_particles: num_particles.unwrap_or_default().into_inner(),
            speed: speed.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkbParticleSystemEventPayload> for Vec<HkbParticleSystemEventPayloadVisitor> {
    fn from(data: &HkbParticleSystemEventPayload) -> Self {
        vec![
            HkbParticleSystemEventPayloadVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbParticleSystemEventPayloadVisitor::ReferenceCount(data.reference_count.into()),
            HkbParticleSystemEventPayloadVisitor::Type(data._type.clone().into()),
            HkbParticleSystemEventPayloadVisitor::EmitBoneIndex(data.emit_bone_index.into()),
            HkbParticleSystemEventPayloadVisitor::Offset(data.offset.into()),
            HkbParticleSystemEventPayloadVisitor::Direction(data.direction.into()),
            HkbParticleSystemEventPayloadVisitor::NumParticles(data.num_particles.into()),
            HkbParticleSystemEventPayloadVisitor::Speed(data.speed.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbParticleSystemEventPayload {
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
enum HkbParticleSystemEventPayloadVisitor {
    // C++ Parent class(`hkbEventPayload` => parent: `hkReferencedObject`) has no fields
    //
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
    Type(Primitive<SystemType>),
    /// Visitor fields
    #[serde(rename = "emitBoneIndex")]
    EmitBoneIndex(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "offset")]
    Offset(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "direction")]
    Direction(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "numParticles")]
    NumParticles(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "speed")]
    Speed(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbParticleSystemEventPayloadVisitor, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("type" => Type(Primitive<SystemType>)),
    ("emitBoneIndex" => EmitBoneIndex(Primitive<i16>)),
    ("offset" => Offset(Primitive<Vector4<f32>>)),
    ("direction" => Direction(Primitive<Vector4<f32>>)),
    ("numParticles" => NumParticles(Primitive<i32>)),
    ("speed" => Speed(Primitive<f32>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum SystemType {
    #[serde(rename = "DEBRIS")]
    #[default]
    Debris = 0,
    #[serde(rename = "DUST")]
    Dust = 1,
    #[serde(rename = "EXPLOSION")]
    Explosion = 2,
    #[serde(rename = "SMOKE")]
    Smoke = 3,
    #[serde(rename = "SPARKS")]
    Sparks = 4,
}
