//! A Rust structure that implements a serializer/deserializer corresponding to `hkpTyremarksInfo`, a class defined in C++
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
/// -    size: 28
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpTyremarksInfo<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpTyremarksInfo"`: Name of this class.
    #[serde(default = "HkpTyremarksInfo::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x3d0433d6`: Unique value of this class.
    #[serde(default = "HkpTyremarksInfo::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpTyremarksInfoHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpTyremarksInfoHkParam<'a>>
}

impl HkpTyremarksInfo<'_> {
    /// Return `"hkpTyremarksInfo"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpTyremarksInfo".into()
    }

    /// Return `"0x3d0433d6"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x3d0433d6".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpTyremarksInfoHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"minTyremarkEnergy"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minTyremarkEnergy")]
    MinTyremarkEnergy(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"maxTyremarkEnergy"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxTyremarkEnergy")]
    MaxTyremarkEnergy(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"tyremarksWheel"`
    /// -   type: `hkArray&lt;hkpTyremarksWheel*&gt;`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "tyremarksWheel")]
    TyremarksWheel(Vec<Box<HkpTyremarksWheel>>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpTyremarksInfoHkParam<'de>, "@name",
    ("minTyremarkEnergy" => MinTyremarkEnergy(f64)),
    ("maxTyremarkEnergy" => MaxTyremarkEnergy(f64)),
    ("tyremarksWheel" => TyremarksWheel(Vec<Box<HkpTyremarksWheel>>)),
}
