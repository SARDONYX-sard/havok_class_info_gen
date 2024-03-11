//! A Rust structure that implements a serializer/deserializer corresponding to `hkContactPointMaterial`, a class defined in C++
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
/// -    size: 8
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkContactPointMaterial<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkContactPointMaterial"`: The original C++ class name.
    #[serde(default = "HkContactPointMaterial::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x4e32287c`: Unique value of this class.
    #[serde(default = "HkContactPointMaterial::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkContactPointMaterialHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkContactPointMaterialHkParam<'a>>
}

impl HkContactPointMaterial<'_> {
    /// Return `"hkContactPointMaterial"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkContactPointMaterial".into()
    }

    /// Return `"0x4e32287c"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x4e32287c".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkContactPointMaterialHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData")]
    UserData(Primitive<u64>),
    /// # Field information in the original C++ class
    /// -   name:`"friction"`
    /// -   type: `hkUint8`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "friction")]
    Friction(Primitive<u8>),
    /// # Field information in the original C++ class
    /// -   name:`"restitution"`
    /// -   type: `hkUint8`
    /// - offset: 5
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "restitution")]
    Restitution(Primitive<u8>),
    /// # Field information in the original C++ class
    /// -   name:`"maxImpulse"`
    /// -   type: `hkUint8`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxImpulse")]
    MaxImpulse(Primitive<u8>),
    /// # Field information in the original C++ class
    /// -   name:`"flags"`
    /// -   type: `hkUint8`
    /// - offset: 7
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "flags")]
    Flags(Primitive<u8>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkContactPointMaterialHkParam<'de>, "@name",
    ("userData" => UserData(Primitive<u64>)),
    ("friction" => Friction(Primitive<u8>)),
    ("restitution" => Restitution(Primitive<u8>)),
    ("maxImpulse" => MaxImpulse(Primitive<u8>)),
    ("flags" => Flags(Primitive<u8>)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FlagEnum {
    #[serde(rename = "CONTACT_IS_NEW")]
    ContactIsNew = 1,
    #[serde(rename = "CONTACT_USES_SOLVER_PATH2")]
    ContactUsesSolverPath2 = 2,
    #[serde(rename = "CONTACT_BREAKOFF_OBJECT_ID")]
    ContactBreakoffObjectId = 4,
    #[serde(rename = "CONTACT_IS_DISABLED")]
    ContactIsDisabled = 8,
}
