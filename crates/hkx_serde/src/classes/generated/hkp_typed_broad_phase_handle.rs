//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpTypedBroadPhaseHandle`
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

/// `hkpTypedBroadPhaseHandle`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// -    parent: `hkpBroadPhaseHandle`/`0x940569dc`
/// - signature: `0xf4b0f799`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpTypedBroadPhaseHandle {
    /// # C++ Parent class(`hkpBroadPhaseHandle` => parent: `None`) field Info
    /// -   name:`"id"`
    /// -   type: `hkUint32`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub id: u32,

    /// # C++ Class Fields Info
    /// -   name:`"type"`
    /// -   type: `hkInt8`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub _type: i8,
    /// # C++ Class Fields Info
    /// -   name:`"ownerOffset"`
    /// -   type: `hkInt8`
    /// - offset: 5
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub owner_offset: i8,
    /// # C++ Class Fields Info
    /// -   name:`"objectQualityType"`
    /// -   type: `hkInt8`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE`
    pub object_quality_type: i8,
    /// # C++ Class Fields Info
    /// -   name:`"collisionFilterInfo"`
    /// -   type: `hkUint32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub collision_filter_info: u32,
}

impl Serialize for HkpTypedBroadPhaseHandle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpTypedBroadPhaseHandleVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpTypedBroadPhaseHandle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpTypedBroadPhaseHandleVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpTypedBroadPhaseHandleVisitor>> for HkpTypedBroadPhaseHandle {
    fn from(_values: Vec<HkpTypedBroadPhaseHandleVisitor>) -> Self {
            let mut id = None;
            let mut _type = None;
            let mut owner_offset = None;
            let mut object_quality_type = None;
            let mut collision_filter_info = None;


        for _value in _values {
            match _value {
                HkpTypedBroadPhaseHandleVisitor::Id(m) => id = Some(m),
                HkpTypedBroadPhaseHandleVisitor::Type(m) => _type = Some(m),
                HkpTypedBroadPhaseHandleVisitor::OwnerOffset(m) => owner_offset = Some(m),
                HkpTypedBroadPhaseHandleVisitor::ObjectQualityType(m) => object_quality_type = Some(m),
                HkpTypedBroadPhaseHandleVisitor::CollisionFilterInfo(m) => collision_filter_info = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            id: id.unwrap_or_default().into_inner(),
            _type: _type.unwrap_or_default().into_inner(),
            owner_offset: owner_offset.unwrap_or_default().into_inner(),
            object_quality_type: object_quality_type.unwrap_or_default().into_inner(),
            collision_filter_info: collision_filter_info.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpTypedBroadPhaseHandle> for Vec<HkpTypedBroadPhaseHandleVisitor> {
    fn from(data: &HkpTypedBroadPhaseHandle) -> Self {
        vec![
            HkpTypedBroadPhaseHandleVisitor::Id(data.id.into()),
            HkpTypedBroadPhaseHandleVisitor::Type(data._type.into()),
            HkpTypedBroadPhaseHandleVisitor::OwnerOffset(data.owner_offset.into()),
            HkpTypedBroadPhaseHandleVisitor::ObjectQualityType(data.object_quality_type.into()),
            HkpTypedBroadPhaseHandleVisitor::CollisionFilterInfo(data.collision_filter_info.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpTypedBroadPhaseHandle {
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
enum HkpTypedBroadPhaseHandleVisitor {
    /// Visitor fields
    #[serde(rename = "id", skip_serializing)]
    Id(Primitive<u32>),

    /// Visitor fields
    #[serde(rename = "type")]
    Type(Primitive<i8>),
    /// Visitor fields
    #[serde(rename = "ownerOffset", skip_serializing)]
    OwnerOffset(Primitive<i8>),
    /// Visitor fields
    #[serde(rename = "objectQualityType")]
    ObjectQualityType(Primitive<i8>),
    /// Visitor fields
    #[serde(rename = "collisionFilterInfo")]
    CollisionFilterInfo(Primitive<u32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpTypedBroadPhaseHandleVisitor, "@name",
    ("id" => Id(Primitive<u32>)),
    ("type" => Type(Primitive<i8>)),
    ("ownerOffset" => OwnerOffset(Primitive<i8>)),
    ("objectQualityType" => ObjectQualityType(Primitive<i8>)),
    ("collisionFilterInfo" => CollisionFilterInfo(Primitive<u32>)),
}
