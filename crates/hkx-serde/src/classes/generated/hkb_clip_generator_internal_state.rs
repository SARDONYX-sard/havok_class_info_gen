//! A Rust structure that implements a serializer/deserializer corresponding to `hkbClipGeneratorInternalState`, a class defined in C++
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// In XML, it is enclosed in a `hkobject` tag
/// and the `class` attribute contains the C++ class nam
///
/// # Information on the original C++ class
/// -    size: 112
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbClipGeneratorInternalState<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbClipGeneratorInternalState"`: Name of this class.
    #[serde(default = "HkbClipGeneratorInternalState::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x26ce5bf3`: Unique value of this class.
    #[serde(default = "HkbClipGeneratorInternalState::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbClipGeneratorInternalStateHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbClipGeneratorInternalStateHkParam<'a>>
}

impl HkbClipGeneratorInternalState<'_> {
    /// Return `"hkbClipGeneratorInternalState"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbClipGeneratorInternalState".into()
    }

    /// Return `"0x26ce5bf3"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x26ce5bf3".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbClipGeneratorInternalStateHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"extractedMotion"`
    /// -   type: `hkQsTransform`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "extractedMotion")]
    ExtractedMotion(cgmath::Matrix4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"echos"`
    /// -   type: `hkArray&lt;struct hkbClipGeneratorEcho&gt;`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "echos")]
    Echos(Vec<HkbClipGeneratorEcho>),
    /// # Information on fields in the original C++ class
    /// -   name:`"localTime"`
    /// -   type: `hkReal`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "localTime")]
    LocalTime(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"time"`
    /// -   type: `hkReal`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "time")]
    Time(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"previousUserControlledTimeFraction"`
    /// -   type: `hkReal`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "previousUserControlledTimeFraction")]
    PreviousUserControlledTimeFraction(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"bufferSize"`
    /// -   type: `hkInt32`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bufferSize")]
    BufferSize(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"echoBufferSize"`
    /// -   type: `hkInt32`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "echoBufferSize")]
    EchoBufferSize(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"atEnd"`
    /// -   type: `hkBool`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "atEnd")]
    AtEnd(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"ignoreStartTime"`
    /// -   type: `hkBool`
    /// - offset: 97
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ignoreStartTime")]
    IgnoreStartTime(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"pingPongBackward"`
    /// -   type: `hkBool`
    /// - offset: 98
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pingPongBackward")]
    PingPongBackward(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbClipGeneratorInternalStateHkParam<'de>, "@name",
    ("extractedMotion" => ExtractedMotion(cgmath::Matrix4<f32>)),
    ("echos" => Echos(Vec<HkbClipGeneratorEcho>)),
    ("localTime" => LocalTime(f64)),
    ("time" => Time(f64)),
    ("previousUserControlledTimeFraction" => PreviousUserControlledTimeFraction(f64)),
    ("bufferSize" => BufferSize(i32)),
    ("echoBufferSize" => EchoBufferSize(i32)),
    ("atEnd" => AtEnd(bool)),
    ("ignoreStartTime" => IgnoreStartTime(bool)),
    ("pingPongBackward" => PingPongBackward(bool)),
}
