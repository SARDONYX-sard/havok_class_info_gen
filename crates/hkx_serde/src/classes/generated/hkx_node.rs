//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkxNode`
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

/// `hkxNode`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 72
/// -    vtable: true
/// -    parent: `hkxAttributeHolder`/`0x7468cc44`
/// - signature: `0x5a218502`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkxNode<'a> {
    /// # C++ Parent class(`hkxAttributeHolder` => parent: `hkReferencedObject`) field Info
    /// -   name:`"attributeGroups"`
    /// -   type: `hkArray<struct hkxAttributeGroup>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub attribute_groups: HkArrayClass<HkxAttributeGroup<'a>>,

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
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub name: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"object"`
    /// -   type: `struct hkReferencedObject*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub object: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"keyFrames"`
    /// -   type: `hkArray<hkMatrix4>`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub key_frames: HkArrayMatrix4<Matrix4<f32>>,
    /// # C++ Class Fields Info
    /// -   name:`"children"`
    /// -   type: `hkArray<hkxNode*>`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub children: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"annotations"`
    /// -   type: `hkArray<struct hkxNodeAnnotationData>`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub annotations: HkArrayClass<HkxNodeAnnotationData<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"userProperties"`
    /// -   type: `hkStringPtr`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub user_properties: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"selected"`
    /// -   type: `hkBool`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    pub selected: bool,
}

impl Serialize for HkxNode<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkxNodeVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkxNode<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkxNodeVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkxNodeVisitor<'a>>> for HkxNode<'a> {
    fn from(_values: Vec<HkxNodeVisitor<'a>>) -> Self {
            let mut attribute_groups = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut name = None;
            let mut object = None;
            let mut key_frames = None;
            let mut children = None;
            let mut annotations = None;
            let mut user_properties = None;
            let mut selected = None;


        for _value in _values {
            match _value {
                HkxNodeVisitor::AttributeGroups(m) => attribute_groups = Some(m),
                HkxNodeVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkxNodeVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkxNodeVisitor::Name(m) => name = Some(m),
                HkxNodeVisitor::Object(m) => object = Some(m),
                HkxNodeVisitor::KeyFrames(m) => key_frames = Some(m),
                HkxNodeVisitor::Children(m) => children = Some(m),
                HkxNodeVisitor::Annotations(m) => annotations = Some(m),
                HkxNodeVisitor::UserProperties(m) => user_properties = Some(m),
                HkxNodeVisitor::Selected(m) => selected = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            attribute_groups: attribute_groups.unwrap_or_default(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            name: name.unwrap_or_default().into_inner(),
            object: object.unwrap_or_default().into_inner(),
            key_frames: key_frames.unwrap_or_default(),
            children: children.unwrap_or_default(),
            annotations: annotations.unwrap_or_default(),
            user_properties: user_properties.unwrap_or_default().into_inner(),
            selected: selected.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkxNode<'a>> for Vec<HkxNodeVisitor<'a>> {
    fn from(data: &HkxNode<'a>) -> Self {
        vec![
            HkxNodeVisitor::AttributeGroups(data.attribute_groups.clone()),
            HkxNodeVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkxNodeVisitor::ReferenceCount(data.reference_count.into()),
            HkxNodeVisitor::Name(data.name.clone().into()),
            HkxNodeVisitor::Object(data.object.clone().into()),
            HkxNodeVisitor::KeyFrames(data.key_frames.clone()),
            HkxNodeVisitor::Children(data.children.clone()),
            HkxNodeVisitor::Annotations(data.annotations.clone()),
            HkxNodeVisitor::UserProperties(data.user_properties.clone().into()),
            HkxNodeVisitor::Selected(data.selected.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkxNode<'de> {
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
enum HkxNodeVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "attributeGroups")]
    AttributeGroups(HkArrayClass<HkxAttributeGroup<'a>>),

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
    #[serde(rename = "object")]
    Object(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "keyFrames")]
    KeyFrames(HkArrayMatrix4<Matrix4<f32>>),
    /// Visitor fields
    #[serde(rename = "children")]
    Children(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "annotations")]
    Annotations(HkArrayClass<HkxNodeAnnotationData<'a>>),
    /// Visitor fields
    #[serde(rename = "userProperties")]
    UserProperties(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "selected")]
    Selected(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxNodeVisitor<'de>, "@name",
    ("attributeGroups" => AttributeGroups(HkArrayClass<HkxAttributeGroup<'de>>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("object" => Object(Primitive<Cow<'de, str>>)),
    ("keyFrames" => KeyFrames(HkArrayMatrix4<Matrix4<f32>>)),
    ("children" => Children(HkArrayRef<Cow<'de, str>>)),
    ("annotations" => Annotations(HkArrayClass<HkxNodeAnnotationData<'de>>)),
    ("userProperties" => UserProperties(Primitive<Cow<'de, str>>)),
    ("selected" => Selected(Primitive<bool>)),
}
