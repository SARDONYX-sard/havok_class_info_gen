//! A Rust structure that implements a serializer/deserializer corresponding to `hkpBallSocketChainData`, a class defined in C++
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
/// -    size: 52
/// -  vtable: true
/// -  parent: hkpConstraintChainData/`5facc7ff`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpBallSocketChainData<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpBallSocketChainData"`: The original C++ class name.
    #[serde(default = "HkpBallSocketChainData::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x102aae9c`: Unique value of this class.
    #[serde(default = "HkpBallSocketChainData::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpBallSocketChainDataHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpBallSocketChainDataHkParam<'a>>
}

impl HkpBallSocketChainData<'_> {
    /// Return `"hkpBallSocketChainData"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpBallSocketChainData".into()
    }

    /// Return `"0x102aae9c"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x102aae9c".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpBallSocketChainDataHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"atoms"`
    /// -   type: `struct hkpBridgeAtoms`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "atoms")]
    Atoms(HkpBridgeAtoms),
    /// # Field information in the original C++ class
    /// -   name:`"infos"`
    /// -   type: `hkArray&lt;struct hkpBallSocketChainDataConstraintInfo&gt;`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "infos")]
    Infos(Vec<HkpBallSocketChainDataConstraintInfo>),
    /// # Field information in the original C++ class
    /// -   name:`"tau"`
    /// -   type: `hkReal`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "tau")]
    Tau(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"damping"`
    /// -   type: `hkReal`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "damping")]
    Damping(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"cfm"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "cfm")]
    Cfm(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"maxErrorDistance"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxErrorDistance")]
    MaxErrorDistance(Primitive<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpBallSocketChainDataHkParam<'de>, "@name",
    ("atoms" => Atoms(HkpBridgeAtoms)),
    ("infos" => Infos(Vec<HkpBallSocketChainDataConstraintInfo>)),
    ("tau" => Tau(Primitive<f32>)),
    ("damping" => Damping(Primitive<f32>)),
    ("cfm" => Cfm(Primitive<f32>)),
    ("maxErrorDistance" => MaxErrorDistance(Primitive<f32>)),
}
