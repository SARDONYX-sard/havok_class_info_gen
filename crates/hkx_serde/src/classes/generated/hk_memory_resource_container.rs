//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkMemoryResourceContainer`
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

/// `hkMemoryResourceContainer`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 40
/// -    vtable: true
/// -    parent: `hkResourceContainer`/`0x4e94146`
/// - signature: `0x4762f92a`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkMemoryResourceContainer<'a> {
    // C++ Parent class(`hkResourceContainer` => parent: `hkResourceBase`) has no fields
    //
    // C++ Parent class(`hkResourceBase` => parent: `hkReferencedObject`) has no fields
    //
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
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub name: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"parent"`
    /// -   type: `struct hkMemoryResourceContainer*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub parent: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"resourceHandles"`
    /// -   type: `hkArray<hkMemoryResourceHandle*>`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub resource_handles: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"children"`
    /// -   type: `hkArray<hkMemoryResourceContainer*>`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub children: HkArrayRef<Cow<'a, str>>,
}

impl Serialize for HkMemoryResourceContainer<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkMemoryResourceContainerVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkMemoryResourceContainer<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkMemoryResourceContainerVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkMemoryResourceContainerVisitor<'a>>> for HkMemoryResourceContainer<'a> {
    fn from(_values: Vec<HkMemoryResourceContainerVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut name = None;
            let mut parent = None;
            let mut resource_handles = None;
            let mut children = None;


        for _value in _values {
            match _value {
                HkMemoryResourceContainerVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkMemoryResourceContainerVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkMemoryResourceContainerVisitor::Name(m) => name = Some(m),
                HkMemoryResourceContainerVisitor::Parent(m) => parent = Some(m),
                HkMemoryResourceContainerVisitor::ResourceHandles(m) => resource_handles = Some(m),
                HkMemoryResourceContainerVisitor::Children(m) => children = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            name: name.unwrap_or_default().into_inner(),
            parent: parent.unwrap_or_default().into_inner(),
            resource_handles: resource_handles.unwrap_or_default(),
            children: children.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkMemoryResourceContainer<'a>> for Vec<HkMemoryResourceContainerVisitor<'a>> {
    fn from(data: &HkMemoryResourceContainer<'a>) -> Self {
        vec![
            HkMemoryResourceContainerVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkMemoryResourceContainerVisitor::ReferenceCount(data.reference_count.into()),
            HkMemoryResourceContainerVisitor::Name(data.name.clone().into()),
            HkMemoryResourceContainerVisitor::Parent(data.parent.clone().into()),
            HkMemoryResourceContainerVisitor::ResourceHandles(data.resource_handles.clone()),
            HkMemoryResourceContainerVisitor::Children(data.children.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkMemoryResourceContainer<'de> {
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
enum HkMemoryResourceContainerVisitor<'a> {
    // C++ Parent class(`hkResourceContainer` => parent: `hkResourceBase`) has no fields
    //
    // C++ Parent class(`hkResourceBase` => parent: `hkReferencedObject`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "parent", skip_serializing)]
    Parent(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "resourceHandles")]
    ResourceHandles(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "children")]
    Children(HkArrayRef<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMemoryResourceContainerVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("parent" => Parent(Primitive<Cow<'de, str>>)),
    ("resourceHandles" => ResourceHandles(HkArrayRef<Cow<'de, str>>)),
    ("children" => Children(HkArrayRef<Cow<'de, str>>)),
}
