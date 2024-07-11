//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbExpressionData`
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

/// `hkbExpressionData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// - signature: `0x6740042a`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbExpressionData<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"expression"`
    /// -   type: `hkStringPtr`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub expression: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"assignmentVariableIndex"`
    /// -   type: `hkInt32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub assignment_variable_index: i32,
    /// # C++ Class Fields Info
    /// -   name:`"assignmentEventIndex"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub assignment_event_index: i32,
    /// # C++ Class Fields Info
    /// -   name:`"eventMode"`
    /// -   type: `enum ExpressionEventMode`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub event_mode: ExpressionEventMode,
    /// # C++ Class Fields Info
    /// -   name:`"raisedEvent"`
    /// -   type: `hkBool`
    /// - offset: 13
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub raised_event: bool,
    /// # C++ Class Fields Info
    /// -   name:`"wasTrueInPreviousFrame"`
    /// -   type: `hkBool`
    /// - offset: 14
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub was_true_in_previous_frame: bool,
}

impl Serialize for HkbExpressionData<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbExpressionDataVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbExpressionData<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbExpressionDataVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbExpressionDataVisitor<'a>>> for HkbExpressionData<'a> {
    fn from(_values: Vec<HkbExpressionDataVisitor<'a>>) -> Self {
            let mut expression = None;
            let mut assignment_variable_index = None;
            let mut assignment_event_index = None;
            let mut event_mode = None;
            let mut raised_event = None;
            let mut was_true_in_previous_frame = None;


        for _value in _values {
            match _value {
                HkbExpressionDataVisitor::Expression(m) => expression = Some(m),
                HkbExpressionDataVisitor::AssignmentVariableIndex(m) => assignment_variable_index = Some(m),
                HkbExpressionDataVisitor::AssignmentEventIndex(m) => assignment_event_index = Some(m),
                HkbExpressionDataVisitor::EventMode(m) => event_mode = Some(m),
                HkbExpressionDataVisitor::RaisedEvent(m) => raised_event = Some(m),
                HkbExpressionDataVisitor::WasTrueInPreviousFrame(m) => was_true_in_previous_frame = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            expression: expression.unwrap_or_default().into_inner(),
            assignment_variable_index: assignment_variable_index.unwrap_or_default().into_inner(),
            assignment_event_index: assignment_event_index.unwrap_or_default().into_inner(),
            event_mode: event_mode.unwrap_or_default().into_inner(),
            raised_event: raised_event.unwrap_or_default().into_inner(),
            was_true_in_previous_frame: was_true_in_previous_frame.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbExpressionData<'a>> for Vec<HkbExpressionDataVisitor<'a>> {
    fn from(data: &HkbExpressionData<'a>) -> Self {
        vec![
            HkbExpressionDataVisitor::Expression(data.expression.clone().into()),
            HkbExpressionDataVisitor::AssignmentVariableIndex(data.assignment_variable_index.into()),
            HkbExpressionDataVisitor::AssignmentEventIndex(data.assignment_event_index.into()),
            HkbExpressionDataVisitor::EventMode(data.event_mode.clone().into()),
            HkbExpressionDataVisitor::RaisedEvent(data.raised_event.into()),
            HkbExpressionDataVisitor::WasTrueInPreviousFrame(data.was_true_in_previous_frame.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbExpressionData<'de> {
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
enum HkbExpressionDataVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "expression")]
    Expression(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "assignmentVariableIndex")]
    AssignmentVariableIndex(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "assignmentEventIndex")]
    AssignmentEventIndex(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "eventMode")]
    EventMode(Primitive<ExpressionEventMode>),
    /// Visitor fields
    #[serde(rename = "raisedEvent", skip_serializing)]
    RaisedEvent(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "wasTrueInPreviousFrame", skip_serializing)]
    WasTrueInPreviousFrame(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbExpressionDataVisitor<'de>, "@name",
    ("expression" => Expression(Primitive<Cow<'de, str>>)),
    ("assignmentVariableIndex" => AssignmentVariableIndex(Primitive<i32>)),
    ("assignmentEventIndex" => AssignmentEventIndex(Primitive<i32>)),
    ("eventMode" => EventMode(Primitive<ExpressionEventMode>)),
    ("raisedEvent" => RaisedEvent(Primitive<bool>)),
    ("wasTrueInPreviousFrame" => WasTrueInPreviousFrame(Primitive<bool>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum ExpressionEventMode {
    #[serde(rename = "EVENT_MODE_SEND_ONCE")]
    #[default]
    EventModeSendOnce = 0,
    #[serde(rename = "EVENT_MODE_SEND_ON_TRUE")]
    EventModeSendOnTrue = 1,
    #[serde(rename = "EVENT_MODE_SEND_ON_FALSE_TO_TRUE")]
    EventModeSendOnFalseToTrue = 2,
    #[serde(rename = "EVENT_MODE_SEND_EVERY_FRAME_ONCE_TRUE")]
    EventModeSendEveryFrameOnceTrue = 3,
}
