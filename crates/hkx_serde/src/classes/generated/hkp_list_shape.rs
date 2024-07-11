//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpListShape`
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

/// `hkpListShape`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: true
/// -    parent: `hkpShapeCollection`/`0xe8c3991d`
/// - signature: `0xa1937cbd`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpListShape<'a> {
    /// # C++ Parent class(`hkpShapeCollection` => parent: `hkpShape`) field Info
    /// -   name:`"disableWelding"`
    /// -   type: `hkBool`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub disable_welding: bool,
    /// # C++ Parent class(`hkpShapeCollection` => parent: `hkpShape`) field Info
    /// -   name:`"collectionType"`
    /// -   type: `enum CollectionType`
    /// - offset: 21
    /// -  flags: `FLAGS_NONE`
    pub collection_type: CollectionType,

    /// # C++ Parent class(`hkpShape` => parent: `hkReferencedObject`) field Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub user_data: usize,
    /// # C++ Parent class(`hkpShape` => parent: `hkReferencedObject`) field Info
    /// -   name:`"type"`
    /// -   type: `enum unknown`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub _type: (),

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
    /// -   name:`"childInfo"`
    /// -   type: `hkArray<struct hkpListShapeChildInfo>`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub child_info: HkArrayClass<HkpListShapeChildInfo<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"flags"`
    /// -   type: `hkUint16`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    pub flags: u16,
    /// # C++ Class Fields Info
    /// -   name:`"numDisabledChildren"`
    /// -   type: `hkUint16`
    /// - offset: 38
    /// -  flags: `FLAGS_NONE`
    pub num_disabled_children: u16,
    /// # C++ Class Fields Info
    /// -   name:`"aabbHalfExtents"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub aabb_half_extents: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"aabbCenter"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub aabb_center: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"enabledChildren"`
    /// -   type: `hkUint32[8]`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub enabled_children: CStyleArray<[u32; 8]>,
}

impl Serialize for HkpListShape<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpListShapeVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpListShape<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpListShapeVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpListShapeVisitor<'a>>> for HkpListShape<'a> {
    fn from(_values: Vec<HkpListShapeVisitor<'a>>) -> Self {
            let mut disable_welding = None;
            let mut collection_type = None;
            let mut user_data = None;
            let mut _type = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut child_info = None;
            let mut flags = None;
            let mut num_disabled_children = None;
            let mut aabb_half_extents = None;
            let mut aabb_center = None;
            let mut enabled_children = None;


        for _value in _values {
            match _value {
                HkpListShapeVisitor::DisableWelding(m) => disable_welding = Some(m),
                HkpListShapeVisitor::CollectionType(m) => collection_type = Some(m),
                HkpListShapeVisitor::UserData(m) => user_data = Some(m),
                HkpListShapeVisitor::Type(m) => _type = Some(m),
                HkpListShapeVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpListShapeVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpListShapeVisitor::ChildInfo(m) => child_info = Some(m),
                HkpListShapeVisitor::Flags(m) => flags = Some(m),
                HkpListShapeVisitor::NumDisabledChildren(m) => num_disabled_children = Some(m),
                HkpListShapeVisitor::AabbHalfExtents(m) => aabb_half_extents = Some(m),
                HkpListShapeVisitor::AabbCenter(m) => aabb_center = Some(m),
                HkpListShapeVisitor::EnabledChildren(m) => enabled_children = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            disable_welding: disable_welding.unwrap_or_default().into_inner(),
            collection_type: collection_type.unwrap_or_default().into_inner(),
            user_data: user_data.unwrap_or_default().into_inner(),
            _type: _type.unwrap_or_default().into_inner(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            child_info: child_info.unwrap_or_default(),
            flags: flags.unwrap_or_default().into_inner(),
            num_disabled_children: num_disabled_children.unwrap_or_default().into_inner(),
            aabb_half_extents: aabb_half_extents.unwrap_or_default().into_inner(),
            aabb_center: aabb_center.unwrap_or_default().into_inner(),
            enabled_children: enabled_children.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpListShape<'a>> for Vec<HkpListShapeVisitor<'a>> {
    fn from(data: &HkpListShape<'a>) -> Self {
        vec![
            HkpListShapeVisitor::DisableWelding(data.disable_welding.into()),
            HkpListShapeVisitor::CollectionType(data.collection_type.clone().into()),
            HkpListShapeVisitor::UserData(data.user_data.into()),
            HkpListShapeVisitor::Type(data._type.into()),
            HkpListShapeVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpListShapeVisitor::ReferenceCount(data.reference_count.into()),
            HkpListShapeVisitor::ChildInfo(data.child_info.clone()),
            HkpListShapeVisitor::Flags(data.flags.into()),
            HkpListShapeVisitor::NumDisabledChildren(data.num_disabled_children.into()),
            HkpListShapeVisitor::AabbHalfExtents(data.aabb_half_extents.into()),
            HkpListShapeVisitor::AabbCenter(data.aabb_center.into()),
            HkpListShapeVisitor::EnabledChildren(data.enabled_children.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpListShape<'de> {
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
enum HkpListShapeVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "disableWelding")]
    DisableWelding(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "collectionType")]
    CollectionType(Primitive<CollectionType>),

    /// Visitor fields
    #[serde(rename = "userData")]
    UserData(Primitive<usize>),
    /// Visitor fields
    #[serde(rename = "type", skip_serializing)]
    Type(Primitive<()>),

    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "childInfo")]
    ChildInfo(HkArrayClass<HkpListShapeChildInfo<'a>>),
    /// Visitor fields
    #[serde(rename = "flags")]
    Flags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "numDisabledChildren")]
    NumDisabledChildren(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "aabbHalfExtents")]
    AabbHalfExtents(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "aabbCenter")]
    AabbCenter(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "enabledChildren")]
    EnabledChildren(CStyleArray<[u32; 8]>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpListShapeVisitor<'de>, "@name",
    ("disableWelding" => DisableWelding(Primitive<bool>)),
    ("collectionType" => CollectionType(Primitive<CollectionType>)),
    ("userData" => UserData(Primitive<usize>)),
    ("type" => Type(Primitive<()>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("childInfo" => ChildInfo(HkArrayClass<HkpListShapeChildInfo<'de>>)),
    ("flags" => Flags(Primitive<u16>)),
    ("numDisabledChildren" => NumDisabledChildren(Primitive<u16>)),
    ("aabbHalfExtents" => AabbHalfExtents(Primitive<Vector4<f32>>)),
    ("aabbCenter" => AabbCenter(Primitive<Vector4<f32>>)),
    ("enabledChildren" => EnabledChildren(CStyleArray<[u32; 8]>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum ListShapeFlags {
    #[serde(rename = "ALL_FLAGS_CLEAR")]
    #[default]
    AllFlagsClear = 0,
    #[serde(rename = "DISABLE_SPU_CACHE_FOR_LIST_CHILD_INFO")]
    DisableSpuCacheForListChildInfo = 1,
}
