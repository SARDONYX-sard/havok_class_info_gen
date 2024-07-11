//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbHandle`
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

/// `hkbHandle`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 24
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xd8b6401c`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbHandle<'a> {
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
    /// -   name:`"frame"`
    /// -   type: `struct hkLocalFrame*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub frame: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"rigidBody"`
    /// -   type: `struct hkpRigidBody*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub rigid_body: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"character"`
    /// -   type: `struct hkbCharacter*`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub character: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"animationBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub animation_bone_index: i16,
}

impl Serialize for HkbHandle<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbHandleVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbHandle<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbHandleVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbHandleVisitor<'a>>> for HkbHandle<'a> {
    fn from(_values: Vec<HkbHandleVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut frame = None;
            let mut rigid_body = None;
            let mut character = None;
            let mut animation_bone_index = None;


        for _value in _values {
            match _value {
                HkbHandleVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbHandleVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbHandleVisitor::Frame(m) => frame = Some(m),
                HkbHandleVisitor::RigidBody(m) => rigid_body = Some(m),
                HkbHandleVisitor::Character(m) => character = Some(m),
                HkbHandleVisitor::AnimationBoneIndex(m) => animation_bone_index = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            frame: frame.unwrap_or_default().into_inner(),
            rigid_body: rigid_body.unwrap_or_default().into_inner(),
            character: character.unwrap_or_default().into_inner(),
            animation_bone_index: animation_bone_index.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbHandle<'a>> for Vec<HkbHandleVisitor<'a>> {
    fn from(data: &HkbHandle<'a>) -> Self {
        vec![
            HkbHandleVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbHandleVisitor::ReferenceCount(data.reference_count.into()),
            HkbHandleVisitor::Frame(data.frame.clone().into()),
            HkbHandleVisitor::RigidBody(data.rigid_body.clone().into()),
            HkbHandleVisitor::Character(data.character.clone().into()),
            HkbHandleVisitor::AnimationBoneIndex(data.animation_bone_index.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbHandle<'de> {
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
enum HkbHandleVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "frame")]
    Frame(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "rigidBody")]
    RigidBody(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "character")]
    Character(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "animationBoneIndex")]
    AnimationBoneIndex(Primitive<i16>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbHandleVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("frame" => Frame(Primitive<Cow<'de, str>>)),
    ("rigidBody" => RigidBody(Primitive<Cow<'de, str>>)),
    ("character" => Character(Primitive<Cow<'de, str>>)),
    ("animationBoneIndex" => AnimationBoneIndex(Primitive<i16>)),
}
