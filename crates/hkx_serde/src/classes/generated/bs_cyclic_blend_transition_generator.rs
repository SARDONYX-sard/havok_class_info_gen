//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `BSCyclicBlendTransitionGenerator`
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

/// `BSCyclicBlendTransitionGenerator`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: true
/// -    parent: `hkbGenerator`/`0xd68aefc`
/// - signature: `0x5119eb06`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BsCyclicBlendTransitionGenerator<'a> {
    // C++ Parent class(`hkbGenerator` => parent: `hkbNode`) has no fields
    //
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
    /// -   name:`"pBlenderGenerator"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE|ALIGN16`
    pub p_blender_generator: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"EventToFreezeBlendValue"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub event_to_freeze_blend_value: SingleClass<HkbEventProperty<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"EventToCrossBlend"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    pub event_to_cross_blend: SingleClass<HkbEventProperty<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"fBlendParameter"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    pub f_blend_parameter: f32,
    /// # C++ Class Fields Info
    /// -   name:`"fTransitionDuration"`
    /// -   type: `hkReal`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    pub f_transition_duration: f32,
    /// # C++ Class Fields Info
    /// -   name:`"eBlendCurve"`
    /// -   type: `enum BlendCurve`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    pub e_blend_curve: BlendCurve,
    /// # C++ Class Fields Info
    /// -   name:`"pTransitionBlenderGenerator"`
    /// -   type: `void*`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE|ALIGN16|SERIALIZE_IGNORED`
    pub p_transition_blender_generator: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"pTransitionEffect"`
    /// -   type: `void*`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE|ALIGN16|SERIALIZE_IGNORED`
    pub p_transition_effect: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"currentMode"`
    /// -   type: `enum unknown`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub current_mode: (),
}

impl Serialize for BsCyclicBlendTransitionGenerator<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<BsCyclicBlendTransitionGeneratorVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for BsCyclicBlendTransitionGenerator<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<BsCyclicBlendTransitionGeneratorVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<BsCyclicBlendTransitionGeneratorVisitor<'a>>> for BsCyclicBlendTransitionGenerator<'a> {
    fn from(_values: Vec<BsCyclicBlendTransitionGeneratorVisitor<'a>>) -> Self {
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
            let mut p_blender_generator = None;
            let mut event_to_freeze_blend_value = None;
            let mut event_to_cross_blend = None;
            let mut f_blend_parameter = None;
            let mut f_transition_duration = None;
            let mut e_blend_curve = None;
            let mut p_transition_blender_generator = None;
            let mut p_transition_effect = None;
            let mut current_mode = None;


        for _value in _values {
            match _value {
                BsCyclicBlendTransitionGeneratorVisitor::UserData(m) => user_data = Some(m),
                BsCyclicBlendTransitionGeneratorVisitor::Name(m) => name = Some(m),
                BsCyclicBlendTransitionGeneratorVisitor::Id(m) => id = Some(m),
                BsCyclicBlendTransitionGeneratorVisitor::CloneState(m) => clone_state = Some(m),
                BsCyclicBlendTransitionGeneratorVisitor::PadNode(m) => pad_node = Some(m),
                BsCyclicBlendTransitionGeneratorVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                BsCyclicBlendTransitionGeneratorVisitor::CachedBindables(m) => cached_bindables = Some(m),
                BsCyclicBlendTransitionGeneratorVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                BsCyclicBlendTransitionGeneratorVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                BsCyclicBlendTransitionGeneratorVisitor::ReferenceCount(m) => reference_count = Some(m),
                BsCyclicBlendTransitionGeneratorVisitor::PBlenderGenerator(m) => p_blender_generator = Some(m),
                BsCyclicBlendTransitionGeneratorVisitor::EventToFreezeBlendValue(m) => event_to_freeze_blend_value = Some(m),
                BsCyclicBlendTransitionGeneratorVisitor::EventToCrossBlend(m) => event_to_cross_blend = Some(m),
                BsCyclicBlendTransitionGeneratorVisitor::FBlendParameter(m) => f_blend_parameter = Some(m),
                BsCyclicBlendTransitionGeneratorVisitor::FTransitionDuration(m) => f_transition_duration = Some(m),
                BsCyclicBlendTransitionGeneratorVisitor::EBlendCurve(m) => e_blend_curve = Some(m),
                BsCyclicBlendTransitionGeneratorVisitor::PTransitionBlenderGenerator(m) => p_transition_blender_generator = Some(m),
                BsCyclicBlendTransitionGeneratorVisitor::PTransitionEffect(m) => p_transition_effect = Some(m),
                BsCyclicBlendTransitionGeneratorVisitor::CurrentMode(m) => current_mode = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
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
            p_blender_generator: p_blender_generator.unwrap_or_default().into_inner(),
            event_to_freeze_blend_value: event_to_freeze_blend_value.unwrap_or_default(),
            event_to_cross_blend: event_to_cross_blend.unwrap_or_default(),
            f_blend_parameter: f_blend_parameter.unwrap_or_default().into_inner(),
            f_transition_duration: f_transition_duration.unwrap_or_default().into_inner(),
            e_blend_curve: e_blend_curve.unwrap_or_default().into_inner(),
            p_transition_blender_generator: p_transition_blender_generator.unwrap_or_default().into_inner(),
            p_transition_effect: p_transition_effect.unwrap_or_default().into_inner(),
            current_mode: current_mode.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&BsCyclicBlendTransitionGenerator<'a>> for Vec<BsCyclicBlendTransitionGeneratorVisitor<'a>> {
    fn from(data: &BsCyclicBlendTransitionGenerator<'a>) -> Self {
        vec![
            BsCyclicBlendTransitionGeneratorVisitor::UserData(data.user_data.into()),
            BsCyclicBlendTransitionGeneratorVisitor::Name(data.name.clone().into()),
            BsCyclicBlendTransitionGeneratorVisitor::Id(data.id.into()),
            BsCyclicBlendTransitionGeneratorVisitor::CloneState(data.clone_state.into()),
            BsCyclicBlendTransitionGeneratorVisitor::PadNode(data.pad_node.clone()),
            BsCyclicBlendTransitionGeneratorVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            BsCyclicBlendTransitionGeneratorVisitor::CachedBindables(data.cached_bindables.clone()),
            BsCyclicBlendTransitionGeneratorVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            BsCyclicBlendTransitionGeneratorVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            BsCyclicBlendTransitionGeneratorVisitor::ReferenceCount(data.reference_count.into()),
            BsCyclicBlendTransitionGeneratorVisitor::PBlenderGenerator(data.p_blender_generator.clone().into()),
            BsCyclicBlendTransitionGeneratorVisitor::EventToFreezeBlendValue(data.event_to_freeze_blend_value.clone()),
            BsCyclicBlendTransitionGeneratorVisitor::EventToCrossBlend(data.event_to_cross_blend.clone()),
            BsCyclicBlendTransitionGeneratorVisitor::FBlendParameter(data.f_blend_parameter.into()),
            BsCyclicBlendTransitionGeneratorVisitor::FTransitionDuration(data.f_transition_duration.into()),
            BsCyclicBlendTransitionGeneratorVisitor::EBlendCurve(data.e_blend_curve.clone().into()),
            BsCyclicBlendTransitionGeneratorVisitor::PTransitionBlenderGenerator(data.p_transition_blender_generator.clone().into()),
            BsCyclicBlendTransitionGeneratorVisitor::PTransitionEffect(data.p_transition_effect.clone().into()),
            BsCyclicBlendTransitionGeneratorVisitor::CurrentMode(data.current_mode.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for BsCyclicBlendTransitionGenerator<'de> {
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
enum BsCyclicBlendTransitionGeneratorVisitor<'a> {
    // C++ Parent class(`hkbGenerator` => parent: `hkbNode`) has no fields
    //
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
    #[serde(rename = "pBlenderGenerator")]
    PBlenderGenerator(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "EventToFreezeBlendValue")]
    EventToFreezeBlendValue(SingleClass<HkbEventProperty<'a>>),
    /// Visitor fields
    #[serde(rename = "EventToCrossBlend")]
    EventToCrossBlend(SingleClass<HkbEventProperty<'a>>),
    /// Visitor fields
    #[serde(rename = "fBlendParameter")]
    FBlendParameter(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "fTransitionDuration")]
    FTransitionDuration(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "eBlendCurve")]
    EBlendCurve(Primitive<BlendCurve>),
    /// Visitor fields
    #[serde(rename = "pTransitionBlenderGenerator", skip_serializing)]
    PTransitionBlenderGenerator(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "pTransitionEffect", skip_serializing)]
    PTransitionEffect(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "currentMode", skip_serializing)]
    CurrentMode(Primitive<()>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BsCyclicBlendTransitionGeneratorVisitor<'de>, "@name",
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
    ("pBlenderGenerator" => PBlenderGenerator(Primitive<Cow<'de, str>>)),
    ("EventToFreezeBlendValue" => EventToFreezeBlendValue(SingleClass<HkbEventProperty<'de>>)),
    ("EventToCrossBlend" => EventToCrossBlend(SingleClass<HkbEventProperty<'de>>)),
    ("fBlendParameter" => FBlendParameter(Primitive<f32>)),
    ("fTransitionDuration" => FTransitionDuration(Primitive<f32>)),
    ("eBlendCurve" => EBlendCurve(Primitive<BlendCurve>)),
    ("pTransitionBlenderGenerator" => PTransitionBlenderGenerator(Primitive<Cow<'de, str>>)),
    ("pTransitionEffect" => PTransitionEffect(Primitive<Cow<'de, str>>)),
    ("currentMode" => CurrentMode(Primitive<()>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum CurrentBlendMode {
    #[serde(rename = "MODE_INACTIVE")]
    #[default]
    ModeInactive = -1,
    #[serde(rename = "MODE_DEFAULT")]
    ModeDefault = 0,
    #[serde(rename = "MODE_FROZEN")]
    ModeFrozen = 1,
    #[serde(rename = "MODE_BLENDING")]
    ModeBlending = 2,
    #[serde(rename = "MODE_WAITINGFORBLENDING")]
    ModeWaitingforblending = 3,
}
