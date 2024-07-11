//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkMultiThreadCheck`
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

/// `hkMultiThreadCheck`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// - signature: `0x11e4408b`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkMultiThreadCheck {
    /// # C++ Class Fields Info
    /// -   name:`"threadId"`
    /// -   type: `hkUint32`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub thread_id: u32,
    /// # C++ Class Fields Info
    /// -   name:`"stackTraceId"`
    /// -   type: `hkInt32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub stack_trace_id: i32,
    /// # C++ Class Fields Info
    /// -   name:`"markCount"`
    /// -   type: `hkUint16`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub mark_count: u16,
    /// # C++ Class Fields Info
    /// -   name:`"markBitStack"`
    /// -   type: `hkUint16`
    /// - offset: 10
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub mark_bit_stack: u16,
}

impl Serialize for HkMultiThreadCheck {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkMultiThreadCheckVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkMultiThreadCheck {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkMultiThreadCheckVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkMultiThreadCheckVisitor>> for HkMultiThreadCheck {
    fn from(_values: Vec<HkMultiThreadCheckVisitor>) -> Self {
            let mut thread_id = None;
            let mut stack_trace_id = None;
            let mut mark_count = None;
            let mut mark_bit_stack = None;


        for _value in _values {
            match _value {
                HkMultiThreadCheckVisitor::ThreadId(m) => thread_id = Some(m),
                HkMultiThreadCheckVisitor::StackTraceId(m) => stack_trace_id = Some(m),
                HkMultiThreadCheckVisitor::MarkCount(m) => mark_count = Some(m),
                HkMultiThreadCheckVisitor::MarkBitStack(m) => mark_bit_stack = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            thread_id: thread_id.unwrap_or_default().into_inner(),
            stack_trace_id: stack_trace_id.unwrap_or_default().into_inner(),
            mark_count: mark_count.unwrap_or_default().into_inner(),
            mark_bit_stack: mark_bit_stack.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkMultiThreadCheck> for Vec<HkMultiThreadCheckVisitor> {
    fn from(data: &HkMultiThreadCheck) -> Self {
        vec![
            HkMultiThreadCheckVisitor::ThreadId(data.thread_id.into()),
            HkMultiThreadCheckVisitor::StackTraceId(data.stack_trace_id.into()),
            HkMultiThreadCheckVisitor::MarkCount(data.mark_count.into()),
            HkMultiThreadCheckVisitor::MarkBitStack(data.mark_bit_stack.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkMultiThreadCheck {
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
enum HkMultiThreadCheckVisitor {
    /// Visitor fields
    #[serde(rename = "threadId", skip_serializing)]
    ThreadId(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "stackTraceId", skip_serializing)]
    StackTraceId(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "markCount", skip_serializing)]
    MarkCount(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "markBitStack", skip_serializing)]
    MarkBitStack(Primitive<u16>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMultiThreadCheckVisitor, "@name",
    ("threadId" => ThreadId(Primitive<u32>)),
    ("stackTraceId" => StackTraceId(Primitive<i32>)),
    ("markCount" => MarkCount(Primitive<u16>)),
    ("markBitStack" => MarkBitStack(Primitive<u16>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum AccessType {
    #[serde(rename = "HK_ACCESS_IGNORE")]
    #[default]
    HkAccessIgnore = 0,
    #[serde(rename = "HK_ACCESS_RO")]
    HkAccessRo = 1,
    #[serde(rename = "HK_ACCESS_RW")]
    HkAccessRw = 2,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum ReadMode {
    #[serde(rename = "THIS_OBJECT_ONLY")]
    #[default]
    ThisObjectOnly = 0,
    #[serde(rename = "RECURSIVE")]
    Recursive = 1,
}
