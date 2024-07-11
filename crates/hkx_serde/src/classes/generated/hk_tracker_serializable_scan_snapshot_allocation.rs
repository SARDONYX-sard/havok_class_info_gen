//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkTrackerSerializableScanSnapshotAllocation`
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

/// `hkTrackerSerializableScanSnapshotAllocation`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// - signature: `0x9ab3a6ac`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkTrackerSerializableScanSnapshotAllocation {
    /// # C++ Class Fields Info
    /// -   name:`"start"`
    /// -   type: `hkUlong`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub start: usize,
    /// # C++ Class Fields Info
    /// -   name:`"size"`
    /// -   type: `hkUlong`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub size: usize,
    /// # C++ Class Fields Info
    /// -   name:`"traceId"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub trace_id: i32,
}

impl Serialize for HkTrackerSerializableScanSnapshotAllocation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkTrackerSerializableScanSnapshotAllocationVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkTrackerSerializableScanSnapshotAllocation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkTrackerSerializableScanSnapshotAllocationVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkTrackerSerializableScanSnapshotAllocationVisitor>> for HkTrackerSerializableScanSnapshotAllocation {
    fn from(_values: Vec<HkTrackerSerializableScanSnapshotAllocationVisitor>) -> Self {
            let mut start = None;
            let mut size = None;
            let mut trace_id = None;


        for _value in _values {
            match _value {
                HkTrackerSerializableScanSnapshotAllocationVisitor::Start(m) => start = Some(m),
                HkTrackerSerializableScanSnapshotAllocationVisitor::Size(m) => size = Some(m),
                HkTrackerSerializableScanSnapshotAllocationVisitor::TraceId(m) => trace_id = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            start: start.unwrap_or_default().into_inner(),
            size: size.unwrap_or_default().into_inner(),
            trace_id: trace_id.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkTrackerSerializableScanSnapshotAllocation> for Vec<HkTrackerSerializableScanSnapshotAllocationVisitor> {
    fn from(data: &HkTrackerSerializableScanSnapshotAllocation) -> Self {
        vec![
            HkTrackerSerializableScanSnapshotAllocationVisitor::Start(data.start.into()),
            HkTrackerSerializableScanSnapshotAllocationVisitor::Size(data.size.into()),
            HkTrackerSerializableScanSnapshotAllocationVisitor::TraceId(data.trace_id.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkTrackerSerializableScanSnapshotAllocation {
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
enum HkTrackerSerializableScanSnapshotAllocationVisitor {
    /// Visitor fields
    #[serde(rename = "start")]
    Start(Primitive<usize>),
    /// Visitor fields
    #[serde(rename = "size")]
    Size(Primitive<usize>),
    /// Visitor fields
    #[serde(rename = "traceId")]
    TraceId(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkTrackerSerializableScanSnapshotAllocationVisitor, "@name",
    ("start" => Start(Primitive<usize>)),
    ("size" => Size(Primitive<usize>)),
    ("traceId" => TraceId(Primitive<i32>)),
}
