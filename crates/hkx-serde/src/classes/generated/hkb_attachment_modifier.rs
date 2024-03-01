//! A Rust structure that implements a serializer/deserializer corresponding to `hkbAttachmentModifier`, a class defined in C++
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
/// -    size: 108
/// -  vtable: true
/// -  parent: hkbModifier/`96ec5ced`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbAttachmentModifier<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbAttachmentModifier"`: Name of this class.
    #[serde(default = "HkbAttachmentModifier::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xcc0aab32`: Unique value of this class.
    #[serde(default = "HkbAttachmentModifier::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbAttachmentModifierHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbAttachmentModifierHkParam<'a>>
}

impl HkbAttachmentModifier<'_> {
    /// Return `"hkbAttachmentModifier"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbAttachmentModifier".into()
    }

    /// Return `"0xcc0aab32"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xcc0aab32".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbAttachmentModifierHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"sendToAttacherOnAttach"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sendToAttacherOnAttach")]
    SendToAttacherOnAttach(HkbEventProperty),
    /// # Information on fields in the original C++ class
    /// -   name:`"sendToAttacheeOnAttach"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sendToAttacheeOnAttach")]
    SendToAttacheeOnAttach(HkbEventProperty),
    /// # Information on fields in the original C++ class
    /// -   name:`"sendToAttacherOnDetach"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sendToAttacherOnDetach")]
    SendToAttacherOnDetach(HkbEventProperty),
    /// # Information on fields in the original C++ class
    /// -   name:`"sendToAttacheeOnDetach"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sendToAttacheeOnDetach")]
    SendToAttacheeOnDetach(HkbEventProperty),
    /// # Information on fields in the original C++ class
    /// -   name:`"attachmentSetup"`
    /// -   type: `struct hkbAttachmentSetup*`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "attachmentSetup")]
    AttachmentSetup(Box<HkbAttachmentSetup>),
    /// # Information on fields in the original C++ class
    /// -   name:`"attacherHandle"`
    /// -   type: `struct hkbHandle*`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "attacherHandle")]
    AttacherHandle(Box<HkbHandle>),
    /// # Information on fields in the original C++ class
    /// -   name:`"attacheeHandle"`
    /// -   type: `struct hkbHandle*`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "attacheeHandle")]
    AttacheeHandle(Box<HkbHandle>),
    /// # Information on fields in the original C++ class
    /// -   name:`"attacheeLayer"`
    /// -   type: `hkInt32`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "attacheeLayer")]
    AttacheeLayer(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"attacheeRB"`
    /// -   type: `void*`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "attacheeRB", skip_serializing)]
    AttacheeRb(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"oldMotionType"`
    /// -   type: `enum unknown`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "oldMotionType", skip_serializing)]
    OldMotionType(Unknown),
    /// # Information on fields in the original C++ class
    /// -   name:`"oldFilterInfo"`
    /// -   type: `hkInt32`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "oldFilterInfo", skip_serializing)]
    OldFilterInfo(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"attachment"`
    /// -   type: `void*`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "attachment", skip_serializing)]
    Attachment(()),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbAttachmentModifierHkParam<'de>, "@name",
    ("sendToAttacherOnAttach" => SendToAttacherOnAttach(HkbEventProperty)),
    ("sendToAttacheeOnAttach" => SendToAttacheeOnAttach(HkbEventProperty)),
    ("sendToAttacherOnDetach" => SendToAttacherOnDetach(HkbEventProperty)),
    ("sendToAttacheeOnDetach" => SendToAttacheeOnDetach(HkbEventProperty)),
    ("attachmentSetup" => AttachmentSetup(Box<HkbAttachmentSetup>)),
    ("attacherHandle" => AttacherHandle(Box<HkbHandle>)),
    ("attacheeHandle" => AttacheeHandle(Box<HkbHandle>)),
    ("attacheeLayer" => AttacheeLayer(i32)),
    ("attacheeRB" => AttacheeRb(())),
    ("oldMotionType" => OldMotionType(Unknown)),
    ("oldFilterInfo" => OldFilterInfo(i32)),
    ("attachment" => Attachment(())),
}
