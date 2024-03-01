//! A Rust structure that implements a serializer/deserializer corresponding to `BSRagdollContactListenerModifier`, a class defined in C++
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
/// -    size: 76
/// -  vtable: true
/// -  parent: hkbModifier/`96ec5ced`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct BsRagdollContactListenerModifier<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"BSRagdollContactListenerModifier"`: Name of this class.
    #[serde(default = "BsRagdollContactListenerModifier::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x8003d8ce`: Unique value of this class.
    #[serde(default = "BsRagdollContactListenerModifier::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<BsRagdollContactListenerModifierHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<BsRagdollContactListenerModifierHkParam<'a>>
}

impl BsRagdollContactListenerModifier<'_> {
    /// Return `"BSRagdollContactListenerModifier"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "BsRagdollContactListenerModifier".into()
    }

    /// Return `"0x8003d8ce"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x8003d8ce".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsRagdollContactListenerModifierHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"contactEvent"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "contactEvent")]
    ContactEvent(HkbEventProperty),
    /// # Information on fields in the original C++ class
    /// -   name:`"bones"`
    /// -   type: `struct hkbBoneIndexArray*`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bones")]
    Bones(Box<HkbBoneIndexArray>),
    /// # Information on fields in the original C++ class
    /// -   name:`"throwEvent"`
    /// -   type: `hkBool`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "throwEvent", skip_serializing)]
    ThrowEvent(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"ragdollRigidBodies"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "ragdollRigidBodies", skip_serializing)]
    RagdollRigidBodies(Vec<()>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    BsRagdollContactListenerModifierHkParam<'de>, "@name",
    ("contactEvent" => ContactEvent(HkbEventProperty)),
    ("bones" => Bones(Box<HkbBoneIndexArray>)),
    ("throwEvent" => ThrowEvent(bool)),
    ("ragdollRigidBodies" => RagdollRigidBodies(Vec<()>)),
}
