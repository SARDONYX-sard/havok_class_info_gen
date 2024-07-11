//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbCombineTransformsModifier`
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

/// `hkbCombineTransformsModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 160
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0xfd1f0b79`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbCombineTransformsModifier<'a> {
    /// # C++ Parent class(`hkbModifier` => parent: `hkbNode`) field Info
    /// -   name:`"enable"`
    /// -   type: `hkBool`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub enable: bool,
    /// # C++ Parent class(`hkbModifier` => parent: `hkbNode`) field Info
    /// -   name:`"padModifier"`
    /// -   type: `hkBool[3]`
    /// - offset: 41
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub pad_modifier: CStyleArray<[bool; 3]>,

    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub user_data: usize,
    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub name: Cow<'a, str>,
    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"id"`
    /// -   type: `hkInt16`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub id: i16,
    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"cloneState"`
    /// -   type: `enum unknown`
    /// - offset: 38
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub clone_state: (),
    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"padNode"`
    /// -   type: `hkBool[1]`
    /// - offset: 39
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub pad_node: CStyleArray<[bool; 1]>,

    /// # C++ Parent class(`hkbBindable` => parent: `hkReferencedObject`) field Info
    /// -   name:`"variableBindingSet"`
    /// -   type: `struct hkbVariableBindingSet*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub variable_binding_set: Cow<'a, str>,
    /// # C++ Parent class(`hkbBindable` => parent: `hkReferencedObject`) field Info
    /// -   name:`"cachedBindables"`
    /// -   type: `hkArray<void>`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub cached_bindables: HkArrayRef<()>,
    /// # C++ Parent class(`hkbBindable` => parent: `hkReferencedObject`) field Info
    /// -   name:`"areBindablesCached"`
    /// -   type: `hkBool`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub are_bindables_cached: bool,

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
    /// -   name:`"translationOut"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub translation_out: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"rotationOut"`
    /// -   type: `hkQuaternion`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub rotation_out: Quaternion<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"leftTranslation"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub left_translation: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"leftRotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    pub left_rotation: Quaternion<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"rightTranslation"`
    /// -   type: `hkVector4`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    pub right_translation: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"rightRotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    pub right_rotation: Quaternion<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"invertLeftTransform"`
    /// -   type: `hkBool`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    pub invert_left_transform: bool,
    /// # C++ Class Fields Info
    /// -   name:`"invertRightTransform"`
    /// -   type: `hkBool`
    /// - offset: 145
    /// -  flags: `FLAGS_NONE`
    pub invert_right_transform: bool,
    /// # C++ Class Fields Info
    /// -   name:`"invertResult"`
    /// -   type: `hkBool`
    /// - offset: 146
    /// -  flags: `FLAGS_NONE`
    pub invert_result: bool,
}

