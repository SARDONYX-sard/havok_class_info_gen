//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkTrackerSerializableScanSnapshot`
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

/// `hkTrackerSerializableScanSnapshot`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 92
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x875af1d9`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkTrackerSerializableScanSnapshot {
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
    /// -   name:`"allocations"`
    /// -   type: `hkArray<struct hkTrackerSerializableScanSnapshotAllocation>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub allocations: HkArrayClass<HkTrackerSerializableScanSnapshotAllocation>,
    /// # C++ Class Fields Info
    /// -   name:`"blocks"`
    /// -   type: `hkArray<struct hkTrackerSerializableScanSnapshotBlock>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub blocks: HkArrayClass<HkTrackerSerializableScanSnapshotBlock>,
    /// # C++ Class Fields Info
    /// -   name:`"refs"`
    /// -   type: `hkArray<hkInt32>`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub refs: HkArrayNum<i32>,
    /// # C++ Class Fields Info
    /// -   name:`"typeNames"`
    /// -   type: `hkArray<hkUint8>`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub type_names: HkArrayNum<u8>,
    /// # C++ Class Fields Info
    /// -   name:`"traceText"`
    /// -   type: `hkArray<hkUint8>`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    pub trace_text: HkArrayNum<u8>,
    /// # C++ Class Fields Info
    /// -   name:`"traceAddrs"`
    /// -   type: `hkArray<hkUint64>`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    pub trace_addrs: HkArrayNum<u64>,
    /// # C++ Class Fields Info
    /// -   name:`"traceParents"`
    /// -   type: `hkArray<hkInt32>`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub trace_parents: HkArrayNum<i32>,
}

impl Serialize for HkTrackerSerializableScanSnapshot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkTrackerSerializableScanSnapshotVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkTrackerSerializableScanSnapshot {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkTrackerSerializableScanSnapshotVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkTrackerSerializableScanSnapshotVisitor>> for HkTrackerSerializableScanSnapshot {
    fn from(_values: Vec<HkTrackerSerializableScanSnapshotVisitor>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut allocations = None;
            let mut blocks = None;
            let mut refs = None;
            let mut type_names = None;
            let mut trace_text = None;
            let mut trace_addrs = None;
            let mut trace_parents = None;


        for _value in _values {
            match _value {
                HkTrackerSerializableScanSnapshotVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkTrackerSerializableScanSnapshotVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkTrackerSerializableScanSnapshotVisitor::Allocations(m) => allocations = Some(m),
                HkTrackerSerializableScanSnapshotVisitor::Blocks(m) => blocks = Some(m),
                HkTrackerSerializableScanSnapshotVisitor::Refs(m) => refs = Some(m),
                HkTrackerSerializableScanSnapshotVisitor::TypeNames(m) => type_names = Some(m),
                HkTrackerSerializableScanSnapshotVisitor::TraceText(m) => trace_text = Some(m),
                HkTrackerSerializableScanSnapshotVisitor::TraceAddrs(m) => trace_addrs = Some(m),
                HkTrackerSerializableScanSnapshotVisitor::TraceParents(m) => trace_parents = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            allocations: allocations.unwrap_or_default(),
            blocks: blocks.unwrap_or_default(),
            refs: refs.unwrap_or_default(),
            type_names: type_names.unwrap_or_default(),
            trace_text: trace_text.unwrap_or_default(),
            trace_addrs: trace_addrs.unwrap_or_default(),
            trace_parents: trace_parents.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkTrackerSerializableScanSnapshot> for Vec<HkTrackerSerializableScanSnapshotVisitor> {
    fn from(data: &HkTrackerSerializableScanSnapshot) -> Self {
        vec![
            HkTrackerSerializableScanSnapshotVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkTrackerSerializableScanSnapshotVisitor::ReferenceCount(data.reference_count.into()),
            HkTrackerSerializableScanSnapshotVisitor::Allocations(data.allocations.clone()),
            HkTrackerSerializableScanSnapshotVisitor::Blocks(data.blocks.clone()),
            HkTrackerSerializableScanSnapshotVisitor::Refs(data.refs.clone()),
            HkTrackerSerializableScanSnapshotVisitor::TypeNames(data.type_names.clone()),
            HkTrackerSerializableScanSnapshotVisitor::TraceText(data.trace_text.clone()),
            HkTrackerSerializableScanSnapshotVisitor::TraceAddrs(data.trace_addrs.clone()),
            HkTrackerSerializableScanSnapshotVisitor::TraceParents(data.trace_parents.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkTrackerSerializableScanSnapshot {
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
enum HkTrackerSerializableScanSnapshotVisitor {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "allocations")]
    Allocations(HkArrayClass<HkTrackerSerializableScanSnapshotAllocation>),
    /// Visitor fields
    #[serde(rename = "blocks")]
    Blocks(HkArrayClass<HkTrackerSerializableScanSnapshotBlock>),
    /// Visitor fields
    #[serde(rename = "refs")]
    Refs(HkArrayNum<i32>),
    /// Visitor fields
    #[serde(rename = "typeNames")]
    TypeNames(HkArrayNum<u8>),
    /// Visitor fields
    #[serde(rename = "traceText")]
    TraceText(HkArrayNum<u8>),
    /// Visitor fields
    #[serde(rename = "traceAddrs")]
    TraceAddrs(HkArrayNum<u64>),
    /// Visitor fields
    #[serde(rename = "traceParents")]
    TraceParents(HkArrayNum<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkTrackerSerializableScanSnapshotVisitor, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("allocations" => Allocations(HkArrayClass<HkTrackerSerializableScanSnapshotAllocation>)),
    ("blocks" => Blocks(HkArrayClass<HkTrackerSerializableScanSnapshotBlock>)),
    ("refs" => Refs(HkArrayNum<i32>)),
    ("typeNames" => TypeNames(HkArrayNum<u8>)),
    ("traceText" => TraceText(HkArrayNum<u8>)),
    ("traceAddrs" => TraceAddrs(HkArrayNum<u64>)),
    ("traceParents" => TraceParents(HkArrayNum<i32>)),
}
