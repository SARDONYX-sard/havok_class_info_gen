//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbVariableBindingSetBinding`
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

/// `hkbVariableBindingSetBinding`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: false
/// - signature: `0x4d592f72`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbVariableBindingSetBinding<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"memberPath"`
    /// -   type: `hkStringPtr`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub member_path: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"memberClass"`
    /// -   type: `void*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub member_class: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"offsetInObjectPlusOne"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub offset_in_object_plus_one: i32,
    /// # C++ Class Fields Info
    /// -   name:`"offsetInArrayPlusOne"`
    /// -   type: `hkInt32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub offset_in_array_plus_one: i32,
    /// # C++ Class Fields Info
    /// -   name:`"rootVariableIndex"`
    /// -   type: `hkInt32`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub root_variable_index: i32,
    /// # C++ Class Fields Info
    /// -   name:`"variableIndex"`
    /// -   type: `hkInt32`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub variable_index: i32,
    /// # C++ Class Fields Info
    /// -   name:`"bitIndex"`
    /// -   type: `hkInt8`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub bit_index: i8,
    /// # C++ Class Fields Info
    /// -   name:`"bindingType"`
    /// -   type: `enum BindingType`
    /// - offset: 25
    /// -  flags: `FLAGS_NONE`
    pub binding_type: BindingType,
    /// # C++ Class Fields Info
    /// -   name:`"memberType"`
    /// -   type: `enum unknown`
    /// - offset: 26
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub member_type: (),
    /// # C++ Class Fields Info
    /// -   name:`"variableType"`
    /// -   type: `hkInt8`
    /// - offset: 27
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub variable_type: i8,
    /// # C++ Class Fields Info
    /// -   name:`"flags"`
    /// -   type: `flags unknown`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub flags: (),
}

