//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbNodeInternalStateInfo`
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

/// `hkbNodeInternalStateInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 100
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x7db9971d`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbNodeInternalStateInfo<'a> {
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
    /// -   name:`"syncInfo"`
    /// -   type: `struct hkbGeneratorSyncInfo`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub sync_info: SingleClass<HkbGeneratorSyncInfo>,
    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    pub name: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"internalState"`
    /// -   type: `struct hkReferencedObject*`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    pub internal_state: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"nodeId"`
    /// -   type: `hkInt16`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    pub node_id: i16,
    /// # C++ Class Fields Info
    /// -   name:`"hasActivateBeenCalled"`
    /// -   type: `hkBool`
    /// - offset: 98
    /// -  flags: `FLAGS_NONE`
    pub has_activate_been_called: bool,
}

impl Serialize for HkbNodeInternalStateInfo<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbNodeInternalStateInfoVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbNodeInternalStateInfo<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbNodeInternalStateInfoVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbNodeInternalStateInfoVisitor<'a>>> for HkbNodeInternalStateInfo<'a> {
    fn from(_values: Vec<HkbNodeInternalStateInfoVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut sync_info = None;
            let mut name = None;
            let mut internal_state = None;
            let mut node_id = None;
            let mut has_activate_been_called = None;


        for _value in _values {
            match _value {
                HkbNodeInternalStateInfoVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbNodeInternalStateInfoVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbNodeInternalStateInfoVisitor::SyncInfo(m) => sync_info = Some(m),
                HkbNodeInternalStateInfoVisitor::Name(m) => name = Some(m),
                HkbNodeInternalStateInfoVisitor::InternalState(m) => internal_state = Some(m),
                HkbNodeInternalStateInfoVisitor::NodeId(m) => node_id = Some(m),
                HkbNodeInternalStateInfoVisitor::HasActivateBeenCalled(m) => has_activate_been_called = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            sync_info: sync_info.unwrap_or_default(),
            name: name.unwrap_or_default().into_inner(),
            internal_state: internal_state.unwrap_or_default().into_inner(),
            node_id: node_id.unwrap_or_default().into_inner(),
            has_activate_been_called: has_activate_been_called.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbNodeInternalStateInfo<'a>> for Vec<HkbNodeInternalStateInfoVisitor<'a>> {
    fn from(data: &HkbNodeInternalStateInfo<'a>) -> Self {
        vec![
            HkbNodeInternalStateInfoVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbNodeInternalStateInfoVisitor::ReferenceCount(data.reference_count.into()),
            HkbNodeInternalStateInfoVisitor::SyncInfo(data.sync_info.clone()),
            HkbNodeInternalStateInfoVisitor::Name(data.name.clone().into()),
            HkbNodeInternalStateInfoVisitor::InternalState(data.internal_state.clone().into()),
            HkbNodeInternalStateInfoVisitor::NodeId(data.node_id.into()),
            HkbNodeInternalStateInfoVisitor::HasActivateBeenCalled(data.has_activate_been_called.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbNodeInternalStateInfo<'de> {
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
enum HkbNodeInternalStateInfoVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "syncInfo")]
    SyncInfo(SingleClass<HkbGeneratorSyncInfo>),
    /// Visitor fields
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "internalState")]
    InternalState(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "nodeId")]
    NodeId(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "hasActivateBeenCalled")]
    HasActivateBeenCalled(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbNodeInternalStateInfoVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("syncInfo" => SyncInfo(SingleClass<HkbGeneratorSyncInfo>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("internalState" => InternalState(Primitive<Cow<'de, str>>)),
    ("nodeId" => NodeId(Primitive<i16>)),
    ("hasActivateBeenCalled" => HasActivateBeenCalled(Primitive<bool>)),
}
