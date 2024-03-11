//! A Rust structure that implements a serializer/deserializer corresponding to `hkaBoneAttachment`, a class defined in C++
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::hk_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// In XML, it is enclosed in a `hkobject` tag
/// and the `class` attribute contains the C++ class nam
///
/// # Information on the original C++ class
/// -    size: 96
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkaBoneAttachment<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkaBoneAttachment"`: The original C++ class name.
    #[serde(default = "HkaBoneAttachment::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xa8ccd5cf`: Unique value of this class.
    #[serde(default = "HkaBoneAttachment::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkaBoneAttachmentHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkaBoneAttachmentHkParam<'a>>
}

impl HkaBoneAttachment<'_> {
    /// Return `"hkaBoneAttachment"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkaBoneAttachment".into()
    }

    /// Return `"0xa8ccd5cf"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xa8ccd5cf".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaBoneAttachmentHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"originalSkeletonName"`
    /// -   type: `hkStringPtr`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "originalSkeletonName")]
    OriginalSkeletonName(Primitive<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"boneFromAttachment"`
    /// -   type: `hkMatrix4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "boneFromAttachment")]
    BoneFromAttachment(Matrix4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"attachment"`
    /// -   type: `struct hkReferencedObject*`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "attachment")]
    Attachment(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"boneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "boneIndex")]
    BoneIndex(Primitive<i16>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkaBoneAttachmentHkParam<'de>, "@name",
    ("originalSkeletonName" => OriginalSkeletonName(Primitive<Cow<'a, str>>)),
    ("boneFromAttachment" => BoneFromAttachment(Matrix4<f32>)),
    ("attachment" => Attachment(Cow<'a, str>)),
    ("name" => Name(Primitive<Cow<'a, str>>)),
    ("boneIndex" => BoneIndex(Primitive<i16>)),
}
