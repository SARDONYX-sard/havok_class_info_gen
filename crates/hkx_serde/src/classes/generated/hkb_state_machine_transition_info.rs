//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbStateMachineTransitionInfo`
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

/// `hkbStateMachineTransitionInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 60
/// -    vtable: false
/// - signature: `0xcdec8025`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbStateMachineTransitionInfo<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"triggerInterval"`
    /// -   type: `struct hkbStateMachineTimeInterval`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub trigger_interval: SingleClass<HkbStateMachineTimeInterval>,
    /// # C++ Class Fields Info
    /// -   name:`"initiateInterval"`
    /// -   type: `struct hkbStateMachineTimeInterval`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub initiate_interval: SingleClass<HkbStateMachineTimeInterval>,
    /// # C++ Class Fields Info
    /// -   name:`"transition"`
    /// -   type: `struct hkbTransitionEffect*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub transition: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"condition"`
    /// -   type: `struct hkbCondition*`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    pub condition: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"eventId"`
    /// -   type: `hkInt32`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub event_id: i32,
    /// # C++ Class Fields Info
    /// -   name:`"toStateId"`
    /// -   type: `hkInt32`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub to_state_id: i32,
    /// # C++ Class Fields Info
    /// -   name:`"fromNestedStateId"`
    /// -   type: `hkInt32`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub from_nested_state_id: i32,
    /// # C++ Class Fields Info
    /// -   name:`"toNestedStateId"`
    /// -   type: `hkInt32`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub to_nested_state_id: i32,
    /// # C++ Class Fields Info
    /// -   name:`"priority"`
    /// -   type: `hkInt16`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    pub priority: i16,
    /// # C++ Class Fields Info
    /// -   name:`"flags"`
    /// -   type: `flags TransitionFlags`
    /// - offset: 58
    /// -  flags: `FLAGS_NONE`
    pub flags: TransitionFlags,
}

