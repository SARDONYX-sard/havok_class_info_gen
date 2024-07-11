//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkxSparselyAnimatedEnum`
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

/// `hkxSparselyAnimatedEnum`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 36
/// -    vtable: true
/// -    parent: `hkxSparselyAnimatedInt`/`0xca961951`
/// - signature: `0x68a47b64`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkxSparselyAnimatedEnum<'a> {
    /// # C++ Parent class(`hkxSparselyAnimatedInt` => parent: `hkReferencedObject`) field Info
    /// -   name:`"ints"`
    /// -   type: `hkArray<hkInt32>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub ints: HkArrayNum<i32>,
    /// # C++ Parent class(`hkxSparselyAnimatedInt` => parent: `hkReferencedObject`) field Info
    /// -   name:`"times"`
    /// -   type: `hkArray<hkReal>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub times: HkArrayNum<f32>,

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
    /// -   name:`"enum"`
    /// -   type: `struct hkxEnum*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub _enum: Cow<'a, str>,
}

impl Serialize for HkxSparselyAnimatedEnum<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkxSparselyAnimatedEnumVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkxSparselyAnimatedEnum<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkxSparselyAnimatedEnumVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkxSparselyAnimatedEnumVisitor<'a>>> for HkxSparselyAnimatedEnum<'a> {
    fn from(_values: Vec<HkxSparselyAnimatedEnumVisitor<'a>>) -> Self {
            let mut ints = None;
            let mut times = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut _enum = None;


        for _value in _values {
            match _value {
                HkxSparselyAnimatedEnumVisitor::Ints(m) => ints = Some(m),
                HkxSparselyAnimatedEnumVisitor::Times(m) => times = Some(m),
                HkxSparselyAnimatedEnumVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkxSparselyAnimatedEnumVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkxSparselyAnimatedEnumVisitor::Enum(m) => _enum = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            ints: ints.unwrap_or_default(),
            times: times.unwrap_or_default(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            _enum: _enum.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkxSparselyAnimatedEnum<'a>> for Vec<HkxSparselyAnimatedEnumVisitor<'a>> {
    fn from(data: &HkxSparselyAnimatedEnum<'a>) -> Self {
        vec![
            HkxSparselyAnimatedEnumVisitor::Ints(data.ints.clone()),
            HkxSparselyAnimatedEnumVisitor::Times(data.times.clone()),
            HkxSparselyAnimatedEnumVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkxSparselyAnimatedEnumVisitor::ReferenceCount(data.reference_count.into()),
            HkxSparselyAnimatedEnumVisitor::Enum(data._enum.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkxSparselyAnimatedEnum<'de> {
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
enum HkxSparselyAnimatedEnumVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "ints")]
    Ints(HkArrayNum<i32>),
    /// Visitor fields
    #[serde(rename = "times")]
    Times(HkArrayNum<f32>),

    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "enum")]
    Enum(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxSparselyAnimatedEnumVisitor<'de>, "@name",
    ("ints" => Ints(HkArrayNum<i32>)),
    ("times" => Times(HkArrayNum<f32>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("enum" => Enum(Primitive<Cow<'de, str>>)),
}
