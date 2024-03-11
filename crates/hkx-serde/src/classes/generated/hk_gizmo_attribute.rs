//! A Rust structure that implements a serializer/deserializer corresponding to `hkGizmoAttribute`, a class defined in C++
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
/// -    size: 12
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkGizmoAttribute<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkGizmoAttribute"`: The original C++ class name.
    #[serde(default = "HkGizmoAttribute::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x23aadfb6`: Unique value of this class.
    #[serde(default = "HkGizmoAttribute::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkGizmoAttributeHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkGizmoAttributeHkParam<'a>>
}

impl HkGizmoAttribute<'_> {
    /// Return `"hkGizmoAttribute"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkGizmoAttribute".into()
    }

    /// Return `"0x23aadfb6"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x23aadfb6".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkGizmoAttributeHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"visible"`
    /// -   type: `hkBool`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "visible")]
    Visible(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"label"`
    /// -   type: `char*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "label")]
    Label(Primitive<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"type"`
    /// -   type: `enum GizmoType`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(GizmoType),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkGizmoAttributeHkParam<'de>, "@name",
    ("visible" => Visible(Primitive<bool>)),
    ("label" => Label(Primitive<Cow<'a, str>>)),
    ("type" => Type(GizmoType)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GizmoType {
    #[serde(rename = "POINT")]
    Point = 0,
    #[serde(rename = "SPHERE")]
    Sphere = 1,
    #[serde(rename = "PLANE")]
    Plane = 2,
    #[serde(rename = "ARROW")]
    Arrow = 3,
}
