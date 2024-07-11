//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpMoppBvTreeShape`
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

/// `hkpMoppBvTreeShape`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: true
/// -    parent: `hkMoppBvTreeShapeBase`/`0x7c338c66`
/// - signature: `0x90b29d39`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpMoppBvTreeShape<'a> {
    /// # C++ Parent class(`hkMoppBvTreeShapeBase` => parent: `hkpBvTreeShape`) field Info
    /// -   name:`"code"`
    /// -   type: `struct hkpMoppCode*`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub code: Cow<'a, str>,
    /// # C++ Parent class(`hkMoppBvTreeShapeBase` => parent: `hkpBvTreeShape`) field Info
    /// -   name:`"moppData"`
    /// -   type: `void*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub mopp_data: Cow<'a, str>,
    /// # C++ Parent class(`hkMoppBvTreeShapeBase` => parent: `hkpBvTreeShape`) field Info
    /// -   name:`"moppDataSize"`
    /// -   type: `hkUint32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub mopp_data_size: u32,
    /// # C++ Parent class(`hkMoppBvTreeShapeBase` => parent: `hkpBvTreeShape`) field Info
    /// -   name:`"codeInfoCopy"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub code_info_copy: Vector4<f32>,

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
    /// -   name:`"child"`
    /// -   type: `struct hkpSingleShapeContainer`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub child: SingleClass<HkpSingleShapeContainer<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"childSize"`
    /// -   type: `hkInt32`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub child_size: i32,
}

impl Serialize for HkpMoppBvTreeShape<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpMoppBvTreeShapeVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpMoppBvTreeShape<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpMoppBvTreeShapeVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpMoppBvTreeShapeVisitor<'a>>> for HkpMoppBvTreeShape<'a> {
    fn from(_values: Vec<HkpMoppBvTreeShapeVisitor<'a>>) -> Self {
            let mut code = None;
            let mut mopp_data = None;
            let mut mopp_data_size = None;
            let mut code_info_copy = None;
            let mut bv_tree_type = None;
            let mut user_data = None;
            let mut _type = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut child = None;
            let mut child_size = None;


        for _value in _values {
            match _value {
                HkpMoppBvTreeShapeVisitor::Code(m) => code = Some(m),
                HkpMoppBvTreeShapeVisitor::MoppData(m) => mopp_data = Some(m),
                HkpMoppBvTreeShapeVisitor::MoppDataSize(m) => mopp_data_size = Some(m),
                HkpMoppBvTreeShapeVisitor::CodeInfoCopy(m) => code_info_copy = Some(m),
                HkpMoppBvTreeShapeVisitor::BvTreeType(m) => bv_tree_type = Some(m),
                HkpMoppBvTreeShapeVisitor::UserData(m) => user_data = Some(m),
                HkpMoppBvTreeShapeVisitor::Type(m) => _type = Some(m),
                HkpMoppBvTreeShapeVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpMoppBvTreeShapeVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpMoppBvTreeShapeVisitor::Child(m) => child = Some(m),
                HkpMoppBvTreeShapeVisitor::ChildSize(m) => child_size = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            code: code.unwrap_or_default().into_inner(),
            mopp_data: mopp_data.unwrap_or_default().into_inner(),
            mopp_data_size: mopp_data_size.unwrap_or_default().into_inner(),
            code_info_copy: code_info_copy.unwrap_or_default().into_inner(),
            bv_tree_type: bv_tree_type.unwrap_or_default().into_inner(),
            user_data: user_data.unwrap_or_default().into_inner(),
            _type: _type.unwrap_or_default().into_inner(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            child: child.unwrap_or_default(),
            child_size: child_size.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpMoppBvTreeShape<'a>> for Vec<HkpMoppBvTreeShapeVisitor<'a>> {
    fn from(data: &HkpMoppBvTreeShape<'a>) -> Self {
        vec![
            HkpMoppBvTreeShapeVisitor::Code(data.code.clone().into()),
            HkpMoppBvTreeShapeVisitor::MoppData(data.mopp_data.clone().into()),
            HkpMoppBvTreeShapeVisitor::MoppDataSize(data.mopp_data_size.into()),
            HkpMoppBvTreeShapeVisitor::CodeInfoCopy(data.code_info_copy.into()),
            HkpMoppBvTreeShapeVisitor::BvTreeType(data.bv_tree_type.clone().into()),
            HkpMoppBvTreeShapeVisitor::UserData(data.user_data.into()),
            HkpMoppBvTreeShapeVisitor::Type(data._type.into()),
            HkpMoppBvTreeShapeVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpMoppBvTreeShapeVisitor::ReferenceCount(data.reference_count.into()),
            HkpMoppBvTreeShapeVisitor::Child(data.child.clone()),
            HkpMoppBvTreeShapeVisitor::ChildSize(data.child_size.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpMoppBvTreeShape<'de> {
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
enum HkpMoppBvTreeShapeVisitor<'a> {
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
    #[serde(rename = "child")]
    Child(SingleClass<HkpSingleShapeContainer<'a>>),
    /// Visitor fields
    #[serde(rename = "childSize", skip_serializing)]
    ChildSize(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpMoppBvTreeShapeVisitor<'de>, "@name",
    ("code" => Code(Primitive<Cow<'de, str>>)),
    ("moppData" => MoppData(Primitive<Cow<'de, str>>)),
    ("moppDataSize" => MoppDataSize(Primitive<u32>)),
    ("codeInfoCopy" => CodeInfoCopy(Primitive<Vector4<f32>>)),
    ("bvTreeType" => BvTreeType(Primitive<BvTreeType>)),
    ("userData" => UserData(Primitive<usize>)),
    ("type" => Type(Primitive<()>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("child" => Child(SingleClass<HkpSingleShapeContainer<'de>>)),
    ("childSize" => ChildSize(Primitive<i32>)),
}
