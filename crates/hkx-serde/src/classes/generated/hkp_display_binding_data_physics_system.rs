//! A Rust structure that implements a serializer/deserializer corresponding to `hkpDisplayBindingDataPhysicsSystem`, a class defined in C++
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
/// -    size: 24
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpDisplayBindingDataPhysicsSystem<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpDisplayBindingDataPhysicsSystem"`: Name of this class.
    #[serde(default = "HkpDisplayBindingDataPhysicsSystem::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xc8ae86a7`: Unique value of this class.
    #[serde(default = "HkpDisplayBindingDataPhysicsSystem::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpDisplayBindingDataPhysicsSystemHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpDisplayBindingDataPhysicsSystemHkParam<'a>>
}

impl HkpDisplayBindingDataPhysicsSystem<'_> {
    /// Return `"hkpDisplayBindingDataPhysicsSystem"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpDisplayBindingDataPhysicsSystem".into()
    }

    /// Return `"0xc8ae86a7"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xc8ae86a7".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpDisplayBindingDataPhysicsSystemHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"bindings"`
    /// -   type: `hkArray&lt;hkpDisplayBindingDataRigidBody*&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bindings")]
    Bindings(Vec<Box<HkpDisplayBindingDataRigidBody>>),
    /// # Information on fields in the original C++ class
    /// -   name:`"system"`
    /// -   type: `struct hkpPhysicsSystem*`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "system")]
    System(Box<HkpPhysicsSystem>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpDisplayBindingDataPhysicsSystemHkParam<'de>, "@name",
    ("bindings" => Bindings(Vec<Box<HkpDisplayBindingDataRigidBody>>)),
    ("system" => System(Box<HkpPhysicsSystem>)),
}
