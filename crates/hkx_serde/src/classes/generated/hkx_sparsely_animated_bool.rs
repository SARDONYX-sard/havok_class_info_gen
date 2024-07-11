//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkxSparselyAnimatedBool`
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

/// `hkxSparselyAnimatedBool`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x7a894596`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkxSparselyAnimatedBool {
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
    /// -   name:`"bools"`
    /// -   type: `hkArray<hkBool>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub bools: HkArrayRef<bool>,
    /// # C++ Class Fields Info
    /// -   name:`"times"`
    /// -   type: `hkArray<hkReal>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub times: HkArrayNum<f32>,
}

impl Serialize for HkxSparselyAnimatedBool {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkxSparselyAnimatedBoolVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkxSparselyAnimatedBool {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkxSparselyAnimatedBoolVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkxSparselyAnimatedBoolVisitor>> for HkxSparselyAnimatedBool {
    fn from(_values: Vec<HkxSparselyAnimatedBoolVisitor>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut bools = None;
            let mut times = None;


        for _value in _values {
            match _value {
                HkxSparselyAnimatedBoolVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkxSparselyAnimatedBoolVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkxSparselyAnimatedBoolVisitor::Bools(m) => bools = Some(m),
                HkxSparselyAnimatedBoolVisitor::Times(m) => times = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            bools: bools.unwrap_or_default(),
            times: times.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkxSparselyAnimatedBool> for Vec<HkxSparselyAnimatedBoolVisitor> {
    fn from(data: &HkxSparselyAnimatedBool) -> Self {
        vec![
            HkxSparselyAnimatedBoolVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkxSparselyAnimatedBoolVisitor::ReferenceCount(data.reference_count.into()),
            HkxSparselyAnimatedBoolVisitor::Bools(data.bools.clone()),
            HkxSparselyAnimatedBoolVisitor::Times(data.times.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkxSparselyAnimatedBool {
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
enum HkxSparselyAnimatedBoolVisitor {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "bools")]
    Bools(HkArrayRef<bool>),
    /// Visitor fields
    #[serde(rename = "times")]
    Times(HkArrayNum<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxSparselyAnimatedBoolVisitor, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("bools" => Bools(HkArrayRef<bool>)),
    ("times" => Times(HkArrayNum<f32>)),
}
