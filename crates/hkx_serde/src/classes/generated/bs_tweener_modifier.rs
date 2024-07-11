//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `BSTweenerModifier`
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

/// `BSTweenerModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 176
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0xd2d9a04`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BsTweenerModifier<'a> {
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
    /// -   name:`"tweenPosition"`
    /// -   type: `hkBool`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub tween_position: bool,
    /// # C++ Class Fields Info
    /// -   name:`"tweenRotation"`
    /// -   type: `hkBool`
    /// - offset: 45
    /// -  flags: `FLAGS_NONE`
    pub tween_rotation: bool,
    /// # C++ Class Fields Info
    /// -   name:`"useTweenDuration"`
    /// -   type: `hkBool`
    /// - offset: 46
    /// -  flags: `FLAGS_NONE`
    pub use_tween_duration: bool,
    /// # C++ Class Fields Info
    /// -   name:`"tweenDuration"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub tween_duration: f32,
    /// # C++ Class Fields Info
    /// -   name:`"targetPosition"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub target_position: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"targetRotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub target_rotation: Quaternion<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"duration"`
    /// -   type: `hkReal`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub duration: f32,
    /// # C++ Class Fields Info
    /// -   name:`"startTransform"`
    /// -   type: `hkQsTransform`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub start_transform: QsTransform<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"time"`
    /// -   type: `hkReal`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub time: f32,
}

impl Serialize for BsTweenerModifier<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<BsTweenerModifierVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for BsTweenerModifier<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<BsTweenerModifierVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<BsTweenerModifierVisitor<'a>>> for BsTweenerModifier<'a> {
    fn from(_values: Vec<BsTweenerModifierVisitor<'a>>) -> Self {
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
            let mut tween_position = None;
            let mut tween_rotation = None;
            let mut use_tween_duration = None;
            let mut tween_duration = None;
            let mut target_position = None;
            let mut target_rotation = None;
            let mut duration = None;
            let mut start_transform = None;
            let mut time = None;


        for _value in _values {
            match _value {
                BsTweenerModifierVisitor::Enable(m) => enable = Some(m),
                BsTweenerModifierVisitor::PadModifier(m) => pad_modifier = Some(m),
                BsTweenerModifierVisitor::UserData(m) => user_data = Some(m),
                BsTweenerModifierVisitor::Name(m) => name = Some(m),
                BsTweenerModifierVisitor::Id(m) => id = Some(m),
                BsTweenerModifierVisitor::CloneState(m) => clone_state = Some(m),
                BsTweenerModifierVisitor::PadNode(m) => pad_node = Some(m),
                BsTweenerModifierVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                BsTweenerModifierVisitor::CachedBindables(m) => cached_bindables = Some(m),
                BsTweenerModifierVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                BsTweenerModifierVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                BsTweenerModifierVisitor::ReferenceCount(m) => reference_count = Some(m),
                BsTweenerModifierVisitor::TweenPosition(m) => tween_position = Some(m),
                BsTweenerModifierVisitor::TweenRotation(m) => tween_rotation = Some(m),
                BsTweenerModifierVisitor::UseTweenDuration(m) => use_tween_duration = Some(m),
                BsTweenerModifierVisitor::TweenDuration(m) => tween_duration = Some(m),
                BsTweenerModifierVisitor::TargetPosition(m) => target_position = Some(m),
                BsTweenerModifierVisitor::TargetRotation(m) => target_rotation = Some(m),
                BsTweenerModifierVisitor::Duration(m) => duration = Some(m),
                BsTweenerModifierVisitor::StartTransform(m) => start_transform = Some(m),
                BsTweenerModifierVisitor::Time(m) => time = Some(m),

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
            tween_position: tween_position.unwrap_or_default().into_inner(),
            tween_rotation: tween_rotation.unwrap_or_default().into_inner(),
            use_tween_duration: use_tween_duration.unwrap_or_default().into_inner(),
            tween_duration: tween_duration.unwrap_or_default().into_inner(),
            target_position: target_position.unwrap_or_default().into_inner(),
            target_rotation: target_rotation.unwrap_or_default().into_inner(),
            duration: duration.unwrap_or_default().into_inner(),
            start_transform: start_transform.unwrap_or_default().into_inner(),
            time: time.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&BsTweenerModifier<'a>> for Vec<BsTweenerModifierVisitor<'a>> {
    fn from(data: &BsTweenerModifier<'a>) -> Self {
        vec![
            BsTweenerModifierVisitor::Enable(data.enable.into()),
            BsTweenerModifierVisitor::PadModifier(data.pad_modifier.clone()),
            BsTweenerModifierVisitor::UserData(data.user_data.into()),
            BsTweenerModifierVisitor::Name(data.name.clone().into()),
            BsTweenerModifierVisitor::Id(data.id.into()),
            BsTweenerModifierVisitor::CloneState(data.clone_state.into()),
            BsTweenerModifierVisitor::PadNode(data.pad_node.clone()),
            BsTweenerModifierVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            BsTweenerModifierVisitor::CachedBindables(data.cached_bindables.clone()),
            BsTweenerModifierVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            BsTweenerModifierVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            BsTweenerModifierVisitor::ReferenceCount(data.reference_count.into()),
            BsTweenerModifierVisitor::TweenPosition(data.tween_position.into()),
            BsTweenerModifierVisitor::TweenRotation(data.tween_rotation.into()),
            BsTweenerModifierVisitor::UseTweenDuration(data.use_tween_duration.into()),
            BsTweenerModifierVisitor::TweenDuration(data.tween_duration.into()),
            BsTweenerModifierVisitor::TargetPosition(data.target_position.into()),
            BsTweenerModifierVisitor::TargetRotation(data.target_rotation.clone().into()),
            BsTweenerModifierVisitor::Duration(data.duration.into()),
            BsTweenerModifierVisitor::StartTransform(data.start_transform.clone().into()),
            BsTweenerModifierVisitor::Time(data.time.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for BsTweenerModifier<'de> {
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
enum BsTweenerModifierVisitor<'a> {
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
    #[serde(rename = "tweenPosition")]
    TweenPosition(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "tweenRotation")]
    TweenRotation(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "useTweenDuration")]
    UseTweenDuration(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "tweenDuration")]
    TweenDuration(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "targetPosition")]
    TargetPosition(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "targetRotation")]
    TargetRotation(Primitive<Quaternion<f32>>),
    /// Visitor fields
    #[serde(rename = "duration", skip_serializing)]
    Duration(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "startTransform", skip_serializing)]
    StartTransform(Primitive<QsTransform<f32>>),
    /// Visitor fields
    #[serde(rename = "time", skip_serializing)]
    Time(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BsTweenerModifierVisitor<'de>, "@name",
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
    ("tweenPosition" => TweenPosition(Primitive<bool>)),
    ("tweenRotation" => TweenRotation(Primitive<bool>)),
    ("useTweenDuration" => UseTweenDuration(Primitive<bool>)),
    ("tweenDuration" => TweenDuration(Primitive<f32>)),
    ("targetPosition" => TargetPosition(Primitive<Vector4<f32>>)),
    ("targetRotation" => TargetRotation(Primitive<Quaternion<f32>>)),
    ("duration" => Duration(Primitive<f32>)),
    ("startTransform" => StartTransform(Primitive<QsTransform<f32>>)),
    ("time" => Time(Primitive<f32>)),
}
