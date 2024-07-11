//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbCharacterDataCharacterControllerInfo`
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

/// `hkbCharacterDataCharacterControllerInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// - signature: `0xa0f415bf`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbCharacterDataCharacterControllerInfo<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"capsuleHeight"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub capsule_height: f32,
    /// # C++ Class Fields Info
    /// -   name:`"capsuleRadius"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub capsule_radius: f32,
    /// # C++ Class Fields Info
    /// -   name:`"collisionFilterInfo"`
    /// -   type: `hkUint32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub collision_filter_info: u32,
    /// # C++ Class Fields Info
    /// -   name:`"characterControllerCinfo"`
    /// -   type: `struct hkpCharacterControllerCinfo*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub character_controller_cinfo: Cow<'a, str>,
}

impl Serialize for HkbCharacterDataCharacterControllerInfo<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbCharacterDataCharacterControllerInfoVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbCharacterDataCharacterControllerInfo<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbCharacterDataCharacterControllerInfoVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbCharacterDataCharacterControllerInfoVisitor<'a>>> for HkbCharacterDataCharacterControllerInfo<'a> {
    fn from(_values: Vec<HkbCharacterDataCharacterControllerInfoVisitor<'a>>) -> Self {
            let mut capsule_height = None;
            let mut capsule_radius = None;
            let mut collision_filter_info = None;
            let mut character_controller_cinfo = None;


        for _value in _values {
            match _value {
                HkbCharacterDataCharacterControllerInfoVisitor::CapsuleHeight(m) => capsule_height = Some(m),
                HkbCharacterDataCharacterControllerInfoVisitor::CapsuleRadius(m) => capsule_radius = Some(m),
                HkbCharacterDataCharacterControllerInfoVisitor::CollisionFilterInfo(m) => collision_filter_info = Some(m),
                HkbCharacterDataCharacterControllerInfoVisitor::CharacterControllerCinfo(m) => character_controller_cinfo = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            capsule_height: capsule_height.unwrap_or_default().into_inner(),
            capsule_radius: capsule_radius.unwrap_or_default().into_inner(),
            collision_filter_info: collision_filter_info.unwrap_or_default().into_inner(),
            character_controller_cinfo: character_controller_cinfo.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbCharacterDataCharacterControllerInfo<'a>> for Vec<HkbCharacterDataCharacterControllerInfoVisitor<'a>> {
    fn from(data: &HkbCharacterDataCharacterControllerInfo<'a>) -> Self {
        vec![
            HkbCharacterDataCharacterControllerInfoVisitor::CapsuleHeight(data.capsule_height.into()),
            HkbCharacterDataCharacterControllerInfoVisitor::CapsuleRadius(data.capsule_radius.into()),
            HkbCharacterDataCharacterControllerInfoVisitor::CollisionFilterInfo(data.collision_filter_info.into()),
            HkbCharacterDataCharacterControllerInfoVisitor::CharacterControllerCinfo(data.character_controller_cinfo.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbCharacterDataCharacterControllerInfo<'de> {
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
enum HkbCharacterDataCharacterControllerInfoVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "capsuleHeight")]
    CapsuleHeight(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "capsuleRadius")]
    CapsuleRadius(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "collisionFilterInfo")]
    CollisionFilterInfo(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "characterControllerCinfo")]
    CharacterControllerCinfo(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbCharacterDataCharacterControllerInfoVisitor<'de>, "@name",
    ("capsuleHeight" => CapsuleHeight(Primitive<f32>)),
    ("capsuleRadius" => CapsuleRadius(Primitive<f32>)),
    ("collisionFilterInfo" => CollisionFilterInfo(Primitive<u32>)),
    ("characterControllerCinfo" => CharacterControllerCinfo(Primitive<Cow<'de, str>>)),
}
