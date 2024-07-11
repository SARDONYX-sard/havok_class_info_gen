//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpMoppCode`
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

/// `hkpMoppCode`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x924c2661`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpMoppCode {
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
    /// -   name:`"info"`
    /// -   type: `struct hkpMoppCodeCodeInfo`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub info: SingleClass<HkpMoppCodeCodeInfo>,
    /// # C++ Class Fields Info
    /// -   name:`"data"`
    /// -   type: `hkArray<hkUint8>`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub data: HkArrayNum<u8>,
    /// # C++ Class Fields Info
    /// -   name:`"buildType"`
    /// -   type: `enum BuildType`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub build_type: BuildType,
}

impl Serialize for HkpMoppCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpMoppCodeVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpMoppCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpMoppCodeVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpMoppCodeVisitor>> for HkpMoppCode {
    fn from(_values: Vec<HkpMoppCodeVisitor>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut info = None;
            let mut data = None;
            let mut build_type = None;


        for _value in _values {
            match _value {
                HkpMoppCodeVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpMoppCodeVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpMoppCodeVisitor::Info(m) => info = Some(m),
                HkpMoppCodeVisitor::Data(m) => data = Some(m),
                HkpMoppCodeVisitor::BuildType(m) => build_type = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            info: info.unwrap_or_default(),
            data: data.unwrap_or_default(),
            build_type: build_type.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpMoppCode> for Vec<HkpMoppCodeVisitor> {
    fn from(data: &HkpMoppCode) -> Self {
        vec![
            HkpMoppCodeVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpMoppCodeVisitor::ReferenceCount(data.reference_count.into()),
            HkpMoppCodeVisitor::Info(data.info.clone()),
            HkpMoppCodeVisitor::Data(data.data.clone()),
            HkpMoppCodeVisitor::BuildType(data.build_type.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpMoppCode {
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
enum HkpMoppCodeVisitor {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "info")]
    Info(SingleClass<HkpMoppCodeCodeInfo>),
    /// Visitor fields
    #[serde(rename = "data")]
    Data(HkArrayNum<u8>),
    /// Visitor fields
    #[serde(rename = "buildType")]
    BuildType(Primitive<BuildType>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpMoppCodeVisitor, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("info" => Info(SingleClass<HkpMoppCodeCodeInfo>)),
    ("data" => Data(HkArrayNum<u8>)),
    ("buildType" => BuildType(Primitive<BuildType>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum BuildType {
    #[serde(rename = "BUILT_WITH_CHUNK_SUBDIVISION")]
    #[default]
    BuiltWithChunkSubdivision = 0,
    #[serde(rename = "BUILT_WITHOUT_CHUNK_SUBDIVISION")]
    BuiltWithoutChunkSubdivision = 1,
    #[serde(rename = "BUILD_NOT_SET")]
    BuildNotSet = 2,
}
