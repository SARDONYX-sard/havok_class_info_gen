//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpReorientAction`
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

/// `hkpReorientAction`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 80
/// -    vtable: true
/// -    parent: `hkpUnaryAction`/`0x895532c0`
/// - signature: `0x2dc0ec6a`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpReorientAction<'a> {
    /// # C++ Parent class(`hkpUnaryAction` => parent: `hkpAction`) field Info
    /// -   name:`"entity"`
    /// -   type: `struct hkpEntity*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub entity: Cow<'a, str>,

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
    /// -   name:`"rotationAxis"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub rotation_axis: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"upAxis"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub up_axis: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"strength"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub strength: f32,
    /// # C++ Class Fields Info
    /// -   name:`"damping"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    pub damping: f32,
}

impl Serialize for HkpReorientAction<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpReorientActionVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpReorientAction<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpReorientActionVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpReorientActionVisitor<'a>>> for HkpReorientAction<'a> {
    fn from(_values: Vec<HkpReorientActionVisitor<'a>>) -> Self {
            let mut entity = None;
            let mut world = None;
            let mut island = None;
            let mut user_data = None;
            let mut name = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut rotation_axis = None;
            let mut up_axis = None;
            let mut strength = None;
            let mut damping = None;


        for _value in _values {
            match _value {
                HkpReorientActionVisitor::Entity(m) => entity = Some(m),
                HkpReorientActionVisitor::World(m) => world = Some(m),
                HkpReorientActionVisitor::Island(m) => island = Some(m),
                HkpReorientActionVisitor::UserData(m) => user_data = Some(m),
                HkpReorientActionVisitor::Name(m) => name = Some(m),
                HkpReorientActionVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpReorientActionVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpReorientActionVisitor::RotationAxis(m) => rotation_axis = Some(m),
                HkpReorientActionVisitor::UpAxis(m) => up_axis = Some(m),
                HkpReorientActionVisitor::Strength(m) => strength = Some(m),
                HkpReorientActionVisitor::Damping(m) => damping = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            entity: entity.unwrap_or_default().into_inner(),
            world: world.unwrap_or_default().into_inner(),
            island: island.unwrap_or_default().into_inner(),
            user_data: user_data.unwrap_or_default().into_inner(),
            name: name.unwrap_or_default().into_inner(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            rotation_axis: rotation_axis.unwrap_or_default().into_inner(),
            up_axis: up_axis.unwrap_or_default().into_inner(),
            strength: strength.unwrap_or_default().into_inner(),
            damping: damping.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpReorientAction<'a>> for Vec<HkpReorientActionVisitor<'a>> {
    fn from(data: &HkpReorientAction<'a>) -> Self {
        vec![
            HkpReorientActionVisitor::Entity(data.entity.clone().into()),
            HkpReorientActionVisitor::World(data.world.clone().into()),
            HkpReorientActionVisitor::Island(data.island.clone().into()),
            HkpReorientActionVisitor::UserData(data.user_data.into()),
            HkpReorientActionVisitor::Name(data.name.clone().into()),
            HkpReorientActionVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpReorientActionVisitor::ReferenceCount(data.reference_count.into()),
            HkpReorientActionVisitor::RotationAxis(data.rotation_axis.into()),
            HkpReorientActionVisitor::UpAxis(data.up_axis.into()),
            HkpReorientActionVisitor::Strength(data.strength.into()),
            HkpReorientActionVisitor::Damping(data.damping.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpReorientAction<'de> {
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
enum HkpReorientActionVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "entity")]
    Entity(Primitive<Cow<'a, str>>),

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
    #[serde(rename = "rotationAxis")]
    RotationAxis(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "upAxis")]
    UpAxis(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "strength")]
    Strength(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "damping")]
    Damping(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpReorientActionVisitor<'de>, "@name",
    ("entity" => Entity(Primitive<Cow<'de, str>>)),
    ("world" => World(Primitive<Cow<'de, str>>)),
    ("island" => Island(Primitive<Cow<'de, str>>)),
    ("userData" => UserData(Primitive<usize>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("rotationAxis" => RotationAxis(Primitive<Vector4<f32>>)),
    ("upAxis" => UpAxis(Primitive<Vector4<f32>>)),
    ("strength" => Strength(Primitive<f32>)),
    ("damping" => Damping(Primitive<f32>)),
}
