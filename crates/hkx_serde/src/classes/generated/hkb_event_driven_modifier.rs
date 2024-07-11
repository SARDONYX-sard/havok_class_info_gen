//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbEventDrivenModifier`
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

/// `hkbEventDrivenModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 60
/// -    vtable: true
/// -    parent: `hkbModifierWrapper`/`0x3697e044`
/// - signature: `0x7ed3f44e`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbEventDrivenModifier<'a> {
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
    /// -   name:`"activateEventId"`
    /// -   type: `hkInt32`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub activate_event_id: i32,
    /// # C++ Class Fields Info
    /// -   name:`"deactivateEventId"`
    /// -   type: `hkInt32`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub deactivate_event_id: i32,
    /// # C++ Class Fields Info
    /// -   name:`"activeByDefault"`
    /// -   type: `hkBool`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    pub active_by_default: bool,
    /// # C++ Class Fields Info
    /// -   name:`"isActive"`
    /// -   type: `hkBool`
    /// - offset: 57
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub is_active: bool,
}

impl Serialize for HkbEventDrivenModifier<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbEventDrivenModifierVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbEventDrivenModifier<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbEventDrivenModifierVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbEventDrivenModifierVisitor<'a>>> for HkbEventDrivenModifier<'a> {
    fn from(_values: Vec<HkbEventDrivenModifierVisitor<'a>>) -> Self {
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
            let mut activate_event_id = None;
            let mut deactivate_event_id = None;
            let mut active_by_default = None;
            let mut is_active = None;


        for _value in _values {
            match _value {
                HkbEventDrivenModifierVisitor::Modifier(m) => modifier = Some(m),
                HkbEventDrivenModifierVisitor::Enable(m) => enable = Some(m),
                HkbEventDrivenModifierVisitor::PadModifier(m) => pad_modifier = Some(m),
                HkbEventDrivenModifierVisitor::UserData(m) => user_data = Some(m),
                HkbEventDrivenModifierVisitor::Name(m) => name = Some(m),
                HkbEventDrivenModifierVisitor::Id(m) => id = Some(m),
                HkbEventDrivenModifierVisitor::CloneState(m) => clone_state = Some(m),
                HkbEventDrivenModifierVisitor::PadNode(m) => pad_node = Some(m),
                HkbEventDrivenModifierVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                HkbEventDrivenModifierVisitor::CachedBindables(m) => cached_bindables = Some(m),
                HkbEventDrivenModifierVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                HkbEventDrivenModifierVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbEventDrivenModifierVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbEventDrivenModifierVisitor::ActivateEventId(m) => activate_event_id = Some(m),
                HkbEventDrivenModifierVisitor::DeactivateEventId(m) => deactivate_event_id = Some(m),
                HkbEventDrivenModifierVisitor::ActiveByDefault(m) => active_by_default = Some(m),
                HkbEventDrivenModifierVisitor::IsActive(m) => is_active = Some(m),

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
            activate_event_id: activate_event_id.unwrap_or_default().into_inner(),
            deactivate_event_id: deactivate_event_id.unwrap_or_default().into_inner(),
            active_by_default: active_by_default.unwrap_or_default().into_inner(),
            is_active: is_active.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbEventDrivenModifier<'a>> for Vec<HkbEventDrivenModifierVisitor<'a>> {
    fn from(data: &HkbEventDrivenModifier<'a>) -> Self {
        vec![
            HkbEventDrivenModifierVisitor::Modifier(data.modifier.clone().into()),
            HkbEventDrivenModifierVisitor::Enable(data.enable.into()),
            HkbEventDrivenModifierVisitor::PadModifier(data.pad_modifier.clone()),
            HkbEventDrivenModifierVisitor::UserData(data.user_data.into()),
            HkbEventDrivenModifierVisitor::Name(data.name.clone().into()),
            HkbEventDrivenModifierVisitor::Id(data.id.into()),
            HkbEventDrivenModifierVisitor::CloneState(data.clone_state.into()),
            HkbEventDrivenModifierVisitor::PadNode(data.pad_node.clone()),
            HkbEventDrivenModifierVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            HkbEventDrivenModifierVisitor::CachedBindables(data.cached_bindables.clone()),
            HkbEventDrivenModifierVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            HkbEventDrivenModifierVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbEventDrivenModifierVisitor::ReferenceCount(data.reference_count.into()),
            HkbEventDrivenModifierVisitor::ActivateEventId(data.activate_event_id.into()),
            HkbEventDrivenModifierVisitor::DeactivateEventId(data.deactivate_event_id.into()),
            HkbEventDrivenModifierVisitor::ActiveByDefault(data.active_by_default.into()),
            HkbEventDrivenModifierVisitor::IsActive(data.is_active.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbEventDrivenModifier<'de> {
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
enum HkbEventDrivenModifierVisitor<'a> {
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
    #[serde(rename = "activateEventId")]
    ActivateEventId(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "deactivateEventId")]
    DeactivateEventId(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "activeByDefault")]
    ActiveByDefault(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "isActive", skip_serializing)]
    IsActive(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbEventDrivenModifierVisitor<'de>, "@name",
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
    ("activateEventId" => ActivateEventId(Primitive<i32>)),
    ("deactivateEventId" => DeactivateEventId(Primitive<i32>)),
    ("activeByDefault" => ActiveByDefault(Primitive<bool>)),
    ("isActive" => IsActive(Primitive<bool>)),
}
