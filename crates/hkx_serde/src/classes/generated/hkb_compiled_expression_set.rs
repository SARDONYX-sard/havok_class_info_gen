//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbCompiledExpressionSet`
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

/// `hkbCompiledExpressionSet`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 36
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x3a7d76cc`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbCompiledExpressionSet {
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
    /// -   name:`"rpn"`
    /// -   type: `hkArray<struct hkbCompiledExpressionSetToken>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub rpn: HkArrayClass<HkbCompiledExpressionSetToken>,
    /// # C++ Class Fields Info
    /// -   name:`"expressionToRpnIndex"`
    /// -   type: `hkArray<hkInt32>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub expression_to_rpn_index: HkArrayNum<i32>,
    /// # C++ Class Fields Info
    /// -   name:`"numExpressions"`
    /// -   type: `hkInt8`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub num_expressions: i8,
}

impl Serialize for HkbCompiledExpressionSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbCompiledExpressionSetVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbCompiledExpressionSet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbCompiledExpressionSetVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkbCompiledExpressionSetVisitor>> for HkbCompiledExpressionSet {
    fn from(_values: Vec<HkbCompiledExpressionSetVisitor>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut rpn = None;
            let mut expression_to_rpn_index = None;
            let mut num_expressions = None;


        for _value in _values {
            match _value {
                HkbCompiledExpressionSetVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbCompiledExpressionSetVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbCompiledExpressionSetVisitor::Rpn(m) => rpn = Some(m),
                HkbCompiledExpressionSetVisitor::ExpressionToRpnIndex(m) => expression_to_rpn_index = Some(m),
                HkbCompiledExpressionSetVisitor::NumExpressions(m) => num_expressions = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            rpn: rpn.unwrap_or_default(),
            expression_to_rpn_index: expression_to_rpn_index.unwrap_or_default(),
            num_expressions: num_expressions.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkbCompiledExpressionSet> for Vec<HkbCompiledExpressionSetVisitor> {
    fn from(data: &HkbCompiledExpressionSet) -> Self {
        vec![
            HkbCompiledExpressionSetVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbCompiledExpressionSetVisitor::ReferenceCount(data.reference_count.into()),
            HkbCompiledExpressionSetVisitor::Rpn(data.rpn.clone()),
            HkbCompiledExpressionSetVisitor::ExpressionToRpnIndex(data.expression_to_rpn_index.clone()),
            HkbCompiledExpressionSetVisitor::NumExpressions(data.num_expressions.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbCompiledExpressionSet {
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
enum HkbCompiledExpressionSetVisitor {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "rpn")]
    Rpn(HkArrayClass<HkbCompiledExpressionSetToken>),
    /// Visitor fields
    #[serde(rename = "expressionToRpnIndex")]
    ExpressionToRpnIndex(HkArrayNum<i32>),
    /// Visitor fields
    #[serde(rename = "numExpressions")]
    NumExpressions(Primitive<i8>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbCompiledExpressionSetVisitor, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("rpn" => Rpn(HkArrayClass<HkbCompiledExpressionSetToken>)),
    ("expressionToRpnIndex" => ExpressionToRpnIndex(HkArrayNum<i32>)),
    ("numExpressions" => NumExpressions(Primitive<i8>)),
}
