//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpPairCollisionFilter`
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

/// `hkpPairCollisionFilter`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: true
/// -    parent: `hkpCollisionFilter`/`0x60960336`
/// - signature: `0x4abc140e`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpPairCollisionFilter<'a> {
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
    /// -   name:`"disabledPairs"`
    /// -   type: `struct hkpPairCollisionFilterMapPairFilterKeyOverrideType`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub disabled_pairs: SingleClass<HkpPairCollisionFilterMapPairFilterKeyOverrideType<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"childFilter"`
    /// -   type: `struct hkpCollisionFilter*`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    pub child_filter: Cow<'a, str>,
}

impl Serialize for HkpPairCollisionFilter<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpPairCollisionFilterVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpPairCollisionFilter<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpPairCollisionFilterVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpPairCollisionFilterVisitor<'a>>> for HkpPairCollisionFilter<'a> {
    fn from(_values: Vec<HkpPairCollisionFilterVisitor<'a>>) -> Self {
            let mut prepad = None;
            let mut _type = None;
            let mut postpad = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut disabled_pairs = None;
            let mut child_filter = None;


        for _value in _values {
            match _value {
                HkpPairCollisionFilterVisitor::Prepad(m) => prepad = Some(m),
                HkpPairCollisionFilterVisitor::Type(m) => _type = Some(m),
                HkpPairCollisionFilterVisitor::Postpad(m) => postpad = Some(m),
                HkpPairCollisionFilterVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpPairCollisionFilterVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpPairCollisionFilterVisitor::DisabledPairs(m) => disabled_pairs = Some(m),
                HkpPairCollisionFilterVisitor::ChildFilter(m) => child_filter = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            prepad: prepad.unwrap_or_default(),
            _type: _type.unwrap_or_default().into_inner(),
            postpad: postpad.unwrap_or_default(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            disabled_pairs: disabled_pairs.unwrap_or_default(),
            child_filter: child_filter.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpPairCollisionFilter<'a>> for Vec<HkpPairCollisionFilterVisitor<'a>> {
    fn from(data: &HkpPairCollisionFilter<'a>) -> Self {
        vec![
            HkpPairCollisionFilterVisitor::Prepad(data.prepad.clone()),
            HkpPairCollisionFilterVisitor::Type(data._type.clone().into()),
            HkpPairCollisionFilterVisitor::Postpad(data.postpad.clone()),
            HkpPairCollisionFilterVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpPairCollisionFilterVisitor::ReferenceCount(data.reference_count.into()),
            HkpPairCollisionFilterVisitor::DisabledPairs(data.disabled_pairs.clone()),
            HkpPairCollisionFilterVisitor::ChildFilter(data.child_filter.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpPairCollisionFilter<'de> {
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
enum HkpPairCollisionFilterVisitor<'a> {
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
    #[serde(rename = "disabledPairs", skip_serializing)]
    DisabledPairs(SingleClass<HkpPairCollisionFilterMapPairFilterKeyOverrideType<'a>>),
    /// Visitor fields
    #[serde(rename = "childFilter")]
    ChildFilter(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpPairCollisionFilterVisitor<'de>, "@name",
    ("prepad" => Prepad(CStyleArray<[u32; 2]>)),
    ("type" => Type(Primitive<HkpFilterType>)),
    ("postpad" => Postpad(CStyleArray<[u32; 3]>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("disabledPairs" => DisabledPairs(SingleClass<HkpPairCollisionFilterMapPairFilterKeyOverrideType<'de>>)),
    ("childFilter" => ChildFilter(Primitive<Cow<'de, str>>)),
}
