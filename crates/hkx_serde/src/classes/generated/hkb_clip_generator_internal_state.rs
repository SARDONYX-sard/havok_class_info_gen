//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbClipGeneratorInternalState`
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

/// `hkbClipGeneratorInternalState`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x26ce5bf3`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbClipGeneratorInternalState {
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
    /// -   name:`"extractedMotion"`
    /// -   type: `hkQsTransform`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub extracted_motion: QsTransform<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"echos"`
    /// -   type: `hkArray<struct hkbClipGeneratorEcho>`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub echos: HkArrayClass<HkbClipGeneratorEcho>,
    /// # C++ Class Fields Info
    /// -   name:`"localTime"`
    /// -   type: `hkReal`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    pub local_time: f32,
    /// # C++ Class Fields Info
    /// -   name:`"time"`
    /// -   type: `hkReal`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub time: f32,
    /// # C++ Class Fields Info
    /// -   name:`"previousUserControlledTimeFraction"`
    /// -   type: `hkReal`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    pub previous_user_controlled_time_fraction: f32,
    /// # C++ Class Fields Info
    /// -   name:`"bufferSize"`
    /// -   type: `hkInt32`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    pub buffer_size: i32,
    /// # C++ Class Fields Info
    /// -   name:`"echoBufferSize"`
    /// -   type: `hkInt32`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    pub echo_buffer_size: i32,
    /// # C++ Class Fields Info
    /// -   name:`"atEnd"`
    /// -   type: `hkBool`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    pub at_end: bool,
    /// # C++ Class Fields Info
    /// -   name:`"ignoreStartTime"`
    /// -   type: `hkBool`
    /// - offset: 97
    /// -  flags: `FLAGS_NONE`
    pub ignore_start_time: bool,
    /// # C++ Class Fields Info
    /// -   name:`"pingPongBackward"`
    /// -   type: `hkBool`
    /// - offset: 98
    /// -  flags: `FLAGS_NONE`
    pub ping_pong_backward: bool,
}

impl Serialize for HkbClipGeneratorInternalState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbClipGeneratorInternalStateVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbClipGeneratorInternalState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbClipGeneratorInternalStateVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkbClipGeneratorInternalStateVisitor>> for HkbClipGeneratorInternalState {
    fn from(_values: Vec<HkbClipGeneratorInternalStateVisitor>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut extracted_motion = None;
            let mut echos = None;
            let mut local_time = None;
            let mut time = None;
            let mut previous_user_controlled_time_fraction = None;
            let mut buffer_size = None;
            let mut echo_buffer_size = None;
            let mut at_end = None;
            let mut ignore_start_time = None;
            let mut ping_pong_backward = None;


        for _value in _values {
            match _value {
                HkbClipGeneratorInternalStateVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbClipGeneratorInternalStateVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbClipGeneratorInternalStateVisitor::ExtractedMotion(m) => extracted_motion = Some(m),
                HkbClipGeneratorInternalStateVisitor::Echos(m) => echos = Some(m),
                HkbClipGeneratorInternalStateVisitor::LocalTime(m) => local_time = Some(m),
                HkbClipGeneratorInternalStateVisitor::Time(m) => time = Some(m),
                HkbClipGeneratorInternalStateVisitor::PreviousUserControlledTimeFraction(m) => previous_user_controlled_time_fraction = Some(m),
                HkbClipGeneratorInternalStateVisitor::BufferSize(m) => buffer_size = Some(m),
                HkbClipGeneratorInternalStateVisitor::EchoBufferSize(m) => echo_buffer_size = Some(m),
                HkbClipGeneratorInternalStateVisitor::AtEnd(m) => at_end = Some(m),
                HkbClipGeneratorInternalStateVisitor::IgnoreStartTime(m) => ignore_start_time = Some(m),
                HkbClipGeneratorInternalStateVisitor::PingPongBackward(m) => ping_pong_backward = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            extracted_motion: extracted_motion.unwrap_or_default().into_inner(),
            echos: echos.unwrap_or_default(),
            local_time: local_time.unwrap_or_default().into_inner(),
            time: time.unwrap_or_default().into_inner(),
            previous_user_controlled_time_fraction: previous_user_controlled_time_fraction.unwrap_or_default().into_inner(),
            buffer_size: buffer_size.unwrap_or_default().into_inner(),
            echo_buffer_size: echo_buffer_size.unwrap_or_default().into_inner(),
            at_end: at_end.unwrap_or_default().into_inner(),
            ignore_start_time: ignore_start_time.unwrap_or_default().into_inner(),
            ping_pong_backward: ping_pong_backward.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkbClipGeneratorInternalState> for Vec<HkbClipGeneratorInternalStateVisitor> {
    fn from(data: &HkbClipGeneratorInternalState) -> Self {
        vec![
            HkbClipGeneratorInternalStateVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbClipGeneratorInternalStateVisitor::ReferenceCount(data.reference_count.into()),
            HkbClipGeneratorInternalStateVisitor::ExtractedMotion(data.extracted_motion.clone().into()),
            HkbClipGeneratorInternalStateVisitor::Echos(data.echos.clone()),
            HkbClipGeneratorInternalStateVisitor::LocalTime(data.local_time.into()),
            HkbClipGeneratorInternalStateVisitor::Time(data.time.into()),
            HkbClipGeneratorInternalStateVisitor::PreviousUserControlledTimeFraction(data.previous_user_controlled_time_fraction.into()),
            HkbClipGeneratorInternalStateVisitor::BufferSize(data.buffer_size.into()),
            HkbClipGeneratorInternalStateVisitor::EchoBufferSize(data.echo_buffer_size.into()),
            HkbClipGeneratorInternalStateVisitor::AtEnd(data.at_end.into()),
            HkbClipGeneratorInternalStateVisitor::IgnoreStartTime(data.ignore_start_time.into()),
            HkbClipGeneratorInternalStateVisitor::PingPongBackward(data.ping_pong_backward.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbClipGeneratorInternalState {
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
enum HkbClipGeneratorInternalStateVisitor {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "extractedMotion")]
    ExtractedMotion(Primitive<QsTransform<f32>>),
    /// Visitor fields
    #[serde(rename = "echos")]
    Echos(HkArrayClass<HkbClipGeneratorEcho>),
    /// Visitor fields
    #[serde(rename = "localTime")]
    LocalTime(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "time")]
    Time(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "previousUserControlledTimeFraction")]
    PreviousUserControlledTimeFraction(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "bufferSize")]
    BufferSize(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "echoBufferSize")]
    EchoBufferSize(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "atEnd")]
    AtEnd(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "ignoreStartTime")]
    IgnoreStartTime(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "pingPongBackward")]
    PingPongBackward(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbClipGeneratorInternalStateVisitor, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("extractedMotion" => ExtractedMotion(Primitive<QsTransform<f32>>)),
    ("echos" => Echos(HkArrayClass<HkbClipGeneratorEcho>)),
    ("localTime" => LocalTime(Primitive<f32>)),
    ("time" => Time(Primitive<f32>)),
    ("previousUserControlledTimeFraction" => PreviousUserControlledTimeFraction(Primitive<f32>)),
    ("bufferSize" => BufferSize(Primitive<i32>)),
    ("echoBufferSize" => EchoBufferSize(Primitive<i32>)),
    ("atEnd" => AtEnd(Primitive<bool>)),
    ("ignoreStartTime" => IgnoreStartTime(Primitive<bool>)),
    ("pingPongBackward" => PingPongBackward(Primitive<bool>)),
}
