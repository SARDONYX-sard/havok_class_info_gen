//! A Rust structure that implements a serializer/deserializer corresponding to `hkpBallAndSocketConstraintDataAtoms`, a class defined in C++
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
/// -    size: 80
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpBallAndSocketConstraintDataAtoms<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpBallAndSocketConstraintDataAtoms"`: Name of this class.
    #[serde(default = "HkpBallAndSocketConstraintDataAtoms::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xc73dcaf9`: Unique value of this class.
    #[serde(default = "HkpBallAndSocketConstraintDataAtoms::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpBallAndSocketConstraintDataAtomsHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpBallAndSocketConstraintDataAtomsHkParam<'a>>
}

impl HkpBallAndSocketConstraintDataAtoms<'_> {
    /// Return `"hkpBallAndSocketConstraintDataAtoms"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpBallAndSocketConstraintDataAtoms".into()
    }

    /// Return `"0xc73dcaf9"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xc73dcaf9".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpBallAndSocketConstraintDataAtomsHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"pivots"`
    /// -   type: `struct hkpSetLocalTranslationsConstraintAtom`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pivots")]
    Pivots(HkpSetLocalTranslationsConstraintAtom),
    /// # Information on fields in the original C++ class
    /// -   name:`"setupStabilization"`
    /// -   type: `struct hkpSetupStabilizationAtom`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "setupStabilization")]
    SetupStabilization(HkpSetupStabilizationAtom),
    /// # Information on fields in the original C++ class
    /// -   name:`"ballSocket"`
    /// -   type: `struct hkpBallSocketConstraintAtom`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ballSocket")]
    BallSocket(HkpBallSocketConstraintAtom),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpBallAndSocketConstraintDataAtomsHkParam<'de>, "@name",
    ("pivots" => Pivots(HkpSetLocalTranslationsConstraintAtom)),
    ("setupStabilization" => SetupStabilization(HkpSetupStabilizationAtom)),
    ("ballSocket" => BallSocket(HkpBallSocketConstraintAtom)),
}
