//! A Rust structure that implements a serializer/deserializer corresponding to `hkpPhysicsSystem`, a class defined in C++
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
/// -    size: 68
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpPhysicsSystem<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpPhysicsSystem"`: Name of this class.
    #[serde(default = "HkpPhysicsSystem::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xff724c17`: Unique value of this class.
    #[serde(default = "HkpPhysicsSystem::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpPhysicsSystemHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpPhysicsSystemHkParam<'a>>
}

impl HkpPhysicsSystem<'_> {
    /// Return `"hkpPhysicsSystem"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpPhysicsSystem".into()
    }

    /// Return `"0xff724c17"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xff724c17".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpPhysicsSystemHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"rigidBodies"`
    /// -   type: `hkArray&lt;hkpRigidBody*&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rigidBodies")]
    RigidBodies(Vec<Box<HkpRigidBody>>),
    /// # Information on fields in the original C++ class
    /// -   name:`"constraints"`
    /// -   type: `hkArray&lt;hkpConstraintInstance*&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "constraints")]
    Constraints(Vec<Box<HkpConstraintInstance>>),
    /// # Information on fields in the original C++ class
    /// -   name:`"actions"`
    /// -   type: `hkArray&lt;hkpAction*&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "actions")]
    Actions(Vec<Box<HkpAction>>),
    /// # Information on fields in the original C++ class
    /// -   name:`"phantoms"`
    /// -   type: `hkArray&lt;hkpPhantom*&gt;`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "phantoms")]
    Phantoms(Vec<Box<HkpPhantom>>),
    /// # Information on fields in the original C++ class
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(String),
    /// # Information on fields in the original C++ class
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData")]
    UserData(u64),
    /// # Information on fields in the original C++ class
    /// -   name:`"active"`
    /// -   type: `hkBool`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "active")]
    Active(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpPhysicsSystemHkParam<'de>, "@name",
    ("rigidBodies" => RigidBodies(Vec<Box<HkpRigidBody>>)),
    ("constraints" => Constraints(Vec<Box<HkpConstraintInstance>>)),
    ("actions" => Actions(Vec<Box<HkpAction>>)),
    ("phantoms" => Phantoms(Vec<Box<HkpPhantom>>)),
    ("name" => Name(String)),
    ("userData" => UserData(u64)),
    ("active" => Active(bool)),
}