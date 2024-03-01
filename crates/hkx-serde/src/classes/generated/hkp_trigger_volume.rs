//! A Rust structure that implements a serializer/deserializer corresponding to `hkpTriggerVolume`, a class defined in C++
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
/// -    size: 52
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpTriggerVolume<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpTriggerVolume"`: Name of this class.
    #[serde(default = "HkpTriggerVolume::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xa29a8d1a`: Unique value of this class.
    #[serde(default = "HkpTriggerVolume::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpTriggerVolumeHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpTriggerVolumeHkParam<'a>>
}

impl HkpTriggerVolume<'_> {
    /// Return `"hkpTriggerVolume"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpTriggerVolume".into()
    }

    /// Return `"0xa29a8d1a"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xa29a8d1a".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpTriggerVolumeHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"overlappingBodies"`
    /// -   type: `hkArray&lt;hkpRigidBody*&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "overlappingBodies")]
    OverlappingBodies(Vec<Box<HkpRigidBody>>),
    /// # Information on fields in the original C++ class
    /// -   name:`"eventQueue"`
    /// -   type: `hkArray&lt;struct hkpTriggerVolumeEventInfo&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "eventQueue")]
    EventQueue(Vec<HkpTriggerVolumeEventInfo>),
    /// # Information on fields in the original C++ class
    /// -   name:`"triggerBody"`
    /// -   type: `struct hkpRigidBody*`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "triggerBody")]
    TriggerBody(Box<HkpRigidBody>),
    /// # Information on fields in the original C++ class
    /// -   name:`"sequenceNumber"`
    /// -   type: `hkUint32`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sequenceNumber")]
    SequenceNumber(u32),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpTriggerVolumeHkParam<'de>, "@name",
    ("overlappingBodies" => OverlappingBodies(Vec<Box<HkpRigidBody>>)),
    ("eventQueue" => EventQueue(Vec<HkpTriggerVolumeEventInfo>)),
    ("triggerBody" => TriggerBody(Box<HkpRigidBody>)),
    ("sequenceNumber" => SequenceNumber(u32)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum EventType {
    #[serde(rename = "ENTERED_EVENT")]
    EnteredEvent = 1,
    #[serde(rename = "LEFT_EVENT")]
    LeftEvent = 2,
    #[serde(rename = "ENTERED_AND_LEFT_EVENT")]
    EnteredAndLeftEvent = 3,
    #[serde(rename = "TRIGGER_BODY_LEFT_EVENT")]
    TriggerBodyLeftEvent = 6,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Operation {
    #[serde(rename = "ADDED_OP")]
    AddedOp = 0,
    #[serde(rename = "REMOVED_OP")]
    RemovedOp = 1,
    #[serde(rename = "CONTACT_OP")]
    ContactOp = 2,
    #[serde(rename = "TOI_OP")]
    ToiOp = 3,
}
