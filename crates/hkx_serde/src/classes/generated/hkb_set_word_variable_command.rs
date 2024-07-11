//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbSetWordVariableCommand`
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

/// `hkbSetWordVariableCommand`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xf3ae5fca`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbSetWordVariableCommand {
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
    /// -   name:`"quadValue"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub quad_value: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"characterId"`
    /// -   type: `hkUint64`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub character_id: u64,
    /// # C++ Class Fields Info
    /// -   name:`"variableId"`
    /// -   type: `hkInt32`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub variable_id: i32,
    /// # C++ Class Fields Info
    /// -   name:`"value"`
    /// -   type: `struct hkbVariableValue`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub value: SingleClass<HkbVariableValue>,
    /// # C++ Class Fields Info
    /// -   name:`"type"`
    /// -   type: `enum VariableType`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub _type: VariableType,
    /// # C++ Class Fields Info
    /// -   name:`"global"`
    /// -   type: `hkBool`
    /// - offset: 49
    /// -  flags: `FLAGS_NONE`
    pub global: bool,
}

impl Serialize for HkbSetWordVariableCommand {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbSetWordVariableCommandVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbSetWordVariableCommand {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbSetWordVariableCommandVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkbSetWordVariableCommandVisitor>> for HkbSetWordVariableCommand {
    fn from(_values: Vec<HkbSetWordVariableCommandVisitor>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut quad_value = None;
            let mut character_id = None;
            let mut variable_id = None;
            let mut value = None;
            let mut _type = None;
            let mut global = None;


        for _value in _values {
            match _value {
                HkbSetWordVariableCommandVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbSetWordVariableCommandVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbSetWordVariableCommandVisitor::QuadValue(m) => quad_value = Some(m),
                HkbSetWordVariableCommandVisitor::CharacterId(m) => character_id = Some(m),
                HkbSetWordVariableCommandVisitor::VariableId(m) => variable_id = Some(m),
                HkbSetWordVariableCommandVisitor::Value(m) => value = Some(m),
                HkbSetWordVariableCommandVisitor::Type(m) => _type = Some(m),
                HkbSetWordVariableCommandVisitor::Global(m) => global = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            quad_value: quad_value.unwrap_or_default().into_inner(),
            character_id: character_id.unwrap_or_default().into_inner(),
            variable_id: variable_id.unwrap_or_default().into_inner(),
            value: value.unwrap_or_default(),
            _type: _type.unwrap_or_default().into_inner(),
            global: global.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkbSetWordVariableCommand> for Vec<HkbSetWordVariableCommandVisitor> {
    fn from(data: &HkbSetWordVariableCommand) -> Self {
        vec![
            HkbSetWordVariableCommandVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbSetWordVariableCommandVisitor::ReferenceCount(data.reference_count.into()),
            HkbSetWordVariableCommandVisitor::QuadValue(data.quad_value.into()),
            HkbSetWordVariableCommandVisitor::CharacterId(data.character_id.into()),
            HkbSetWordVariableCommandVisitor::VariableId(data.variable_id.into()),
            HkbSetWordVariableCommandVisitor::Value(data.value.clone()),
            HkbSetWordVariableCommandVisitor::Type(data._type.clone().into()),
            HkbSetWordVariableCommandVisitor::Global(data.global.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbSetWordVariableCommand {
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
enum HkbSetWordVariableCommandVisitor {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "quadValue")]
    QuadValue(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "characterId")]
    CharacterId(Primitive<u64>),
    /// Visitor fields
    #[serde(rename = "variableId")]
    VariableId(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "value")]
    Value(SingleClass<HkbVariableValue>),
    /// Visitor fields
    #[serde(rename = "type")]
    Type(Primitive<VariableType>),
    /// Visitor fields
    #[serde(rename = "global")]
    Global(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbSetWordVariableCommandVisitor, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("quadValue" => QuadValue(Primitive<Vector4<f32>>)),
    ("characterId" => CharacterId(Primitive<u64>)),
    ("variableId" => VariableId(Primitive<i32>)),
    ("value" => Value(SingleClass<HkbVariableValue>)),
    ("type" => Type(Primitive<VariableType>)),
    ("global" => Global(Primitive<bool>)),
}