impl Serialize for HkbStateMachineTransitionInfo<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbStateMachineTransitionInfoVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbStateMachineTransitionInfo<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbStateMachineTransitionInfoVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbStateMachineTransitionInfoVisitor<'a>>> for HkbStateMachineTransitionInfo<'a> {
    fn from(_values: Vec<HkbStateMachineTransitionInfoVisitor<'a>>) -> Self {
            let mut trigger_interval = None;
            let mut initiate_interval = None;
            let mut transition = None;
            let mut condition = None;
            let mut event_id = None;
            let mut to_state_id = None;
            let mut from_nested_state_id = None;
            let mut to_nested_state_id = None;
            let mut priority = None;
            let mut flags = None;


        for _value in _values {
            match _value {
                HkbStateMachineTransitionInfoVisitor::TriggerInterval(m) => trigger_interval = Some(m),
                HkbStateMachineTransitionInfoVisitor::InitiateInterval(m) => initiate_interval = Some(m),
                HkbStateMachineTransitionInfoVisitor::Transition(m) => transition = Some(m),
                HkbStateMachineTransitionInfoVisitor::Condition(m) => condition = Some(m),
                HkbStateMachineTransitionInfoVisitor::EventId(m) => event_id = Some(m),
                HkbStateMachineTransitionInfoVisitor::ToStateId(m) => to_state_id = Some(m),
                HkbStateMachineTransitionInfoVisitor::FromNestedStateId(m) => from_nested_state_id = Some(m),
                HkbStateMachineTransitionInfoVisitor::ToNestedStateId(m) => to_nested_state_id = Some(m),
                HkbStateMachineTransitionInfoVisitor::Priority(m) => priority = Some(m),
                HkbStateMachineTransitionInfoVisitor::Flags(m) => flags = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            trigger_interval: trigger_interval.unwrap_or_default(),
            initiate_interval: initiate_interval.unwrap_or_default(),
            transition: transition.unwrap_or_default().into_inner(),
            condition: condition.unwrap_or_default().into_inner(),
            event_id: event_id.unwrap_or_default().into_inner(),
            to_state_id: to_state_id.unwrap_or_default().into_inner(),
            from_nested_state_id: from_nested_state_id.unwrap_or_default().into_inner(),
            to_nested_state_id: to_nested_state_id.unwrap_or_default().into_inner(),
            priority: priority.unwrap_or_default().into_inner(),
            flags: flags.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbStateMachineTransitionInfo<'a>> for Vec<HkbStateMachineTransitionInfoVisitor<'a>> {
    fn from(data: &HkbStateMachineTransitionInfo<'a>) -> Self {
        vec![
            HkbStateMachineTransitionInfoVisitor::TriggerInterval(data.trigger_interval.clone()),
            HkbStateMachineTransitionInfoVisitor::InitiateInterval(data.initiate_interval.clone()),
            HkbStateMachineTransitionInfoVisitor::Transition(data.transition.clone().into()),
            HkbStateMachineTransitionInfoVisitor::Condition(data.condition.clone().into()),
            HkbStateMachineTransitionInfoVisitor::EventId(data.event_id.into()),
            HkbStateMachineTransitionInfoVisitor::ToStateId(data.to_state_id.into()),
            HkbStateMachineTransitionInfoVisitor::FromNestedStateId(data.from_nested_state_id.into()),
            HkbStateMachineTransitionInfoVisitor::ToNestedStateId(data.to_nested_state_id.into()),
            HkbStateMachineTransitionInfoVisitor::Priority(data.priority.into()),
            HkbStateMachineTransitionInfoVisitor::Flags(data.flags.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbStateMachineTransitionInfo<'de> {
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
enum HkbStateMachineTransitionInfoVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "triggerInterval")]
    TriggerInterval(SingleClass<HkbStateMachineTimeInterval>),
    /// Visitor fields
    #[serde(rename = "initiateInterval")]
    InitiateInterval(SingleClass<HkbStateMachineTimeInterval>),
    /// Visitor fields
    #[serde(rename = "transition")]
    Transition(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "condition")]
    Condition(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "eventId")]
    EventId(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "toStateId")]
    ToStateId(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "fromNestedStateId")]
    FromNestedStateId(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "toNestedStateId")]
    ToNestedStateId(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "priority")]
    Priority(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "flags")]
    Flags(Primitive<TransitionFlags>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbStateMachineTransitionInfoVisitor<'de>, "@name",
    ("triggerInterval" => TriggerInterval(SingleClass<HkbStateMachineTimeInterval>)),
    ("initiateInterval" => InitiateInterval(SingleClass<HkbStateMachineTimeInterval>)),
    ("transition" => Transition(Primitive<Cow<'de, str>>)),
    ("condition" => Condition(Primitive<Cow<'de, str>>)),
    ("eventId" => EventId(Primitive<i32>)),
    ("toStateId" => ToStateId(Primitive<i32>)),
    ("fromNestedStateId" => FromNestedStateId(Primitive<i32>)),
    ("toNestedStateId" => ToNestedStateId(Primitive<i32>)),
    ("priority" => Priority(Primitive<i16>)),
    ("flags" => Flags(Primitive<TransitionFlags>)),
}
bitflags::bitflags! {
    /// # Bit flags that represented enum.
    ///
    /// # Note
    /// The 0 flag is always enabled when the trailing bit is 0.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TransitionFlags: u16 {
        /// Special flags." 0" is used when the character "0" is received.
        /// - only "0" is also entered during serialization.
        /// - This flag is the default value that exists for all flags.
        const NULL= !0;
        const FLAG_USE_TRIGGER_INTERVAL = 1;
        const FLAG_USE_INITIATE_INTERVAL = 2;
        const FLAG_UNINTERRUPTIBLE_WHILE_PLAYING = 4;
        const FLAG_UNINTERRUPTIBLE_WHILE_DELAYED = 8;
        const FLAG_DELAY_STATE_CHANGE = 16;
        const FLAG_DISABLED = 32;
        const FLAG_DISALLOW_RETURN_TO_PREVIOUS_STATE = 64;
        const FLAG_DISALLOW_RANDOM_TRANSITION = 128;
        const FLAG_DISABLE_CONDITION = 256;
        const FLAG_ALLOW_SELF_TRANSITION_BY_TRANSITION_FROM_ANY_STATE = 512;
        const FLAG_IS_GLOBAL_WILDCARD = 1024;
        const FLAG_IS_LOCAL_WILDCARD = 2048;
        const FLAG_FROM_NESTED_STATE_ID_IS_VALID = 4096;
        const FLAG_TO_NESTED_STATE_ID_IS_VALID = 8192;
        const FLAG_ABUT_AT_END_OF_FROM_GENERATOR = 16384;
    }
}

impl Default for TransitionFlags {
    fn default() -> Self {
        Self::NULL
    }
}

impl TryFrom<usize> for TransitionFlags {
    type Error = String;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Self::from_bits(value as u16).ok_or_else(|| format!("Set invalid value: {value}"))
    }
}

impl TryFrom<u16> for TransitionFlags {
    type Error = String;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_bits(value).ok_or_else(|| format!("Set invalid value: {value}"))
    }
}

impl serde::Serialize for TransitionFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if self.contains(Self::NULL) {
            serializer.serialize_str("0")
        } else {
            serializer.serialize_str(&self.human_readable())
        }
    }
}

