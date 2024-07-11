//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkSimpleLocalFrame`
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

/// `hkSimpleLocalFrame`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: true
/// -    parent: `hkLocalFrame`/`0xda8c7d7d`
/// - signature: `0xe758f63c`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkSimpleLocalFrame<'a> {
    // C++ Parent class(`hkLocalFrame` => parent: `hkReferencedObject`) has no fields
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
    /// -   name:`"transform"`
    /// -   type: `hkTransform`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub transform: Transform<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"children"`
    /// -   type: `hkArray<hkLocalFrame*>`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub children: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"parentFrame"`
    /// -   type: `struct hkLocalFrame*`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE|NOT_OWNED`
    pub parent_frame: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"group"`
    /// -   type: `struct hkLocalFrameGroup*`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    pub group: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    pub name: Cow<'a, str>,
}

impl Serialize for HkSimpleLocalFrame<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkSimpleLocalFrameVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkSimpleLocalFrame<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkSimpleLocalFrameVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkSimpleLocalFrameVisitor<'a>>> for HkSimpleLocalFrame<'a> {
    fn from(_values: Vec<HkSimpleLocalFrameVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut transform = None;
            let mut children = None;
            let mut parent_frame = None;
            let mut group = None;
            let mut name = None;


        for _value in _values {
            match _value {
                HkSimpleLocalFrameVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkSimpleLocalFrameVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkSimpleLocalFrameVisitor::Transform(m) => transform = Some(m),
                HkSimpleLocalFrameVisitor::Children(m) => children = Some(m),
                HkSimpleLocalFrameVisitor::ParentFrame(m) => parent_frame = Some(m),
                HkSimpleLocalFrameVisitor::Group(m) => group = Some(m),
                HkSimpleLocalFrameVisitor::Name(m) => name = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            transform: transform.unwrap_or_default().into_inner(),
            children: children.unwrap_or_default(),
            parent_frame: parent_frame.unwrap_or_default().into_inner(),
            group: group.unwrap_or_default().into_inner(),
            name: name.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkSimpleLocalFrame<'a>> for Vec<HkSimpleLocalFrameVisitor<'a>> {
    fn from(data: &HkSimpleLocalFrame<'a>) -> Self {
        vec![
            HkSimpleLocalFrameVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkSimpleLocalFrameVisitor::ReferenceCount(data.reference_count.into()),
            HkSimpleLocalFrameVisitor::Transform(data.transform.clone().into()),
            HkSimpleLocalFrameVisitor::Children(data.children.clone()),
            HkSimpleLocalFrameVisitor::ParentFrame(data.parent_frame.clone().into()),
            HkSimpleLocalFrameVisitor::Group(data.group.clone().into()),
            HkSimpleLocalFrameVisitor::Name(data.name.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkSimpleLocalFrame<'de> {
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
enum HkSimpleLocalFrameVisitor<'a> {
    // C++ Parent class(`hkLocalFrame` => parent: `hkReferencedObject`) has no fields
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
    #[serde(rename = "transform")]
    Transform(Primitive<Transform<f32>>),
    /// Visitor fields
    #[serde(rename = "children")]
    Children(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "parentFrame")]
    ParentFrame(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "group")]
    Group(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkSimpleLocalFrameVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("transform" => Transform(Primitive<Transform<f32>>)),
    ("children" => Children(HkArrayRef<Cow<'de, str>>)),
    ("parentFrame" => ParentFrame(Primitive<Cow<'de, str>>)),
    ("group" => Group(Primitive<Cow<'de, str>>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
}
