//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkMoppBvTreeShapeBase`
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

/// `hkMoppBvTreeShapeBase`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: true
/// -    parent: `hkpBvTreeShape`/`0xa823d623`
/// - signature: `0x7c338c66`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkMoppBvTreeShapeBase<'a> {
    /// # C++ Parent class(`hkpBvTreeShape` => parent: `hkpShape`) field Info
    /// -   name:`"bvTreeType"`
    /// -   type: `enum BvTreeType`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub bv_tree_type: BvTreeType,

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
    /// -   name:`"code"`
    /// -   type: `struct hkpMoppCode*`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub code: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"moppData"`
    /// -   type: `void*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub mopp_data: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"moppDataSize"`
    /// -   type: `hkUint32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub mopp_data_size: u32,
    /// # C++ Class Fields Info
    /// -   name:`"codeInfoCopy"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub code_info_copy: Vector4<f32>,
}

impl Serialize for HkMoppBvTreeShapeBase<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkMoppBvTreeShapeBaseVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkMoppBvTreeShapeBase<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkMoppBvTreeShapeBaseVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkMoppBvTreeShapeBaseVisitor<'a>>> for HkMoppBvTreeShapeBase<'a> {
    fn from(_values: Vec<HkMoppBvTreeShapeBaseVisitor<'a>>) -> Self {
            let mut bv_tree_type = None;
            let mut user_data = None;
            let mut _type = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut code = None;
            let mut mopp_data = None;
            let mut mopp_data_size = None;
            let mut code_info_copy = None;


        for _value in _values {
            match _value {
                HkMoppBvTreeShapeBaseVisitor::BvTreeType(m) => bv_tree_type = Some(m),
                HkMoppBvTreeShapeBaseVisitor::UserData(m) => user_data = Some(m),
                HkMoppBvTreeShapeBaseVisitor::Type(m) => _type = Some(m),
                HkMoppBvTreeShapeBaseVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkMoppBvTreeShapeBaseVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkMoppBvTreeShapeBaseVisitor::Code(m) => code = Some(m),
                HkMoppBvTreeShapeBaseVisitor::MoppData(m) => mopp_data = Some(m),
                HkMoppBvTreeShapeBaseVisitor::MoppDataSize(m) => mopp_data_size = Some(m),
                HkMoppBvTreeShapeBaseVisitor::CodeInfoCopy(m) => code_info_copy = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            bv_tree_type: bv_tree_type.unwrap_or_default().into_inner(),
            user_data: user_data.unwrap_or_default().into_inner(),
            _type: _type.unwrap_or_default().into_inner(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            code: code.unwrap_or_default().into_inner(),
            mopp_data: mopp_data.unwrap_or_default().into_inner(),
            mopp_data_size: mopp_data_size.unwrap_or_default().into_inner(),
            code_info_copy: code_info_copy.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkMoppBvTreeShapeBase<'a>> for Vec<HkMoppBvTreeShapeBaseVisitor<'a>> {
    fn from(data: &HkMoppBvTreeShapeBase<'a>) -> Self {
        vec![
            HkMoppBvTreeShapeBaseVisitor::BvTreeType(data.bv_tree_type.clone().into()),
            HkMoppBvTreeShapeBaseVisitor::UserData(data.user_data.into()),
            HkMoppBvTreeShapeBaseVisitor::Type(data._type.into()),
            HkMoppBvTreeShapeBaseVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkMoppBvTreeShapeBaseVisitor::ReferenceCount(data.reference_count.into()),
            HkMoppBvTreeShapeBaseVisitor::Code(data.code.clone().into()),
            HkMoppBvTreeShapeBaseVisitor::MoppData(data.mopp_data.clone().into()),
            HkMoppBvTreeShapeBaseVisitor::MoppDataSize(data.mopp_data_size.into()),
            HkMoppBvTreeShapeBaseVisitor::CodeInfoCopy(data.code_info_copy.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkMoppBvTreeShapeBase<'de> {
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
enum HkMoppBvTreeShapeBaseVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "bvTreeType")]
    BvTreeType(Primitive<BvTreeType>),

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
    #[serde(rename = "code")]
    Code(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "moppData", skip_serializing)]
    MoppData(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "moppDataSize", skip_serializing)]
    MoppDataSize(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "codeInfoCopy", skip_serializing)]
    CodeInfoCopy(Primitive<Vector4<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMoppBvTreeShapeBaseVisitor<'de>, "@name",
    ("bvTreeType" => BvTreeType(Primitive<BvTreeType>)),
    ("userData" => UserData(Primitive<usize>)),
    ("type" => Type(Primitive<()>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("code" => Code(Primitive<Cow<'de, str>>)),
    ("moppData" => MoppData(Primitive<Cow<'de, str>>)),
    ("moppDataSize" => MoppDataSize(Primitive<u32>)),
    ("codeInfoCopy" => CodeInfoCopy(Primitive<Vector4<f32>>)),
}