impl<'de> serde::Deserialize<'de> for TransitionFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Option::<std::borrow::Cow<'de, str>>::deserialize(deserializer)?;

        match value {
            Some(s) => {
                if s.as_ref() == "0" {
                    return Ok(Self::NULL);
                };
                let mut flags = Self::empty();
                for token in s.split('|') {
                    match token.trim() {
                        "FLAG_USE_TRIGGER_INTERVAL" => flags |= Self::FLAG_USE_TRIGGER_INTERVAL,
                        "FLAG_USE_INITIATE_INTERVAL" => flags |= Self::FLAG_USE_INITIATE_INTERVAL,
                        "FLAG_UNINTERRUPTIBLE_WHILE_PLAYING" => flags |= Self::FLAG_UNINTERRUPTIBLE_WHILE_PLAYING,
                        "FLAG_UNINTERRUPTIBLE_WHILE_DELAYED" => flags |= Self::FLAG_UNINTERRUPTIBLE_WHILE_DELAYED,
                        "FLAG_DELAY_STATE_CHANGE" => flags |= Self::FLAG_DELAY_STATE_CHANGE,
                        "FLAG_DISABLED" => flags |= Self::FLAG_DISABLED,
                        "FLAG_DISALLOW_RETURN_TO_PREVIOUS_STATE" => flags |= Self::FLAG_DISALLOW_RETURN_TO_PREVIOUS_STATE,
                        "FLAG_DISALLOW_RANDOM_TRANSITION" => flags |= Self::FLAG_DISALLOW_RANDOM_TRANSITION,
                        "FLAG_DISABLE_CONDITION" => flags |= Self::FLAG_DISABLE_CONDITION,
                        "FLAG_ALLOW_SELF_TRANSITION_BY_TRANSITION_FROM_ANY_STATE" => flags |= Self::FLAG_ALLOW_SELF_TRANSITION_BY_TRANSITION_FROM_ANY_STATE,
                        "FLAG_IS_GLOBAL_WILDCARD" => flags |= Self::FLAG_IS_GLOBAL_WILDCARD,
                        "FLAG_IS_LOCAL_WILDCARD" => flags |= Self::FLAG_IS_LOCAL_WILDCARD,
                        "FLAG_FROM_NESTED_STATE_ID_IS_VALID" => flags |= Self::FLAG_FROM_NESTED_STATE_ID_IS_VALID,
                        "FLAG_TO_NESTED_STATE_ID_IS_VALID" => flags |= Self::FLAG_TO_NESTED_STATE_ID_IS_VALID,
                        "FLAG_ABUT_AT_END_OF_FROM_GENERATOR" => flags |= Self::FLAG_ABUT_AT_END_OF_FROM_GENERATOR,
                        unknown => match parse_int::parse(unknown) {
                            Ok(int) => {
                                if let Some(bits) = Self::from_bits(int) {
                                    flags |= bits
                                } else {
                                    return Err(serde::de::Error::custom(format!(
                                        "Expected TransitionFlags flags but got '{unknown}'",
                                    )));
                                };
                            }
                            Err(_err) => {
                                return Err(serde::de::Error::custom(format!(
                                    "Expected TransitionFlags flags or integer, but got '{unknown}'"
                                )))
                            }
                        },
                    }
                }
                Ok(flags)
            }
            None => Ok(Self::NULL),
        }
    }
}

