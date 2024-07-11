//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpSpringAction`
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

/// `hkpSpringAction`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 96
/// -    vtable: true
/// -    parent: `hkpBinaryAction`/`0xc00f3403`
/// - signature: `0x88fc09fa`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpSpringAction<'a> {
    /// # C++ Parent class(`hkpBinaryAction` => parent: `hkpAction`) field Info
    /// -   name:`"entityA"`
    /// -   type: `struct hkpEntity*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub entity_a: Cow<'a, str>,
    /// # C++ Parent class(`hkpBinaryAction` => parent: `hkpAction`) field Info
    /// -   name:`"entityB"`
    /// -   type: `struct hkpEntity*`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub entity_b: Cow<'a, str>,

    /// # C++ Parent class(`hkpAction` => parent: `hkReferencedObject`) field Info
    /// -   name:`"world"`
    /// -   type: `void*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub world: Cow<'a, str>,
    /// # C++ Parent class(`hkpAction` => parent: `hkReferencedObject`) field Info
    /// -   name:`"island"`
    /// -   type: `void*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub island: Cow<'a, str>,
    /// # C++ Parent class(`hkpAction` => parent: `hkReferencedObject`) field Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub user_data: usize,
    /// # C++ Parent class(`hkpAction` => parent: `hkReferencedObject`) field Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub name: Cow<'a, str>,

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
    /// -   name:`"lastForce"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub last_force: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"positionAinA"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub position_ain_a: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"positionBinB"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub position_bin_b: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"restLength"`
    /// -   type: `hkReal`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub rest_length: f32,
    /// # C++ Class Fields Info
    /// -   name:`"strength"`
    /// -   type: `hkReal`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    pub strength: f32,
    /// # C++ Class Fields Info
    /// -   name:`"damping"`
    /// -   type: `hkReal`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    pub damping: f32,
    /// # C++ Class Fields Info
    /// -   name:`"onCompression"`
    /// -   type: `hkBool`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    pub on_compression: bool,
    /// # C++ Class Fields Info
    /// -   name:`"onExtension"`
    /// -   type: `hkBool`
    /// - offset: 93
    /// -  flags: `FLAGS_NONE`
    pub on_extension: bool,
}

