//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbContext`
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

/// `hkbContext`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 40
/// -    vtable: false
/// - signature: `0xe0c4d4a7`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbContext<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"character"`
    /// -   type: `void*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub character: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"behavior"`
    /// -   type: `void*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub behavior: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"nodeToIndexMap"`
    /// -   type: `void*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub node_to_index_map: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"eventQueue"`
    /// -   type: `void*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub event_queue: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"sharedEventQueue"`
    /// -   type: `void*`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub shared_event_queue: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"generatorOutputListener"`
    /// -   type: `struct hkbGeneratorOutputListener*`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub generator_output_listener: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"eventTriggeredTransition"`
    /// -   type: `hkBool`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub event_triggered_transition: bool,
    /// # C++ Class Fields Info
    /// -   name:`"world"`
    /// -   type: `void*`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub world: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"attachmentManager"`
    /// -   type: `void*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub attachment_manager: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"animationCache"`
    /// -   type: `void*`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub animation_cache: Cow<'a, str>,
}

impl Serialize for HkbContext<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbContextVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbContext<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbContextVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbContextVisitor<'a>>> for HkbContext<'a> {
    fn from(_values: Vec<HkbContextVisitor<'a>>) -> Self {
            let mut character = None;
            let mut behavior = None;
            let mut node_to_index_map = None;
            let mut event_queue = None;
            let mut shared_event_queue = None;
            let mut generator_output_listener = None;
            let mut event_triggered_transition = None;
            let mut world = None;
            let mut attachment_manager = None;
            let mut animation_cache = None;


        for _value in _values {
            match _value {
                HkbContextVisitor::Character(m) => character = Some(m),
                HkbContextVisitor::Behavior(m) => behavior = Some(m),
                HkbContextVisitor::NodeToIndexMap(m) => node_to_index_map = Some(m),
                HkbContextVisitor::EventQueue(m) => event_queue = Some(m),
                HkbContextVisitor::SharedEventQueue(m) => shared_event_queue = Some(m),
                HkbContextVisitor::GeneratorOutputListener(m) => generator_output_listener = Some(m),
                HkbContextVisitor::EventTriggeredTransition(m) => event_triggered_transition = Some(m),
                HkbContextVisitor::World(m) => world = Some(m),
                HkbContextVisitor::AttachmentManager(m) => attachment_manager = Some(m),
                HkbContextVisitor::AnimationCache(m) => animation_cache = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            character: character.unwrap_or_default().into_inner(),
            behavior: behavior.unwrap_or_default().into_inner(),
            node_to_index_map: node_to_index_map.unwrap_or_default().into_inner(),
            event_queue: event_queue.unwrap_or_default().into_inner(),
            shared_event_queue: shared_event_queue.unwrap_or_default().into_inner(),
            generator_output_listener: generator_output_listener.unwrap_or_default().into_inner(),
            event_triggered_transition: event_triggered_transition.unwrap_or_default().into_inner(),
            world: world.unwrap_or_default().into_inner(),
            attachment_manager: attachment_manager.unwrap_or_default().into_inner(),
            animation_cache: animation_cache.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbContext<'a>> for Vec<HkbContextVisitor<'a>> {
    fn from(data: &HkbContext<'a>) -> Self {
        vec![
            HkbContextVisitor::Character(data.character.clone().into()),
            HkbContextVisitor::Behavior(data.behavior.clone().into()),
            HkbContextVisitor::NodeToIndexMap(data.node_to_index_map.clone().into()),
            HkbContextVisitor::EventQueue(data.event_queue.clone().into()),
            HkbContextVisitor::SharedEventQueue(data.shared_event_queue.clone().into()),
            HkbContextVisitor::GeneratorOutputListener(data.generator_output_listener.clone().into()),
            HkbContextVisitor::EventTriggeredTransition(data.event_triggered_transition.into()),
            HkbContextVisitor::World(data.world.clone().into()),
            HkbContextVisitor::AttachmentManager(data.attachment_manager.clone().into()),
            HkbContextVisitor::AnimationCache(data.animation_cache.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbContext<'de> {
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
enum HkbContextVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "character", skip_serializing)]
    Character(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "behavior", skip_serializing)]
    Behavior(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "nodeToIndexMap", skip_serializing)]
    NodeToIndexMap(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "eventQueue", skip_serializing)]
    EventQueue(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "sharedEventQueue", skip_serializing)]
    SharedEventQueue(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "generatorOutputListener")]
    GeneratorOutputListener(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "eventTriggeredTransition", skip_serializing)]
    EventTriggeredTransition(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "world", skip_serializing)]
    World(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "attachmentManager", skip_serializing)]
    AttachmentManager(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "animationCache", skip_serializing)]
    AnimationCache(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbContextVisitor<'de>, "@name",
    ("character" => Character(Primitive<Cow<'de, str>>)),
    ("behavior" => Behavior(Primitive<Cow<'de, str>>)),
    ("nodeToIndexMap" => NodeToIndexMap(Primitive<Cow<'de, str>>)),
    ("eventQueue" => EventQueue(Primitive<Cow<'de, str>>)),
    ("sharedEventQueue" => SharedEventQueue(Primitive<Cow<'de, str>>)),
    ("generatorOutputListener" => GeneratorOutputListener(Primitive<Cow<'de, str>>)),
    ("eventTriggeredTransition" => EventTriggeredTransition(Primitive<bool>)),
    ("world" => World(Primitive<Cow<'de, str>>)),
    ("attachmentManager" => AttachmentManager(Primitive<Cow<'de, str>>)),
    ("animationCache" => AnimationCache(Primitive<Cow<'de, str>>)),
}
