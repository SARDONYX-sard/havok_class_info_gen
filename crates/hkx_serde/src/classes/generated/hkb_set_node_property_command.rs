//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbSetNodePropertyCommand`
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

/// `hkbSetNodePropertyCommand`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xc5160b64`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbSetNodePropertyCommand<'a> {
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
    /// -   name:`"characterId"`
    /// -   type: `hkUint64`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub character_id: u64,
    /// # C++ Class Fields Info
    /// -   name:`"nodeName"`
    /// -   type: `hkStringPtr`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub node_name: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"propertyName"`
    /// -   type: `hkStringPtr`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub property_name: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"propertyValue"`
    /// -   type: `struct hkbVariableValue`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub property_value: SingleClass<HkbVariableValue>,
    /// # C++ Class Fields Info
    /// -   name:`"padding"`
    /// -   type: `hkInt32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub padding: i32,
}

impl Serialize for HkbSetNodePropertyCommand<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbSetNodePropertyCommandVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbSetNodePropertyCommand<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbSetNodePropertyCommandVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbSetNodePropertyCommandVisitor<'a>>> for HkbSetNodePropertyCommand<'a> {
    fn from(_values: Vec<HkbSetNodePropertyCommandVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut character_id = None;
            let mut node_name = None;
            let mut property_name = None;
            let mut property_value = None;
            let mut padding = None;


        for _value in _values {
            match _value {
                HkbSetNodePropertyCommandVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbSetNodePropertyCommandVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbSetNodePropertyCommandVisitor::CharacterId(m) => character_id = Some(m),
                HkbSetNodePropertyCommandVisitor::NodeName(m) => node_name = Some(m),
                HkbSetNodePropertyCommandVisitor::PropertyName(m) => property_name = Some(m),
                HkbSetNodePropertyCommandVisitor::PropertyValue(m) => property_value = Some(m),
                HkbSetNodePropertyCommandVisitor::Padding(m) => padding = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            character_id: character_id.unwrap_or_default().into_inner(),
            node_name: node_name.unwrap_or_default().into_inner(),
            property_name: property_name.unwrap_or_default().into_inner(),
            property_value: property_value.unwrap_or_default(),
            padding: padding.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbSetNodePropertyCommand<'a>> for Vec<HkbSetNodePropertyCommandVisitor<'a>> {
    fn from(data: &HkbSetNodePropertyCommand<'a>) -> Self {
        vec![
            HkbSetNodePropertyCommandVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbSetNodePropertyCommandVisitor::ReferenceCount(data.reference_count.into()),
            HkbSetNodePropertyCommandVisitor::CharacterId(data.character_id.into()),
            HkbSetNodePropertyCommandVisitor::NodeName(data.node_name.clone().into()),
            HkbSetNodePropertyCommandVisitor::PropertyName(data.property_name.clone().into()),
            HkbSetNodePropertyCommandVisitor::PropertyValue(data.property_value.clone()),
            HkbSetNodePropertyCommandVisitor::Padding(data.padding.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbSetNodePropertyCommand<'de> {
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
enum HkbSetNodePropertyCommandVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "characterId")]
    CharacterId(Primitive<u64>),
    /// Visitor fields
    #[serde(rename = "nodeName")]
    NodeName(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "propertyName")]
    PropertyName(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "propertyValue")]
    PropertyValue(SingleClass<HkbVariableValue>),
    /// Visitor fields
    #[serde(rename = "padding")]
    Padding(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbSetNodePropertyCommandVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("characterId" => CharacterId(Primitive<u64>)),
    ("nodeName" => NodeName(Primitive<Cow<'de, str>>)),
    ("propertyName" => PropertyName(Primitive<Cow<'de, str>>)),
    ("propertyValue" => PropertyValue(SingleClass<HkbVariableValue>)),
    ("padding" => Padding(Primitive<i32>)),
}
