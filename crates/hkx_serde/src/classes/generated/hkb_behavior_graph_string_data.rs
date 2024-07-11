//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbBehaviorGraphStringData`
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

/// `hkbBehaviorGraphStringData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 56
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xc713064e`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbBehaviorGraphStringData<'a> {
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
    /// -   name:`"eventNames"`
    /// -   type: `hkArray<hkStringPtr>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub event_names: HkArrayStringPtr<'a>,
    /// # C++ Class Fields Info
    /// -   name:`"attributeNames"`
    /// -   type: `hkArray<hkStringPtr>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub attribute_names: HkArrayStringPtr<'a>,
    /// # C++ Class Fields Info
    /// -   name:`"variableNames"`
    /// -   type: `hkArray<hkStringPtr>`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub variable_names: HkArrayStringPtr<'a>,
    /// # C++ Class Fields Info
    /// -   name:`"characterPropertyNames"`
    /// -   type: `hkArray<hkStringPtr>`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub character_property_names: HkArrayStringPtr<'a>,
}

impl Serialize for HkbBehaviorGraphStringData<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbBehaviorGraphStringDataVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbBehaviorGraphStringData<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbBehaviorGraphStringDataVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbBehaviorGraphStringDataVisitor<'a>>> for HkbBehaviorGraphStringData<'a> {
    fn from(_values: Vec<HkbBehaviorGraphStringDataVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut event_names = None;
            let mut attribute_names = None;
            let mut variable_names = None;
            let mut character_property_names = None;


        for _value in _values {
            match _value {
                HkbBehaviorGraphStringDataVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbBehaviorGraphStringDataVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbBehaviorGraphStringDataVisitor::EventNames(m) => event_names = Some(m),
                HkbBehaviorGraphStringDataVisitor::AttributeNames(m) => attribute_names = Some(m),
                HkbBehaviorGraphStringDataVisitor::VariableNames(m) => variable_names = Some(m),
                HkbBehaviorGraphStringDataVisitor::CharacterPropertyNames(m) => character_property_names = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            event_names: event_names.unwrap_or_default(),
            attribute_names: attribute_names.unwrap_or_default(),
            variable_names: variable_names.unwrap_or_default(),
            character_property_names: character_property_names.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbBehaviorGraphStringData<'a>> for Vec<HkbBehaviorGraphStringDataVisitor<'a>> {
    fn from(data: &HkbBehaviorGraphStringData<'a>) -> Self {
        vec![
            HkbBehaviorGraphStringDataVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbBehaviorGraphStringDataVisitor::ReferenceCount(data.reference_count.into()),
            HkbBehaviorGraphStringDataVisitor::EventNames(data.event_names.clone()),
            HkbBehaviorGraphStringDataVisitor::AttributeNames(data.attribute_names.clone()),
            HkbBehaviorGraphStringDataVisitor::VariableNames(data.variable_names.clone()),
            HkbBehaviorGraphStringDataVisitor::CharacterPropertyNames(data.character_property_names.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbBehaviorGraphStringData<'de> {
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
enum HkbBehaviorGraphStringDataVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "eventNames")]
    EventNames(HkArrayStringPtr<'a>),
    /// Visitor fields
    #[serde(rename = "attributeNames")]
    AttributeNames(HkArrayStringPtr<'a>),
    /// Visitor fields
    #[serde(rename = "variableNames")]
    VariableNames(HkArrayStringPtr<'a>),
    /// Visitor fields
    #[serde(rename = "characterPropertyNames")]
    CharacterPropertyNames(HkArrayStringPtr<'a>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbBehaviorGraphStringDataVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("eventNames" => EventNames(HkArrayStringPtr<'de>)),
    ("attributeNames" => AttributeNames(HkArrayStringPtr<'de>)),
    ("variableNames" => VariableNames(HkArrayStringPtr<'de>)),
    ("characterPropertyNames" => CharacterPropertyNames(HkArrayStringPtr<'de>)),
}
