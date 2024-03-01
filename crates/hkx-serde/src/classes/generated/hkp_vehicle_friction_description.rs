//! A Rust structure that implements a serializer/deserializer corresponding to `hkpVehicleFrictionDescription`, a class defined in C++
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
/// -    size: 208
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpVehicleFrictionDescription<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpVehicleFrictionDescription"`: Name of this class.
    #[serde(default = "HkpVehicleFrictionDescription::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x1034549a`: Unique value of this class.
    #[serde(default = "HkpVehicleFrictionDescription::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpVehicleFrictionDescriptionHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpVehicleFrictionDescriptionHkParam<'a>>
}

impl HkpVehicleFrictionDescription<'_> {
    /// Return `"hkpVehicleFrictionDescription"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpVehicleFrictionDescription".into()
    }

    /// Return `"0x1034549a"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x1034549a".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleFrictionDescriptionHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"wheelDistance"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wheelDistance")]
    WheelDistance(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"chassisMassInv"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "chassisMassInv")]
    ChassisMassInv(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"axleDescr"`
    /// -   type: `struct hkpVehicleFrictionDescriptionAxisDescription[2]`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "axleDescr")]
    AxleDescr(HkpVehicleFrictionDescriptionAxisDescription[2]),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleFrictionDescriptionHkParam<'de>, "@name",
    ("wheelDistance" => WheelDistance(f64)),
    ("chassisMassInv" => ChassisMassInv(f64)),
    ("axleDescr" => AxleDescr(HkpVehicleFrictionDescriptionAxisDescription[2])),
}
