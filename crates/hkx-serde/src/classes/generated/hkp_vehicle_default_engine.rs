//! A Rust structure that implements a serializer/deserializer corresponding to `hkpVehicleDefaultEngine`, a class defined in C++
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
/// -    size: 48
/// -  vtable: true
/// -  parent: hkpVehicleEngine/`da8c7d7d`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpVehicleDefaultEngine<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpVehicleDefaultEngine"`: Name of this class.
    #[serde(default = "HkpVehicleDefaultEngine::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x56f8ca24`: Unique value of this class.
    #[serde(default = "HkpVehicleDefaultEngine::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpVehicleDefaultEngineHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpVehicleDefaultEngineHkParam<'a>>
}

impl HkpVehicleDefaultEngine<'_> {
    /// Return `"hkpVehicleDefaultEngine"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpVehicleDefaultEngine".into()
    }

    /// Return `"0x56f8ca24"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x56f8ca24".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleDefaultEngineHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"minRPM"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minRPM")]
    MinRpm(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"optRPM"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "optRPM")]
    OptRpm(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"maxRPM"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxRPM")]
    MaxRpm(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"maxTorque"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxTorque")]
    MaxTorque(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"torqueFactorAtMinRPM"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "torqueFactorAtMinRPM")]
    TorqueFactorAtMinRpm(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"torqueFactorAtMaxRPM"`
    /// -   type: `hkReal`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "torqueFactorAtMaxRPM")]
    TorqueFactorAtMaxRpm(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"resistanceFactorAtMinRPM"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "resistanceFactorAtMinRPM")]
    ResistanceFactorAtMinRpm(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"resistanceFactorAtOptRPM"`
    /// -   type: `hkReal`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "resistanceFactorAtOptRPM")]
    ResistanceFactorAtOptRpm(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"resistanceFactorAtMaxRPM"`
    /// -   type: `hkReal`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "resistanceFactorAtMaxRPM")]
    ResistanceFactorAtMaxRpm(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"clutchSlipRPM"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "clutchSlipRPM")]
    ClutchSlipRpm(f64),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleDefaultEngineHkParam<'de>, "@name",
    ("minRPM" => MinRpm(f64)),
    ("optRPM" => OptRpm(f64)),
    ("maxRPM" => MaxRpm(f64)),
    ("maxTorque" => MaxTorque(f64)),
    ("torqueFactorAtMinRPM" => TorqueFactorAtMinRpm(f64)),
    ("torqueFactorAtMaxRPM" => TorqueFactorAtMaxRpm(f64)),
    ("resistanceFactorAtMinRPM" => ResistanceFactorAtMinRpm(f64)),
    ("resistanceFactorAtOptRPM" => ResistanceFactorAtOptRpm(f64)),
    ("resistanceFactorAtMaxRPM" => ResistanceFactorAtMaxRpm(f64)),
    ("clutchSlipRPM" => ClutchSlipRpm(f64)),
}
