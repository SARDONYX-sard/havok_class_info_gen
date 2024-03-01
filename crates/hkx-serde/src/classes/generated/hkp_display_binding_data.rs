//! A Rust structure that implements a serializer/deserializer corresponding to `hkpDisplayBindingData`, a class defined in C++
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
/// -    size: 32
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpDisplayBindingData<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpDisplayBindingData"`: Name of this class.
    #[serde(default = "HkpDisplayBindingData::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xdc46c906`: Unique value of this class.
    #[serde(default = "HkpDisplayBindingData::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpDisplayBindingDataHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpDisplayBindingDataHkParam<'a>>
}

impl HkpDisplayBindingData<'_> {
    /// Return `"hkpDisplayBindingData"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpDisplayBindingData".into()
    }

    /// Return `"0xdc46c906"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xdc46c906".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpDisplayBindingDataHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"rigidBodyBindings"`
    /// -   type: `hkArray&lt;hkpDisplayBindingDataRigidBody*&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rigidBodyBindings")]
    RigidBodyBindings(Vec<Box<HkpDisplayBindingDataRigidBody>>),
    /// # Information on fields in the original C++ class
    /// -   name:`"physicsSystemBindings"`
    /// -   type: `hkArray&lt;hkpDisplayBindingDataPhysicsSystem*&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "physicsSystemBindings")]
    PhysicsSystemBindings(Vec<Box<HkpDisplayBindingDataPhysicsSystem>>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpDisplayBindingDataHkParam<'de>, "@name",
    ("rigidBodyBindings" => RigidBodyBindings(Vec<Box<HkpDisplayBindingDataRigidBody>>)),
    ("physicsSystemBindings" => PhysicsSystemBindings(Vec<Box<HkpDisplayBindingDataPhysicsSystem>>)),
}
