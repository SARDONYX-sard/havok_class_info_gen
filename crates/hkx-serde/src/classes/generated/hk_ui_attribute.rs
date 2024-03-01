//! A Rust structure that implements a serializer/deserializer corresponding to `hkUiAttribute`, a class defined in C++
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
/// -    size: 20
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 2
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkUiAttribute<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkUiAttribute"`: Name of this class.
    #[serde(default = "HkUiAttribute::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xeb6e96e3`: Unique value of this class.
    #[serde(default = "HkUiAttribute::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkUiAttributeHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkUiAttributeHkParam<'a>>
}

impl HkUiAttribute<'_> {
    /// Return `"hkUiAttribute"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkUiAttribute".into()
    }

    /// Return `"0xeb6e96e3"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xeb6e96e3".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkUiAttributeHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"visible"`
    /// -   type: `hkBool`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "visible")]
    Visible(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"hideInModeler"`
    /// -   type: `enum HideInModeler`
    /// - offset: 1
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "hideInModeler")]
    HideInModeler(HideInModeler),
    /// # Information on fields in the original C++ class
    /// -   name:`"label"`
    /// -   type: `char*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "label")]
    Label(String),
    /// # Information on fields in the original C++ class
    /// -   name:`"group"`
    /// -   type: `char*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "group")]
    Group(String),
    /// # Information on fields in the original C++ class
    /// -   name:`"hideBaseClassMembers"`
    /// -   type: `char*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "hideBaseClassMembers")]
    HideBaseClassMembers(String),
    /// # Information on fields in the original C++ class
    /// -   name:`"endGroup"`
    /// -   type: `hkBool`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "endGroup")]
    EndGroup(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"endGroup2"`
    /// -   type: `hkBool`
    /// - offset: 17
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "endGroup2")]
    EndGroup2(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"advanced"`
    /// -   type: `hkBool`
    /// - offset: 18
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "advanced")]
    Advanced(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkUiAttributeHkParam<'de>, "@name",
    ("visible" => Visible(bool)),
    ("hideInModeler" => HideInModeler(HideInModeler)),
    ("label" => Label(String)),
    ("group" => Group(String)),
    ("hideBaseClassMembers" => HideBaseClassMembers(String)),
    ("endGroup" => EndGroup(bool)),
    ("endGroup2" => EndGroup2(bool)),
    ("advanced" => Advanced(bool)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum HideInModeler {
    #[serde(rename = "NONE")]
    None = 0,
    #[serde(rename = "MAX")]
    Max = 1,
    #[serde(rename = "MAYA")]
    Maya = 2,
}
