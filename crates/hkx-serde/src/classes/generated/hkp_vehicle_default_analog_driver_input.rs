//! A Rust structure that implements a serializer/deserializer corresponding to `hkpVehicleDefaultAnalogDriverInput`, a class defined in C++
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
/// -    size: 24
/// -  vtable: true
/// -  parent: hkpVehicleDriverInput/`da8c7d7d`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpVehicleDefaultAnalogDriverInput<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpVehicleDefaultAnalogDriverInput"`: Name of this class.
    #[serde(default = "HkpVehicleDefaultAnalogDriverInput::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x123a5d50`: Unique value of this class.
    #[serde(default = "HkpVehicleDefaultAnalogDriverInput::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpVehicleDefaultAnalogDriverInputHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpVehicleDefaultAnalogDriverInputHkParam<'a>>
}

impl HkpVehicleDefaultAnalogDriverInput<'_> {
    /// Return `"hkpVehicleDefaultAnalogDriverInput"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpVehicleDefaultAnalogDriverInput".into()
    }

    /// Return `"0x123a5d50"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x123a5d50".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleDefaultAnalogDriverInputHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"slopeChangePointX"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "slopeChangePointX")]
    SlopeChangePointX(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"initialSlope"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "initialSlope")]
    InitialSlope(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"deadZone"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "deadZone")]
    DeadZone(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"autoReverse"`
    /// -   type: `hkBool`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "autoReverse")]
    AutoReverse(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleDefaultAnalogDriverInputHkParam<'de>, "@name",
    ("slopeChangePointX" => SlopeChangePointX(f64)),
    ("initialSlope" => InitialSlope(f64)),
    ("deadZone" => DeadZone(f64)),
    ("autoReverse" => AutoReverse(bool)),
}