impl core::fmt::Display for TransitionFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}

impl TransitionFlags {
    /// Use a string format that is easy for humans to read.
    ///
    /// Like this.
    /// `"FLAGS_NONE | SERIALIZE_IGNORED"`
    pub fn human_readable(&self) -> std::borrow::Cow<'_, str> {
        let mut flags = Vec::new();

        if self.contains(Self::NULL) {
            return "0".into();
        };

        if self.contains(Self::FLAG_USE_TRIGGER_INTERVAL) {
            flags.push("FLAG_USE_TRIGGER_INTERVAL");
        }
        if self.contains(Self::FLAG_USE_INITIATE_INTERVAL) {
            flags.push("FLAG_USE_INITIATE_INTERVAL");
        }
        if self.contains(Self::FLAG_UNINTERRUPTIBLE_WHILE_PLAYING) {
            flags.push("FLAG_UNINTERRUPTIBLE_WHILE_PLAYING");
        }
        if self.contains(Self::FLAG_UNINTERRUPTIBLE_WHILE_DELAYED) {
            flags.push("FLAG_UNINTERRUPTIBLE_WHILE_DELAYED");
        }
        if self.contains(Self::FLAG_DELAY_STATE_CHANGE) {
            flags.push("FLAG_DELAY_STATE_CHANGE");
        }
        if self.contains(Self::FLAG_DISABLED) {
            flags.push("FLAG_DISABLED");
        }
        if self.contains(Self::FLAG_DISALLOW_RETURN_TO_PREVIOUS_STATE) {
            flags.push("FLAG_DISALLOW_RETURN_TO_PREVIOUS_STATE");
        }
        if self.contains(Self::FLAG_DISALLOW_RANDOM_TRANSITION) {
            flags.push("FLAG_DISALLOW_RANDOM_TRANSITION");
        }
        if self.contains(Self::FLAG_DISABLE_CONDITION) {
            flags.push("FLAG_DISABLE_CONDITION");
        }
        if self.contains(Self::FLAG_ALLOW_SELF_TRANSITION_BY_TRANSITION_FROM_ANY_STATE) {
            flags.push("FLAG_ALLOW_SELF_TRANSITION_BY_TRANSITION_FROM_ANY_STATE");
        }
        if self.contains(Self::FLAG_IS_GLOBAL_WILDCARD) {
            flags.push("FLAG_IS_GLOBAL_WILDCARD");
        }
        if self.contains(Self::FLAG_IS_LOCAL_WILDCARD) {
            flags.push("FLAG_IS_LOCAL_WILDCARD");
        }
        if self.contains(Self::FLAG_FROM_NESTED_STATE_ID_IS_VALID) {
            flags.push("FLAG_FROM_NESTED_STATE_ID_IS_VALID");
        }
        if self.contains(Self::FLAG_TO_NESTED_STATE_ID_IS_VALID) {
            flags.push("FLAG_TO_NESTED_STATE_ID_IS_VALID");
        }
        if self.contains(Self::FLAG_ABUT_AT_END_OF_FROM_GENERATOR) {
            flags.push("FLAG_ABUT_AT_END_OF_FROM_GENERATOR");
        }

        flags.join("|").into()
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum InternalFlagBits {
    #[serde(rename = "FLAG_INTERNAL_IN_TRIGGER_INTERVAL")]
    #[default]
    FlagInternalInTriggerInterval = 1,
    #[serde(rename = "FLAG_INTERNAL_IN_INITIATE_INTERVAL")]
    FlagInternalInInitiateInterval = 2,
}