impl Serialize for HkbCombineTransformsModifier<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbCombineTransformsModifierVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbCombineTransformsModifier<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbCombineTransformsModifierVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbCombineTransformsModifierVisitor<'a>>> for HkbCombineTransformsModifier<'a> {
    fn from(_values: Vec<HkbCombineTransformsModifierVisitor<'a>>) -> Self {
            let mut enable = None;
            let mut pad_modifier = None;
            let mut user_data = None;
            let mut name = None;
            let mut id = None;
            let mut clone_state = None;
            let mut pad_node = None;
            let mut variable_binding_set = None;
            let mut cached_bindables = None;
            let mut are_bindables_cached = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut translation_out = None;
            let mut rotation_out = None;
            let mut left_translation = None;
            let mut left_rotation = None;
            let mut right_translation = None;
            let mut right_rotation = None;
            let mut invert_left_transform = None;
            let mut invert_right_transform = None;
            let mut invert_result = None;


        for _value in _values {
            match _value {
                HkbCombineTransformsModifierVisitor::Enable(m) => enable = Some(m),
                HkbCombineTransformsModifierVisitor::PadModifier(m) => pad_modifier = Some(m),
                HkbCombineTransformsModifierVisitor::UserData(m) => user_data = Some(m),
                HkbCombineTransformsModifierVisitor::Name(m) => name = Some(m),
                HkbCombineTransformsModifierVisitor::Id(m) => id = Some(m),
                HkbCombineTransformsModifierVisitor::CloneState(m) => clone_state = Some(m),
                HkbCombineTransformsModifierVisitor::PadNode(m) => pad_node = Some(m),
                HkbCombineTransformsModifierVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                HkbCombineTransformsModifierVisitor::CachedBindables(m) => cached_bindables = Some(m),
                HkbCombineTransformsModifierVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                HkbCombineTransformsModifierVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbCombineTransformsModifierVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbCombineTransformsModifierVisitor::TranslationOut(m) => translation_out = Some(m),
                HkbCombineTransformsModifierVisitor::RotationOut(m) => rotation_out = Some(m),
                HkbCombineTransformsModifierVisitor::LeftTranslation(m) => left_translation = Some(m),
                HkbCombineTransformsModifierVisitor::LeftRotation(m) => left_rotation = Some(m),
                HkbCombineTransformsModifierVisitor::RightTranslation(m) => right_translation = Some(m),
                HkbCombineTransformsModifierVisitor::RightRotation(m) => right_rotation = Some(m),
                HkbCombineTransformsModifierVisitor::InvertLeftTransform(m) => invert_left_transform = Some(m),
                HkbCombineTransformsModifierVisitor::InvertRightTransform(m) => invert_right_transform = Some(m),
                HkbCombineTransformsModifierVisitor::InvertResult(m) => invert_result = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            enable: enable.unwrap_or_default().into_inner(),
            pad_modifier: pad_modifier.unwrap_or_default(),
            user_data: user_data.unwrap_or_default().into_inner(),
            name: name.unwrap_or_default().into_inner(),
            id: id.unwrap_or_default().into_inner(),
            clone_state: clone_state.unwrap_or_default().into_inner(),
            pad_node: pad_node.unwrap_or_default(),
            variable_binding_set: variable_binding_set.unwrap_or_default().into_inner(),
            cached_bindables: cached_bindables.unwrap_or_default(),
            are_bindables_cached: are_bindables_cached.unwrap_or_default().into_inner(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            translation_out: translation_out.unwrap_or_default().into_inner(),
            rotation_out: rotation_out.unwrap_or_default().into_inner(),
            left_translation: left_translation.unwrap_or_default().into_inner(),
            left_rotation: left_rotation.unwrap_or_default().into_inner(),
            right_translation: right_translation.unwrap_or_default().into_inner(),
            right_rotation: right_rotation.unwrap_or_default().into_inner(),
            invert_left_transform: invert_left_transform.unwrap_or_default().into_inner(),
            invert_right_transform: invert_right_transform.unwrap_or_default().into_inner(),
            invert_result: invert_result.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbCombineTransformsModifier<'a>> for Vec<HkbCombineTransformsModifierVisitor<'a>> {
    fn from(data: &HkbCombineTransformsModifier<'a>) -> Self {
        vec![
            HkbCombineTransformsModifierVisitor::Enable(data.enable.into()),
            HkbCombineTransformsModifierVisitor::PadModifier(data.pad_modifier.clone()),
            HkbCombineTransformsModifierVisitor::UserData(data.user_data.into()),
            HkbCombineTransformsModifierVisitor::Name(data.name.clone().into()),
            HkbCombineTransformsModifierVisitor::Id(data.id.into()),
            HkbCombineTransformsModifierVisitor::CloneState(data.clone_state.into()),
            HkbCombineTransformsModifierVisitor::PadNode(data.pad_node.clone()),
            HkbCombineTransformsModifierVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            HkbCombineTransformsModifierVisitor::CachedBindables(data.cached_bindables.clone()),
            HkbCombineTransformsModifierVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            HkbCombineTransformsModifierVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbCombineTransformsModifierVisitor::ReferenceCount(data.reference_count.into()),
            HkbCombineTransformsModifierVisitor::TranslationOut(data.translation_out.into()),
            HkbCombineTransformsModifierVisitor::RotationOut(data.rotation_out.clone().into()),
            HkbCombineTransformsModifierVisitor::LeftTranslation(data.left_translation.into()),
            HkbCombineTransformsModifierVisitor::LeftRotation(data.left_rotation.clone().into()),
            HkbCombineTransformsModifierVisitor::RightTranslation(data.right_translation.into()),
            HkbCombineTransformsModifierVisitor::RightRotation(data.right_rotation.clone().into()),
            HkbCombineTransformsModifierVisitor::InvertLeftTransform(data.invert_left_transform.into()),
            HkbCombineTransformsModifierVisitor::InvertRightTransform(data.invert_right_transform.into()),
            HkbCombineTransformsModifierVisitor::InvertResult(data.invert_result.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbCombineTransformsModifier<'de> {
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
enum HkbCombineTransformsModifierVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "enable")]
    Enable(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "padModifier", skip_serializing)]
    PadModifier(CStyleArray<[bool; 3]>),

    /// Visitor fields
    #[serde(rename = "userData")]
    UserData(Primitive<usize>),
    /// Visitor fields
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "id", skip_serializing)]
    Id(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "cloneState", skip_serializing)]
    CloneState(Primitive<()>),
    /// Visitor fields
    #[serde(rename = "padNode", skip_serializing)]
    PadNode(CStyleArray<[bool; 1]>),

    /// Visitor fields
    #[serde(rename = "variableBindingSet")]
    VariableBindingSet(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "cachedBindables", skip_serializing)]
    CachedBindables(HkArrayRef<()>),
    /// Visitor fields
    #[serde(rename = "areBindablesCached", skip_serializing)]
    AreBindablesCached(Primitive<bool>),

    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "translationOut")]
    TranslationOut(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "rotationOut")]
    RotationOut(Primitive<Quaternion<f32>>),
    /// Visitor fields
    #[serde(rename = "leftTranslation")]
    LeftTranslation(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "leftRotation")]
    LeftRotation(Primitive<Quaternion<f32>>),
    /// Visitor fields
    #[serde(rename = "rightTranslation")]
    RightTranslation(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "rightRotation")]
    RightRotation(Primitive<Quaternion<f32>>),
    /// Visitor fields
    #[serde(rename = "invertLeftTransform")]
    InvertLeftTransform(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "invertRightTransform")]
    InvertRightTransform(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "invertResult")]
    InvertResult(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbCombineTransformsModifierVisitor<'de>, "@name",
    ("enable" => Enable(Primitive<bool>)),
    ("padModifier" => PadModifier(CStyleArray<[bool; 3]>)),
    ("userData" => UserData(Primitive<usize>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("id" => Id(Primitive<i16>)),
    ("cloneState" => CloneState(Primitive<()>)),
    ("padNode" => PadNode(CStyleArray<[bool; 1]>)),
    ("variableBindingSet" => VariableBindingSet(Primitive<Cow<'de, str>>)),
    ("cachedBindables" => CachedBindables(HkArrayRef<()>)),
    ("areBindablesCached" => AreBindablesCached(Primitive<bool>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("translationOut" => TranslationOut(Primitive<Vector4<f32>>)),
    ("rotationOut" => RotationOut(Primitive<Quaternion<f32>>)),
    ("leftTranslation" => LeftTranslation(Primitive<Vector4<f32>>)),
    ("leftRotation" => LeftRotation(Primitive<Quaternion<f32>>)),
    ("rightTranslation" => RightTranslation(Primitive<Vector4<f32>>)),
    ("rightRotation" => RightRotation(Primitive<Quaternion<f32>>)),
    ("invertLeftTransform" => InvertLeftTransform(Primitive<bool>)),
    ("invertRightTransform" => InvertRightTransform(Primitive<bool>)),
    ("invertResult" => InvertResult(Primitive<bool>)),
}
