//! A Rust structure that implements a serializer/deserializer corresponding to `hkpRackAndPinionConstraintData`, a class defined in C++
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
/// -    size: 176
/// -  vtable: true
/// -  parent: hkpConstraintData/`80559a4e`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpRackAndPinionConstraintData<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpRackAndPinionConstraintData"`: The original C++ class name.
    #[serde(default = "HkpRackAndPinionConstraintData::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xd180ebe0`: Unique value of this class.
    #[serde(default = "HkpRackAndPinionConstraintData::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpRackAndPinionConstraintDataHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpRackAndPinionConstraintDataHkParam<'a>>
}

impl HkpRackAndPinionConstraintData<'_> {
    /// Return `"hkpRackAndPinionConstraintData"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpRackAndPinionConstraintData".into()
    }

    /// Return `"0xd180ebe0"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xd180ebe0".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpRackAndPinionConstraintDataHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"atoms"`
    /// -   type: `struct hkpRackAndPinionConstraintDataAtoms`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE | ALIGN16`
    #[serde(rename = "atoms")]
    Atoms(HkpRackAndPinionConstraintDataAtoms),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpRackAndPinionConstraintDataHkParam<'de>, "@name",
    ("atoms" => Atoms(HkpRackAndPinionConstraintDataAtoms)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "TYPE_RACK_AND_PINION")]
    TypeRackAndPinion = 0,
    #[serde(rename = "TYPE_SCREW")]
    TypeScrew = 1,
}
