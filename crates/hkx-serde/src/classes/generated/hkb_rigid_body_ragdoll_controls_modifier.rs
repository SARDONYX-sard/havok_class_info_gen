//! A Rust structure that implements a serializer/deserializer corresponding to `hkbRigidBodyRagdollControlsModifier`, a class defined in C++
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
/// -    size: 128
/// -  vtable: true
/// -  parent: hkbModifier/`96ec5ced`(Non prefix hex signature)
/// - version: 3
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbRigidBodyRagdollControlsModifier<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbRigidBodyRagdollControlsModifier"`: The original C++ class name.
    #[serde(default = "HkbRigidBodyRagdollControlsModifier::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xaa87d1eb`: Unique value of this class.
    #[serde(default = "HkbRigidBodyRagdollControlsModifier::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbRigidBodyRagdollControlsModifierHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbRigidBodyRagdollControlsModifierHkParam<'a>>
}

impl HkbRigidBodyRagdollControlsModifier<'_> {
    /// Return `"hkbRigidBodyRagdollControlsModifier"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbRigidBodyRagdollControlsModifier".into()
    }

    /// Return `"0xaa87d1eb"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xaa87d1eb".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbRigidBodyRagdollControlsModifierHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"controlData"`
    /// -   type: `struct hkbRigidBodyRagdollControlData`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "controlData")]
    ControlData(HkbRigidBodyRagdollControlData),
    /// # Field information in the original C++ class
    /// -   name:`"bones"`
    /// -   type: `struct hkbBoneIndexArray*`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bones")]
    Bones(Cow<'a, str>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbRigidBodyRagdollControlsModifierHkParam<'de>, "@name",
    ("controlData" => ControlData(HkbRigidBodyRagdollControlData)),
    ("bones" => Bones(Cow<'a, str>)),
}