impl Serialize for HkpSpringAction<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpSpringActionVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpSpringAction<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpSpringActionVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpSpringActionVisitor<'a>>> for HkpSpringAction<'a> {
    fn from(_values: Vec<HkpSpringActionVisitor<'a>>) -> Self {
            let mut entity_a = None;
            let mut entity_b = None;
            let mut world = None;
            let mut island = None;
            let mut user_data = None;
            let mut name = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut last_force = None;
            let mut position_ain_a = None;
            let mut position_bin_b = None;
            let mut rest_length = None;
            let mut strength = None;
            let mut damping = None;
            let mut on_compression = None;
            let mut on_extension = None;


        for _value in _values {
            match _value {
                HkpSpringActionVisitor::EntityA(m) => entity_a = Some(m),
                HkpSpringActionVisitor::EntityB(m) => entity_b = Some(m),
                HkpSpringActionVisitor::World(m) => world = Some(m),
                HkpSpringActionVisitor::Island(m) => island = Some(m),
                HkpSpringActionVisitor::UserData(m) => user_data = Some(m),
                HkpSpringActionVisitor::Name(m) => name = Some(m),
                HkpSpringActionVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpSpringActionVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpSpringActionVisitor::LastForce(m) => last_force = Some(m),
                HkpSpringActionVisitor::PositionAinA(m) => position_ain_a = Some(m),
                HkpSpringActionVisitor::PositionBinB(m) => position_bin_b = Some(m),
                HkpSpringActionVisitor::RestLength(m) => rest_length = Some(m),
                HkpSpringActionVisitor::Strength(m) => strength = Some(m),
                HkpSpringActionVisitor::Damping(m) => damping = Some(m),
                HkpSpringActionVisitor::OnCompression(m) => on_compression = Some(m),
                HkpSpringActionVisitor::OnExtension(m) => on_extension = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            entity_a: entity_a.unwrap_or_default().into_inner(),
            entity_b: entity_b.unwrap_or_default().into_inner(),
            world: world.unwrap_or_default().into_inner(),
            island: island.unwrap_or_default().into_inner(),
            user_data: user_data.unwrap_or_default().into_inner(),
            name: name.unwrap_or_default().into_inner(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            last_force: last_force.unwrap_or_default().into_inner(),
            position_ain_a: position_ain_a.unwrap_or_default().into_inner(),
            position_bin_b: position_bin_b.unwrap_or_default().into_inner(),
            rest_length: rest_length.unwrap_or_default().into_inner(),
            strength: strength.unwrap_or_default().into_inner(),
            damping: damping.unwrap_or_default().into_inner(),
            on_compression: on_compression.unwrap_or_default().into_inner(),
            on_extension: on_extension.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpSpringAction<'a>> for Vec<HkpSpringActionVisitor<'a>> {
    fn from(data: &HkpSpringAction<'a>) -> Self {
        vec![
            HkpSpringActionVisitor::EntityA(data.entity_a.clone().into()),
            HkpSpringActionVisitor::EntityB(data.entity_b.clone().into()),
            HkpSpringActionVisitor::World(data.world.clone().into()),
            HkpSpringActionVisitor::Island(data.island.clone().into()),
            HkpSpringActionVisitor::UserData(data.user_data.into()),
            HkpSpringActionVisitor::Name(data.name.clone().into()),
            HkpSpringActionVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpSpringActionVisitor::ReferenceCount(data.reference_count.into()),
            HkpSpringActionVisitor::LastForce(data.last_force.into()),
            HkpSpringActionVisitor::PositionAinA(data.position_ain_a.into()),
            HkpSpringActionVisitor::PositionBinB(data.position_bin_b.into()),
            HkpSpringActionVisitor::RestLength(data.rest_length.into()),
            HkpSpringActionVisitor::Strength(data.strength.into()),
            HkpSpringActionVisitor::Damping(data.damping.into()),
            HkpSpringActionVisitor::OnCompression(data.on_compression.into()),
            HkpSpringActionVisitor::OnExtension(data.on_extension.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpSpringAction<'de> {
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
enum HkpSpringActionVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "entityA")]
    EntityA(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "entityB")]
    EntityB(Primitive<Cow<'a, str>>),

    /// Visitor fields
    #[serde(rename = "world", skip_serializing)]
    World(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "island", skip_serializing)]
    Island(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "userData")]
    UserData(Primitive<usize>),
    /// Visitor fields
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),

    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "lastForce")]
    LastForce(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "positionAinA")]
    PositionAinA(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "positionBinB")]
    PositionBinB(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "restLength")]
    RestLength(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "strength")]
    Strength(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "damping")]
    Damping(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "onCompression")]
    OnCompression(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "onExtension")]
    OnExtension(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpSpringActionVisitor<'de>, "@name",
    ("entityA" => EntityA(Primitive<Cow<'de, str>>)),
    ("entityB" => EntityB(Primitive<Cow<'de, str>>)),
    ("world" => World(Primitive<Cow<'de, str>>)),
    ("island" => Island(Primitive<Cow<'de, str>>)),
    ("userData" => UserData(Primitive<usize>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("lastForce" => LastForce(Primitive<Vector4<f32>>)),
    ("positionAinA" => PositionAinA(Primitive<Vector4<f32>>)),
    ("positionBinB" => PositionBinB(Primitive<Vector4<f32>>)),
    ("restLength" => RestLength(Primitive<f32>)),
    ("strength" => Strength(Primitive<f32>)),
    ("damping" => Damping(Primitive<f32>)),
    ("onCompression" => OnCompression(Primitive<bool>)),
    ("onExtension" => OnExtension(Primitive<bool>)),
}
