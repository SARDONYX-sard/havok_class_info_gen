//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbDelayedModifier`
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

/// `hkbDelayedModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: true
/// -    parent: `hkbModifierWrapper`/`0x3697e044`
/// - signature: `0x8e101a7a`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbDelayedModifier<'a> {
    /// # C++ Parent class(`hkbModifierWrapper` => parent: `hkbModifier`) field Info
    /// -   name:`"modifier"`
    /// -   type: `struct hkbModifier*`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub modifier: Cow<'a, str>,

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
    /// -   name:`"delaySeconds"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub delay_seconds: f32,
    /// # C++ Class Fields Info
    /// -   name:`"durationSeconds"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub duration_seconds: f32,
    /// # C++ Class Fields Info
    /// -   name:`"secondsElapsed"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub seconds_elapsed: f32,
    /// # C++ Class Fields Info
    /// -   name:`"isActive"`
    /// -   type: `hkBool`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub is_active: bool,
}

impl Serialize for HkbDelayedModifier<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbDelayedModifierVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbDelayedModifier<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbDelayedModifierVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbDelayedModifierVisitor<'a>>> for HkbDelayedModifier<'a> {
    fn from(_values: Vec<HkbDelayedModifierVisitor<'a>>) -> Self {
            let mut modifier = None;
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
            let mut delay_seconds = None;
            let mut duration_seconds = None;
            let mut seconds_elapsed = None;
            let mut is_active = None;


        for _value in _values {
            match _value {
                HkbDelayedModifierVisitor::Modifier(m) => modifier = Some(m),
                HkbDelayedModifierVisitor::Enable(m) => enable = Some(m),
                HkbDelayedModifierVisitor::PadModifier(m) => pad_modifier = Some(m),
                HkbDelayedModifierVisitor::UserData(m) => user_data = Some(m),
                HkbDelayedModifierVisitor::Name(m) => name = Some(m),
                HkbDelayedModifierVisitor::Id(m) => id = Some(m),
                HkbDelayedModifierVisitor::CloneState(m) => clone_state = Some(m),
                HkbDelayedModifierVisitor::PadNode(m) => pad_node = Some(m),
                HkbDelayedModifierVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                HkbDelayedModifierVisitor::CachedBindables(m) => cached_bindables = Some(m),
                HkbDelayedModifierVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                HkbDelayedModifierVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbDelayedModifierVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbDelayedModifierVisitor::DelaySeconds(m) => delay_seconds = Some(m),
                HkbDelayedModifierVisitor::DurationSeconds(m) => duration_seconds = Some(m),
                HkbDelayedModifierVisitor::SecondsElapsed(m) => seconds_elapsed = Some(m),
                HkbDelayedModifierVisitor::IsActive(m) => is_active = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            modifier: modifier.unwrap_or_default().into_inner(),
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
            delay_seconds: delay_seconds.unwrap_or_default().into_inner(),
            duration_seconds: duration_seconds.unwrap_or_default().into_inner(),
            seconds_elapsed: seconds_elapsed.unwrap_or_default().into_inner(),
            is_active: is_active.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbDelayedModifier<'a>> for Vec<HkbDelayedModifierVisitor<'a>> {
    fn from(data: &HkbDelayedModifier<'a>) -> Self {
        vec![
            HkbDelayedModifierVisitor::Modifier(data.modifier.clone().into()),
            HkbDelayedModifierVisitor::Enable(data.enable.into()),
            HkbDelayedModifierVisitor::PadModifier(data.pad_modifier.clone()),
            HkbDelayedModifierVisitor::UserData(data.user_data.into()),
            HkbDelayedModifierVisitor::Name(data.name.clone().into()),
            HkbDelayedModifierVisitor::Id(data.id.into()),
            HkbDelayedModifierVisitor::CloneState(data.clone_state.into()),
            HkbDelayedModifierVisitor::PadNode(data.pad_node.clone()),
            HkbDelayedModifierVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            HkbDelayedModifierVisitor::CachedBindables(data.cached_bindables.clone()),
            HkbDelayedModifierVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            HkbDelayedModifierVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbDelayedModifierVisitor::ReferenceCount(data.reference_count.into()),
            HkbDelayedModifierVisitor::DelaySeconds(data.delay_seconds.into()),
            HkbDelayedModifierVisitor::DurationSeconds(data.duration_seconds.into()),
            HkbDelayedModifierVisitor::SecondsElapsed(data.seconds_elapsed.into()),
            HkbDelayedModifierVisitor::IsActive(data.is_active.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbDelayedModifier<'de> {
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
enum HkbDelayedModifierVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "modifier")]
    Modifier(Primitive<Cow<'a, str>>),

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
    #[serde(rename = "delaySeconds")]
    DelaySeconds(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "durationSeconds")]
    DurationSeconds(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "secondsElapsed", skip_serializing)]
    SecondsElapsed(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "isActive", skip_serializing)]
    IsActive(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbDelayedModifierVisitor<'de>, "@name",
    ("modifier" => Modifier(Primitive<Cow<'de, str>>)),
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
    ("delaySeconds" => DelaySeconds(Primitive<f32>)),
    ("durationSeconds" => DurationSeconds(Primitive<f32>)),
    ("secondsElapsed" => SecondsElapsed(Primitive<f32>)),
    ("isActive" => IsActive(Primitive<bool>)),
}
