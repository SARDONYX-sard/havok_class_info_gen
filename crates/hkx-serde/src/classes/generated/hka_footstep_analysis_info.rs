//! A Rust structure that implements a serializer/deserializer corresponding to `hkaFootstepAnalysisInfo`, a class defined in C++
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
/// -    size: 152
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkaFootstepAnalysisInfo<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkaFootstepAnalysisInfo"`: Name of this class.
    #[serde(default = "HkaFootstepAnalysisInfo::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x824faf75`: Unique value of this class.
    #[serde(default = "HkaFootstepAnalysisInfo::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkaFootstepAnalysisInfoHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkaFootstepAnalysisInfoHkParam<'a>>
}

impl HkaFootstepAnalysisInfo<'_> {
    /// Return `"hkaFootstepAnalysisInfo"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkaFootstepAnalysisInfo".into()
    }

    /// Return `"0x824faf75"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x824faf75".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaFootstepAnalysisInfoHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"name"`
    /// -   type: `hkArray&lt;hkChar&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Vec<char>),
    /// # Information on fields in the original C++ class
    /// -   name:`"nameStrike"`
    /// -   type: `hkArray&lt;hkChar&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nameStrike")]
    NameStrike(Vec<char>),
    /// # Information on fields in the original C++ class
    /// -   name:`"nameLift"`
    /// -   type: `hkArray&lt;hkChar&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nameLift")]
    NameLift(Vec<char>),
    /// # Information on fields in the original C++ class
    /// -   name:`"nameLock"`
    /// -   type: `hkArray&lt;hkChar&gt;`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nameLock")]
    NameLock(Vec<char>),
    /// # Information on fields in the original C++ class
    /// -   name:`"nameUnlock"`
    /// -   type: `hkArray&lt;hkChar&gt;`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nameUnlock")]
    NameUnlock(Vec<char>),
    /// # Information on fields in the original C++ class
    /// -   name:`"minPos"`
    /// -   type: `hkArray&lt;hkReal&gt;`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minPos")]
    MinPos(Vec<f64>),
    /// # Information on fields in the original C++ class
    /// -   name:`"maxPos"`
    /// -   type: `hkArray&lt;hkReal&gt;`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxPos")]
    MaxPos(Vec<f64>),
    /// # Information on fields in the original C++ class
    /// -   name:`"minVel"`
    /// -   type: `hkArray&lt;hkReal&gt;`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minVel")]
    MinVel(Vec<f64>),
    /// # Information on fields in the original C++ class
    /// -   name:`"maxVel"`
    /// -   type: `hkArray&lt;hkReal&gt;`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxVel")]
    MaxVel(Vec<f64>),
    /// # Information on fields in the original C++ class
    /// -   name:`"allBonesDown"`
    /// -   type: `hkArray&lt;hkReal&gt;`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "allBonesDown")]
    AllBonesDown(Vec<f64>),
    /// # Information on fields in the original C++ class
    /// -   name:`"anyBonesDown"`
    /// -   type: `hkArray&lt;hkReal&gt;`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "anyBonesDown")]
    AnyBonesDown(Vec<f64>),
    /// # Information on fields in the original C++ class
    /// -   name:`"posTol"`
    /// -   type: `hkReal`
    /// - offset: 140
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "posTol")]
    PosTol(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"velTol"`
    /// -   type: `hkReal`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "velTol")]
    VelTol(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"duration"`
    /// -   type: `hkReal`
    /// - offset: 148
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "duration")]
    Duration(f64),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkaFootstepAnalysisInfoHkParam<'de>, "@name",
    ("name" => Name(Vec<char>)),
    ("nameStrike" => NameStrike(Vec<char>)),
    ("nameLift" => NameLift(Vec<char>)),
    ("nameLock" => NameLock(Vec<char>)),
    ("nameUnlock" => NameUnlock(Vec<char>)),
    ("minPos" => MinPos(Vec<f64>)),
    ("maxPos" => MaxPos(Vec<f64>)),
    ("minVel" => MinVel(Vec<f64>)),
    ("maxVel" => MaxVel(Vec<f64>)),
    ("allBonesDown" => AllBonesDown(Vec<f64>)),
    ("anyBonesDown" => AnyBonesDown(Vec<f64>)),
    ("posTol" => PosTol(f64)),
    ("velTol" => VelTol(f64)),
    ("duration" => Duration(f64)),
}