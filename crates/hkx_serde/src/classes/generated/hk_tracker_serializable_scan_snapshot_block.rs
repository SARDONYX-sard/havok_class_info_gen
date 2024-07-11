//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkTrackerSerializableScanSnapshotBlock`
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

/// `hkTrackerSerializableScanSnapshotBlock`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 24
/// -    vtable: false
/// - signature: `0xe7f23e6d`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkTrackerSerializableScanSnapshotBlock {
    /// # C++ Class Fields Info
    /// -   name:`"typeIndex"`
    /// -   type: `hkInt32`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub type_index: i32,
    /// # C++ Class Fields Info
    /// -   name:`"start"`
    /// -   type: `hkUlong`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub start: usize,
    /// # C++ Class Fields Info
    /// -   name:`"size"`
    /// -   type: `hkUlong`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub size: usize,
    /// # C++ Class Fields Info
    /// -   name:`"arraySize"`
    /// -   type: `hkInt32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub array_size: i32,
    /// # C++ Class Fields Info
    /// -   name:`"startReferenceIndex"`
    /// -   type: `hkInt32`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub start_reference_index: i32,
    /// # C++ Class Fields Info
    /// -   name:`"numReferences"`
    /// -   type: `hkInt32`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub num_references: i32,
}

impl Serialize for HkTrackerSerializableScanSnapshotBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkTrackerSerializableScanSnapshotBlockVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkTrackerSerializableScanSnapshotBlock {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkTrackerSerializableScanSnapshotBlockVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkTrackerSerializableScanSnapshotBlockVisitor>> for HkTrackerSerializableScanSnapshotBlock {
    fn from(_values: Vec<HkTrackerSerializableScanSnapshotBlockVisitor>) -> Self {
            let mut type_index = None;
            let mut start = None;
            let mut size = None;
            let mut array_size = None;
            let mut start_reference_index = None;
            let mut num_references = None;


        for _value in _values {
            match _value {
                HkTrackerSerializableScanSnapshotBlockVisitor::TypeIndex(m) => type_index = Some(m),
                HkTrackerSerializableScanSnapshotBlockVisitor::Start(m) => start = Some(m),
                HkTrackerSerializableScanSnapshotBlockVisitor::Size(m) => size = Some(m),
                HkTrackerSerializableScanSnapshotBlockVisitor::ArraySize(m) => array_size = Some(m),
                HkTrackerSerializableScanSnapshotBlockVisitor::StartReferenceIndex(m) => start_reference_index = Some(m),
                HkTrackerSerializableScanSnapshotBlockVisitor::NumReferences(m) => num_references = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            type_index: type_index.unwrap_or_default().into_inner(),
            start: start.unwrap_or_default().into_inner(),
            size: size.unwrap_or_default().into_inner(),
            array_size: array_size.unwrap_or_default().into_inner(),
            start_reference_index: start_reference_index.unwrap_or_default().into_inner(),
            num_references: num_references.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkTrackerSerializableScanSnapshotBlock> for Vec<HkTrackerSerializableScanSnapshotBlockVisitor> {
    fn from(data: &HkTrackerSerializableScanSnapshotBlock) -> Self {
        vec![
            HkTrackerSerializableScanSnapshotBlockVisitor::TypeIndex(data.type_index.into()),
            HkTrackerSerializableScanSnapshotBlockVisitor::Start(data.start.into()),
            HkTrackerSerializableScanSnapshotBlockVisitor::Size(data.size.into()),
            HkTrackerSerializableScanSnapshotBlockVisitor::ArraySize(data.array_size.into()),
            HkTrackerSerializableScanSnapshotBlockVisitor::StartReferenceIndex(data.start_reference_index.into()),
            HkTrackerSerializableScanSnapshotBlockVisitor::NumReferences(data.num_references.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkTrackerSerializableScanSnapshotBlock {
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
enum HkTrackerSerializableScanSnapshotBlockVisitor {
    /// Visitor fields
    #[serde(rename = "typeIndex")]
    TypeIndex(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "start")]
    Start(Primitive<usize>),
    /// Visitor fields
    #[serde(rename = "size")]
    Size(Primitive<usize>),
    /// Visitor fields
    #[serde(rename = "arraySize")]
    ArraySize(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "startReferenceIndex")]
    StartReferenceIndex(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "numReferences")]
    NumReferences(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkTrackerSerializableScanSnapshotBlockVisitor, "@name",
    ("typeIndex" => TypeIndex(Primitive<i32>)),
    ("start" => Start(Primitive<usize>)),
    ("size" => Size(Primitive<usize>)),
    ("arraySize" => ArraySize(Primitive<i32>)),
    ("startReferenceIndex" => StartReferenceIndex(Primitive<i32>)),
    ("numReferences" => NumReferences(Primitive<i32>)),
}
