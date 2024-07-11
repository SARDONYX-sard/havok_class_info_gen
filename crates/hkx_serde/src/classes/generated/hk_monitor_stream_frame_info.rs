//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkMonitorStreamFrameInfo`
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

/// `hkMonitorStreamFrameInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 36
/// -    vtable: false
/// - signature: `0x7798b7db`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkMonitorStreamFrameInfo<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"heading"`
    /// -   type: `hkStringPtr`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub heading: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"indexOfTimer0"`
    /// -   type: `hkInt32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub index_of_timer_0: i32,
    /// # C++ Class Fields Info
    /// -   name:`"indexOfTimer1"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub index_of_timer_1: i32,
    /// # C++ Class Fields Info
    /// -   name:`"absoluteTimeCounter"`
    /// -   type: `enum AbsoluteTimeCounter`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub absolute_time_counter: AbsoluteTimeCounter,
    /// # C++ Class Fields Info
    /// -   name:`"timerFactor0"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub timer_factor_0: f32,
    /// # C++ Class Fields Info
    /// -   name:`"timerFactor1"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub timer_factor_1: f32,
    /// # C++ Class Fields Info
    /// -   name:`"threadId"`
    /// -   type: `hkInt32`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub thread_id: i32,
    /// # C++ Class Fields Info
    /// -   name:`"frameStreamStart"`
    /// -   type: `hkInt32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub frame_stream_start: i32,
    /// # C++ Class Fields Info
    /// -   name:`"frameStreamEnd"`
    /// -   type: `hkInt32`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub frame_stream_end: i32,
}

impl Serialize for HkMonitorStreamFrameInfo<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkMonitorStreamFrameInfoVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkMonitorStreamFrameInfo<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkMonitorStreamFrameInfoVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkMonitorStreamFrameInfoVisitor<'a>>> for HkMonitorStreamFrameInfo<'a> {
    fn from(_values: Vec<HkMonitorStreamFrameInfoVisitor<'a>>) -> Self {
            let mut heading = None;
            let mut index_of_timer_0 = None;
            let mut index_of_timer_1 = None;
            let mut absolute_time_counter = None;
            let mut timer_factor_0 = None;
            let mut timer_factor_1 = None;
            let mut thread_id = None;
            let mut frame_stream_start = None;
            let mut frame_stream_end = None;


        for _value in _values {
            match _value {
                HkMonitorStreamFrameInfoVisitor::Heading(m) => heading = Some(m),
                HkMonitorStreamFrameInfoVisitor::IndexOfTimer0(m) => index_of_timer_0 = Some(m),
                HkMonitorStreamFrameInfoVisitor::IndexOfTimer1(m) => index_of_timer_1 = Some(m),
                HkMonitorStreamFrameInfoVisitor::AbsoluteTimeCounter(m) => absolute_time_counter = Some(m),
                HkMonitorStreamFrameInfoVisitor::TimerFactor0(m) => timer_factor_0 = Some(m),
                HkMonitorStreamFrameInfoVisitor::TimerFactor1(m) => timer_factor_1 = Some(m),
                HkMonitorStreamFrameInfoVisitor::ThreadId(m) => thread_id = Some(m),
                HkMonitorStreamFrameInfoVisitor::FrameStreamStart(m) => frame_stream_start = Some(m),
                HkMonitorStreamFrameInfoVisitor::FrameStreamEnd(m) => frame_stream_end = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            heading: heading.unwrap_or_default().into_inner(),
            index_of_timer_0: index_of_timer_0.unwrap_or_default().into_inner(),
            index_of_timer_1: index_of_timer_1.unwrap_or_default().into_inner(),
            absolute_time_counter: absolute_time_counter.unwrap_or_default().into_inner(),
            timer_factor_0: timer_factor_0.unwrap_or_default().into_inner(),
            timer_factor_1: timer_factor_1.unwrap_or_default().into_inner(),
            thread_id: thread_id.unwrap_or_default().into_inner(),
            frame_stream_start: frame_stream_start.unwrap_or_default().into_inner(),
            frame_stream_end: frame_stream_end.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkMonitorStreamFrameInfo<'a>> for Vec<HkMonitorStreamFrameInfoVisitor<'a>> {
    fn from(data: &HkMonitorStreamFrameInfo<'a>) -> Self {
        vec![
            HkMonitorStreamFrameInfoVisitor::Heading(data.heading.clone().into()),
            HkMonitorStreamFrameInfoVisitor::IndexOfTimer0(data.index_of_timer_0.into()),
            HkMonitorStreamFrameInfoVisitor::IndexOfTimer1(data.index_of_timer_1.into()),
            HkMonitorStreamFrameInfoVisitor::AbsoluteTimeCounter(data.absolute_time_counter.clone().into()),
            HkMonitorStreamFrameInfoVisitor::TimerFactor0(data.timer_factor_0.into()),
            HkMonitorStreamFrameInfoVisitor::TimerFactor1(data.timer_factor_1.into()),
            HkMonitorStreamFrameInfoVisitor::ThreadId(data.thread_id.into()),
            HkMonitorStreamFrameInfoVisitor::FrameStreamStart(data.frame_stream_start.into()),
            HkMonitorStreamFrameInfoVisitor::FrameStreamEnd(data.frame_stream_end.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkMonitorStreamFrameInfo<'de> {
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
enum HkMonitorStreamFrameInfoVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "heading")]
    Heading(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "indexOfTimer0")]
    IndexOfTimer0(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "indexOfTimer1")]
    IndexOfTimer1(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "absoluteTimeCounter")]
    AbsoluteTimeCounter(Primitive<AbsoluteTimeCounter>),
    /// Visitor fields
    #[serde(rename = "timerFactor0")]
    TimerFactor0(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "timerFactor1")]
    TimerFactor1(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "threadId")]
    ThreadId(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "frameStreamStart")]
    FrameStreamStart(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "frameStreamEnd")]
    FrameStreamEnd(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMonitorStreamFrameInfoVisitor<'de>, "@name",
    ("heading" => Heading(Primitive<Cow<'de, str>>)),
    ("indexOfTimer0" => IndexOfTimer0(Primitive<i32>)),
    ("indexOfTimer1" => IndexOfTimer1(Primitive<i32>)),
    ("absoluteTimeCounter" => AbsoluteTimeCounter(Primitive<AbsoluteTimeCounter>)),
    ("timerFactor0" => TimerFactor0(Primitive<f32>)),
    ("timerFactor1" => TimerFactor1(Primitive<f32>)),
    ("threadId" => ThreadId(Primitive<i32>)),
    ("frameStreamStart" => FrameStreamStart(Primitive<i32>)),
    ("frameStreamEnd" => FrameStreamEnd(Primitive<i32>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum AbsoluteTimeCounter {
    #[serde(rename = "ABSOLUTE_TIME_TIMER_0")]
    #[default]
    AbsoluteTimeTimer0 = 0,
    #[serde(rename = "ABSOLUTE_TIME_TIMER_1")]
    AbsoluteTimeTimer1 = 1,
    #[serde(rename = "ABSOLUTE_TIME_NOT_TIMED")]
    AbsoluteTimeNotTimed = -1,
}
