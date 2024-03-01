//! A Rust structure that implements a serializer/deserializer corresponding to `hkbRoleAttribute`, a class defined in C++
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
/// -    size: 4
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbRoleAttribute<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbRoleAttribute"`: Name of this class.
    #[serde(default = "HkbRoleAttribute::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x3eb2e082`: Unique value of this class.
    #[serde(default = "HkbRoleAttribute::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbRoleAttributeHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbRoleAttributeHkParam<'a>>
}

impl HkbRoleAttribute<'_> {
    /// Return `"hkbRoleAttribute"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbRoleAttribute".into()
    }

    /// Return `"0x3eb2e082"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x3eb2e082".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbRoleAttributeHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"role"`
    /// -   type: `enum Role`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "role")]
    Role(Role),
    /// # Information on fields in the original C++ class
    /// -   name:`"flags"`
    /// -   type: `flags RoleFlags`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "flags")]
    Flags(RoleFlags),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbRoleAttributeHkParam<'de>, "@name",
    ("role" => Role(Role)),
    ("flags" => Flags(RoleFlags)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "ROLE_DEFAULT")]
    RoleDefault = 0,
    #[serde(rename = "ROLE_FILE_NAME")]
    RoleFileName = 1,
    #[serde(rename = "ROLE_BONE_INDEX")]
    RoleBoneIndex = 2,
    #[serde(rename = "ROLE_BONE_INDEX_MAP")]
    RoleBoneIndexMap = 3,
    #[serde(rename = "ROLE_EVENT_ID")]
    RoleEventId = 4,
    #[serde(rename = "ROLE_VARIABLE_INDEX")]
    RoleVariableIndex = 5,
    #[serde(rename = "ROLE_ATTRIBUTE_INDEX")]
    RoleAttributeIndex = 6,
    #[serde(rename = "ROLE_TIME")]
    RoleTime = 7,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RoleFlags {
    #[serde(rename = "FLAG_NONE")]
    FlagNone = 0,
    #[serde(rename = "FLAG_RAGDOLL")]
    FlagRagdoll = 1,
    #[serde(rename = "FLAG_NORMALIZED")]
    FlagNormalized = 2,
    #[serde(rename = "FLAG_NOT_VARIABLE")]
    FlagNotVariable = 4,
    #[serde(rename = "FLAG_HIDDEN")]
    FlagHidden = 8,
    #[serde(rename = "FLAG_OUTPUT")]
    FlagOutput = 16,
    #[serde(rename = "FLAG_NOT_CHARACTER_PROPERTY")]
    FlagNotCharacterProperty = 32,
}
