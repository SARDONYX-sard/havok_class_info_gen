//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkAlignSceneToNodeOptions`
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

/// `hkAlignSceneToNodeOptions`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 24
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x207cb01`
/// -   version: 2
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkAlignSceneToNodeOptions<'a> {
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
    /// -   name:`"invert"`
    /// -   type: `hkBool`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub invert: bool,
    /// # C++ Class Fields Info
    /// -   name:`"transformPositionX"`
    /// -   type: `hkBool`
    /// - offset: 9
    /// -  flags: `FLAGS_NONE`
    pub transform_position_x: bool,
    /// # C++ Class Fields Info
    /// -   name:`"transformPositionY"`
    /// -   type: `hkBool`
    /// - offset: 10
    /// -  flags: `FLAGS_NONE`
    pub transform_position_y: bool,
    /// # C++ Class Fields Info
    /// -   name:`"transformPositionZ"`
    /// -   type: `hkBool`
    /// - offset: 11
    /// -  flags: `FLAGS_NONE`
    pub transform_position_z: bool,
    /// # C++ Class Fields Info
    /// -   name:`"transformRotation"`
    /// -   type: `hkBool`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub transform_rotation: bool,
    /// # C++ Class Fields Info
    /// -   name:`"transformScale"`
    /// -   type: `hkBool`
    /// - offset: 13
    /// -  flags: `FLAGS_NONE`
    pub transform_scale: bool,
    /// # C++ Class Fields Info
    /// -   name:`"transformSkew"`
    /// -   type: `hkBool`
    /// - offset: 14
    /// -  flags: `FLAGS_NONE`
    pub transform_skew: bool,
    /// # C++ Class Fields Info
    /// -   name:`"keyframe"`
    /// -   type: `hkInt32`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub keyframe: i32,
    /// # C++ Class Fields Info
    /// -   name:`"nodeName"`
    /// -   type: `hkStringPtr`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub node_name: Cow<'a, str>,
}

impl Serialize for HkAlignSceneToNodeOptions<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkAlignSceneToNodeOptionsVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkAlignSceneToNodeOptions<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkAlignSceneToNodeOptionsVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkAlignSceneToNodeOptionsVisitor<'a>>> for HkAlignSceneToNodeOptions<'a> {
    fn from(_values: Vec<HkAlignSceneToNodeOptionsVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut invert = None;
            let mut transform_position_x = None;
            let mut transform_position_y = None;
            let mut transform_position_z = None;
            let mut transform_rotation = None;
            let mut transform_scale = None;
            let mut transform_skew = None;
            let mut keyframe = None;
            let mut node_name = None;


        for _value in _values {
            match _value {
                HkAlignSceneToNodeOptionsVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkAlignSceneToNodeOptionsVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkAlignSceneToNodeOptionsVisitor::Invert(m) => invert = Some(m),
                HkAlignSceneToNodeOptionsVisitor::TransformPositionX(m) => transform_position_x = Some(m),
                HkAlignSceneToNodeOptionsVisitor::TransformPositionY(m) => transform_position_y = Some(m),
                HkAlignSceneToNodeOptionsVisitor::TransformPositionZ(m) => transform_position_z = Some(m),
                HkAlignSceneToNodeOptionsVisitor::TransformRotation(m) => transform_rotation = Some(m),
                HkAlignSceneToNodeOptionsVisitor::TransformScale(m) => transform_scale = Some(m),
                HkAlignSceneToNodeOptionsVisitor::TransformSkew(m) => transform_skew = Some(m),
                HkAlignSceneToNodeOptionsVisitor::Keyframe(m) => keyframe = Some(m),
                HkAlignSceneToNodeOptionsVisitor::NodeName(m) => node_name = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            invert: invert.unwrap_or_default().into_inner(),
            transform_position_x: transform_position_x.unwrap_or_default().into_inner(),
            transform_position_y: transform_position_y.unwrap_or_default().into_inner(),
            transform_position_z: transform_position_z.unwrap_or_default().into_inner(),
            transform_rotation: transform_rotation.unwrap_or_default().into_inner(),
            transform_scale: transform_scale.unwrap_or_default().into_inner(),
            transform_skew: transform_skew.unwrap_or_default().into_inner(),
            keyframe: keyframe.unwrap_or_default().into_inner(),
            node_name: node_name.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkAlignSceneToNodeOptions<'a>> for Vec<HkAlignSceneToNodeOptionsVisitor<'a>> {
    fn from(data: &HkAlignSceneToNodeOptions<'a>) -> Self {
        vec![
            HkAlignSceneToNodeOptionsVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkAlignSceneToNodeOptionsVisitor::ReferenceCount(data.reference_count.into()),
            HkAlignSceneToNodeOptionsVisitor::Invert(data.invert.into()),
            HkAlignSceneToNodeOptionsVisitor::TransformPositionX(data.transform_position_x.into()),
            HkAlignSceneToNodeOptionsVisitor::TransformPositionY(data.transform_position_y.into()),
            HkAlignSceneToNodeOptionsVisitor::TransformPositionZ(data.transform_position_z.into()),
            HkAlignSceneToNodeOptionsVisitor::TransformRotation(data.transform_rotation.into()),
            HkAlignSceneToNodeOptionsVisitor::TransformScale(data.transform_scale.into()),
            HkAlignSceneToNodeOptionsVisitor::TransformSkew(data.transform_skew.into()),
            HkAlignSceneToNodeOptionsVisitor::Keyframe(data.keyframe.into()),
            HkAlignSceneToNodeOptionsVisitor::NodeName(data.node_name.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkAlignSceneToNodeOptions<'de> {
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
enum HkAlignSceneToNodeOptionsVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "invert")]
    Invert(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "transformPositionX")]
    TransformPositionX(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "transformPositionY")]
    TransformPositionY(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "transformPositionZ")]
    TransformPositionZ(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "transformRotation")]
    TransformRotation(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "transformScale")]
    TransformScale(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "transformSkew")]
    TransformSkew(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "keyframe")]
    Keyframe(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "nodeName")]
    NodeName(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkAlignSceneToNodeOptionsVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("invert" => Invert(Primitive<bool>)),
    ("transformPositionX" => TransformPositionX(Primitive<bool>)),
    ("transformPositionY" => TransformPositionY(Primitive<bool>)),
    ("transformPositionZ" => TransformPositionZ(Primitive<bool>)),
    ("transformRotation" => TransformRotation(Primitive<bool>)),
    ("transformScale" => TransformScale(Primitive<bool>)),
    ("transformSkew" => TransformSkew(Primitive<bool>)),
    ("keyframe" => Keyframe(Primitive<i32>)),
    ("nodeName" => NodeName(Primitive<Cow<'de, str>>)),
}
