//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpGroupFilter`
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

/// `hkpGroupFilter`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 256
/// -    vtable: true
/// -    parent: `hkpCollisionFilter`/`0x60960336`
/// - signature: `0x65ee88e4`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpGroupFilter {
    /// # C++ Parent class(`hkpCollisionFilter` => parent: `hkReferencedObject`) field Info
    /// -   name:`"prepad"`
    /// -   type: `hkUint32[2]`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub prepad: CStyleArray<[u32; 2]>,
    /// # C++ Parent class(`hkpCollisionFilter` => parent: `hkReferencedObject`) field Info
    /// -   name:`"type"`
    /// -   type: `enum hkpFilterType`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub _type: HkpFilterType,
    /// # C++ Parent class(`hkpCollisionFilter` => parent: `hkReferencedObject`) field Info
    /// -   name:`"postpad"`
    /// -   type: `hkUint32[3]`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    pub postpad: CStyleArray<[u32; 3]>,

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
    /// -   name:`"nextFreeSystemGroup"`
    /// -   type: `hkInt32`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub next_free_system_group: i32,
    /// # C++ Class Fields Info
    /// -   name:`"collisionLookupTable"`
    /// -   type: `hkUint32[32]`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub collision_lookup_table: CStyleArray<[u32; 32]>,
    /// # C++ Class Fields Info
    /// -   name:`"pad256"`
    /// -   type: `hkVector4[4]`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE`
    pub pad_256: CStyleArrayVector<Vector4<f32>, 4>,
}

impl Serialize for HkpGroupFilter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpGroupFilterVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpGroupFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpGroupFilterVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpGroupFilterVisitor>> for HkpGroupFilter {
    fn from(_values: Vec<HkpGroupFilterVisitor>) -> Self {
            let mut prepad = None;
            let mut _type = None;
            let mut postpad = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut next_free_system_group = None;
            let mut collision_lookup_table = None;
            let mut pad_256 = None;


        for _value in _values {
            match _value {
                HkpGroupFilterVisitor::Prepad(m) => prepad = Some(m),
                HkpGroupFilterVisitor::Type(m) => _type = Some(m),
                HkpGroupFilterVisitor::Postpad(m) => postpad = Some(m),
                HkpGroupFilterVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpGroupFilterVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpGroupFilterVisitor::NextFreeSystemGroup(m) => next_free_system_group = Some(m),
                HkpGroupFilterVisitor::CollisionLookupTable(m) => collision_lookup_table = Some(m),
                HkpGroupFilterVisitor::Pad256(m) => pad_256 = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            prepad: prepad.unwrap_or_default(),
            _type: _type.unwrap_or_default().into_inner(),
            postpad: postpad.unwrap_or_default(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            next_free_system_group: next_free_system_group.unwrap_or_default().into_inner(),
            collision_lookup_table: collision_lookup_table.unwrap_or_default(),
            pad_256: pad_256.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpGroupFilter> for Vec<HkpGroupFilterVisitor> {
    fn from(data: &HkpGroupFilter) -> Self {
        vec![
            HkpGroupFilterVisitor::Prepad(data.prepad.clone()),
            HkpGroupFilterVisitor::Type(data._type.clone().into()),
            HkpGroupFilterVisitor::Postpad(data.postpad.clone()),
            HkpGroupFilterVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpGroupFilterVisitor::ReferenceCount(data.reference_count.into()),
            HkpGroupFilterVisitor::NextFreeSystemGroup(data.next_free_system_group.into()),
            HkpGroupFilterVisitor::CollisionLookupTable(data.collision_lookup_table.clone()),
            HkpGroupFilterVisitor::Pad256(data.pad_256.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpGroupFilter {
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
enum HkpGroupFilterVisitor {
    /// Visitor fields
    #[serde(rename = "prepad")]
    Prepad(CStyleArray<[u32; 2]>),
    /// Visitor fields
    #[serde(rename = "type")]
    Type(Primitive<HkpFilterType>),
    /// Visitor fields
    #[serde(rename = "postpad")]
    Postpad(CStyleArray<[u32; 3]>),

    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "nextFreeSystemGroup")]
    NextFreeSystemGroup(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "collisionLookupTable")]
    CollisionLookupTable(CStyleArray<[u32; 32]>),
    /// Visitor fields
    #[serde(rename = "pad256")]
    Pad256(CStyleArrayVector<Vector4<f32>, 4>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpGroupFilterVisitor, "@name",
    ("prepad" => Prepad(CStyleArray<[u32; 2]>)),
    ("type" => Type(Primitive<HkpFilterType>)),
    ("postpad" => Postpad(CStyleArray<[u32; 3]>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("nextFreeSystemGroup" => NextFreeSystemGroup(Primitive<i32>)),
    ("collisionLookupTable" => CollisionLookupTable(CStyleArray<[u32; 32]>)),
    ("pad256" => Pad256(CStyleArrayVector<Vector4<f32>, 4>)),
}
