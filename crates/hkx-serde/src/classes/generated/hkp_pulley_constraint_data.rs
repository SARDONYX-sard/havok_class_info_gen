//! A Rust structure that implements a serializer/deserializer corresponding to `hkpPulleyConstraintData`, a class defined in C++
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
/// -    size: 128
/// -  vtable: true
/// -  parent: hkpConstraintData/`80559a4e`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpPulleyConstraintData<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpPulleyConstraintData"`: Name of this class.
    #[serde(default = "HkpPulleyConstraintData::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x972058ed`: Unique value of this class.
    #[serde(default = "HkpPulleyConstraintData::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpPulleyConstraintDataHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpPulleyConstraintDataHkParam<'a>>
}

impl HkpPulleyConstraintData<'_> {
    /// Return `"hkpPulleyConstraintData"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpPulleyConstraintData".into()
    }

    /// Return `"0x972058ed"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x972058ed".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpPulleyConstraintDataHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"atoms"`
    /// -   type: `struct hkpPulleyConstraintDataAtoms`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE | ALIGN16`
    #[serde(rename = "atoms")]
    Atoms(HkpPulleyConstraintDataAtoms),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpPulleyConstraintDataHkParam<'de>, "@name",
    ("atoms" => Atoms(HkpPulleyConstraintDataAtoms)),
}