impl Serialize for HkbVariableBindingSetBinding<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbVariableBindingSetBindingVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbVariableBindingSetBinding<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbVariableBindingSetBindingVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbVariableBindingSetBindingVisitor<'a>>> for HkbVariableBindingSetBinding<'a> {
    fn from(_values: Vec<HkbVariableBindingSetBindingVisitor<'a>>) -> Self {
            let mut member_path = None;
            let mut member_class = None;
            let mut offset_in_object_plus_one = None;
            let mut offset_in_array_plus_one = None;
            let mut root_variable_index = None;
            let mut variable_index = None;
            let mut bit_index = None;
            let mut binding_type = None;
            let mut member_type = None;
            let mut variable_type = None;
            let mut flags = None;


        for _value in _values {
            match _value {
                HkbVariableBindingSetBindingVisitor::MemberPath(m) => member_path = Some(m),
                HkbVariableBindingSetBindingVisitor::MemberClass(m) => member_class = Some(m),
                HkbVariableBindingSetBindingVisitor::OffsetInObjectPlusOne(m) => offset_in_object_plus_one = Some(m),
                HkbVariableBindingSetBindingVisitor::OffsetInArrayPlusOne(m) => offset_in_array_plus_one = Some(m),
                HkbVariableBindingSetBindingVisitor::RootVariableIndex(m) => root_variable_index = Some(m),
                HkbVariableBindingSetBindingVisitor::VariableIndex(m) => variable_index = Some(m),
                HkbVariableBindingSetBindingVisitor::BitIndex(m) => bit_index = Some(m),
                HkbVariableBindingSetBindingVisitor::BindingType(m) => binding_type = Some(m),
                HkbVariableBindingSetBindingVisitor::MemberType(m) => member_type = Some(m),
                HkbVariableBindingSetBindingVisitor::VariableType(m) => variable_type = Some(m),
                HkbVariableBindingSetBindingVisitor::Flags(m) => flags = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            member_path: member_path.unwrap_or_default().into_inner(),
            member_class: member_class.unwrap_or_default().into_inner(),
            offset_in_object_plus_one: offset_in_object_plus_one.unwrap_or_default().into_inner(),
            offset_in_array_plus_one: offset_in_array_plus_one.unwrap_or_default().into_inner(),
            root_variable_index: root_variable_index.unwrap_or_default().into_inner(),
            variable_index: variable_index.unwrap_or_default().into_inner(),
            bit_index: bit_index.unwrap_or_default().into_inner(),
            binding_type: binding_type.unwrap_or_default().into_inner(),
            member_type: member_type.unwrap_or_default().into_inner(),
            variable_type: variable_type.unwrap_or_default().into_inner(),
            flags: flags.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbVariableBindingSetBinding<'a>> for Vec<HkbVariableBindingSetBindingVisitor<'a>> {
    fn from(data: &HkbVariableBindingSetBinding<'a>) -> Self {
        vec![
            HkbVariableBindingSetBindingVisitor::MemberPath(data.member_path.clone().into()),
            HkbVariableBindingSetBindingVisitor::MemberClass(data.member_class.clone().into()),
            HkbVariableBindingSetBindingVisitor::OffsetInObjectPlusOne(data.offset_in_object_plus_one.into()),
            HkbVariableBindingSetBindingVisitor::OffsetInArrayPlusOne(data.offset_in_array_plus_one.into()),
            HkbVariableBindingSetBindingVisitor::RootVariableIndex(data.root_variable_index.into()),
            HkbVariableBindingSetBindingVisitor::VariableIndex(data.variable_index.into()),
            HkbVariableBindingSetBindingVisitor::BitIndex(data.bit_index.into()),
            HkbVariableBindingSetBindingVisitor::BindingType(data.binding_type.clone().into()),
            HkbVariableBindingSetBindingVisitor::MemberType(data.member_type.into()),
            HkbVariableBindingSetBindingVisitor::VariableType(data.variable_type.into()),
            HkbVariableBindingSetBindingVisitor::Flags(data.flags.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbVariableBindingSetBinding<'de> {
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
enum HkbVariableBindingSetBindingVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "memberPath")]
    MemberPath(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "memberClass", skip_serializing)]
    MemberClass(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "offsetInObjectPlusOne", skip_serializing)]
    OffsetInObjectPlusOne(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "offsetInArrayPlusOne", skip_serializing)]
    OffsetInArrayPlusOne(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "rootVariableIndex", skip_serializing)]
    RootVariableIndex(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "variableIndex")]
    VariableIndex(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "bitIndex")]
    BitIndex(Primitive<i8>),
    /// Visitor fields
    #[serde(rename = "bindingType")]
    BindingType(Primitive<BindingType>),
    /// Visitor fields
    #[serde(rename = "memberType", skip_serializing)]
    MemberType(Primitive<()>),
    /// Visitor fields
    #[serde(rename = "variableType", skip_serializing)]
    VariableType(Primitive<i8>),
    /// Visitor fields
    #[serde(rename = "flags", skip_serializing)]
    Flags(Primitive<()>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbVariableBindingSetBindingVisitor<'de>, "@name",
    ("memberPath" => MemberPath(Primitive<Cow<'de, str>>)),
    ("memberClass" => MemberClass(Primitive<Cow<'de, str>>)),
    ("offsetInObjectPlusOne" => OffsetInObjectPlusOne(Primitive<i32>)),
    ("offsetInArrayPlusOne" => OffsetInArrayPlusOne(Primitive<i32>)),
    ("rootVariableIndex" => RootVariableIndex(Primitive<i32>)),
    ("variableIndex" => VariableIndex(Primitive<i32>)),
    ("bitIndex" => BitIndex(Primitive<i8>)),
    ("bindingType" => BindingType(Primitive<BindingType>)),
    ("memberType" => MemberType(Primitive<()>)),
    ("variableType" => VariableType(Primitive<i8>)),
    ("flags" => Flags(Primitive<()>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum BindingType {
    #[serde(rename = "BINDING_TYPE_VARIABLE")]
    #[default]
    BindingTypeVariable = 0,
    #[serde(rename = "BINDING_TYPE_CHARACTER_PROPERTY")]
    BindingTypeCharacterProperty = 1,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum InternalBindingFlags {
    #[serde(rename = "FLAG_NONE")]
    #[default]
    FlagNone = 0,
    #[serde(rename = "FLAG_OUTPUT")]
    FlagOutput = 1,
}
