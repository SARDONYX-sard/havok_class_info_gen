//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbExpressionCondition`
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

/// `hkbExpressionCondition`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: true
/// -    parent: `hkbCondition`/`0xda8c7d7d`
/// - signature: `0x1c3c1045`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbExpressionCondition<'a> {
    // C++ Parent class(`hkbCondition` => parent: `hkReferencedObject`) has no fields
    //
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
    /// -   name:`"expression"`
    /// -   type: `hkStringPtr`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub expression: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"compiledExpressionSet"`
    /// -   type: `void*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub compiled_expression_set: Cow<'a, str>,
}

impl Serialize for HkbExpressionCondition<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbExpressionConditionVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbExpressionCondition<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbExpressionConditionVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbExpressionConditionVisitor<'a>>> for HkbExpressionCondition<'a> {
    fn from(_values: Vec<HkbExpressionConditionVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut expression = None;
            let mut compiled_expression_set = None;


        for _value in _values {
            match _value {
                HkbExpressionConditionVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbExpressionConditionVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbExpressionConditionVisitor::Expression(m) => expression = Some(m),
                HkbExpressionConditionVisitor::CompiledExpressionSet(m) => compiled_expression_set = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            expression: expression.unwrap_or_default().into_inner(),
            compiled_expression_set: compiled_expression_set.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbExpressionCondition<'a>> for Vec<HkbExpressionConditionVisitor<'a>> {
    fn from(data: &HkbExpressionCondition<'a>) -> Self {
        vec![
            HkbExpressionConditionVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbExpressionConditionVisitor::ReferenceCount(data.reference_count.into()),
            HkbExpressionConditionVisitor::Expression(data.expression.clone().into()),
            HkbExpressionConditionVisitor::CompiledExpressionSet(data.compiled_expression_set.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbExpressionCondition<'de> {
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
enum HkbExpressionConditionVisitor<'a> {
    // C++ Parent class(`hkbCondition` => parent: `hkReferencedObject`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "expression")]
    Expression(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "compiledExpressionSet", skip_serializing)]
    CompiledExpressionSet(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbExpressionConditionVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("expression" => Expression(Primitive<Cow<'de, str>>)),
    ("compiledExpressionSet" => CompiledExpressionSet(Primitive<Cow<'de, str>>)),
}
