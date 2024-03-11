//! A Rust structure that implements a serializer/deserializer corresponding to `hkpDisplayBindingDataRigidBody`, a class defined in C++
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
/// -    size: 80
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 2
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpDisplayBindingDataRigidBody<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpDisplayBindingDataRigidBody"`: The original C++ class name.
    #[serde(default = "HkpDisplayBindingDataRigidBody::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xfe16e2a3`: Unique value of this class.
    #[serde(default = "HkpDisplayBindingDataRigidBody::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpDisplayBindingDataRigidBodyHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpDisplayBindingDataRigidBodyHkParam<'a>>
}

impl HkpDisplayBindingDataRigidBody<'_> {
    /// Return `"hkpDisplayBindingDataRigidBody"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpDisplayBindingDataRigidBody".into()
    }

    /// Return `"0xfe16e2a3"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xfe16e2a3".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpDisplayBindingDataRigidBodyHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"rigidBody"`
    /// -   type: `struct hkpRigidBody*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rigidBody")]
    RigidBody(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"displayObjectPtr"`
    /// -   type: `struct hkReferencedObject*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "displayObjectPtr")]
    DisplayObjectPtr(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"rigidBodyFromDisplayObjectTransform"`
    /// -   type: `hkMatrix4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rigidBodyFromDisplayObjectTransform")]
    RigidBodyFromDisplayObjectTransform(Matrix4<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpDisplayBindingDataRigidBodyHkParam<'de>, "@name",
    ("rigidBody" => RigidBody(Cow<'a, str>)),
    ("displayObjectPtr" => DisplayObjectPtr(Cow<'a, str>)),
    ("rigidBodyFromDisplayObjectTransform" => RigidBodyFromDisplayObjectTransform(Matrix4<f32>)),
}
