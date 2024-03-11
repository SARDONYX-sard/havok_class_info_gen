//! A Rust structure that implements a serializer/deserializer corresponding to `hkpPoweredChainData`, a class defined in C++
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
/// -    size: 64
/// -  vtable: true
/// -  parent: hkpConstraintChainData/`5facc7ff`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpPoweredChainData<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpPoweredChainData"`: The original C++ class name.
    #[serde(default = "HkpPoweredChainData::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x38aeafc3`: Unique value of this class.
    #[serde(default = "HkpPoweredChainData::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpPoweredChainDataHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpPoweredChainDataHkParam<'a>>
}

impl HkpPoweredChainData<'_> {
    /// Return `"hkpPoweredChainData"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpPoweredChainData".into()
    }

    /// Return `"0x38aeafc3"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x38aeafc3".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpPoweredChainDataHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"atoms"`
    /// -   type: `struct hkpBridgeAtoms`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "atoms")]
    Atoms(HkpBridgeAtoms),
    /// # Field information in the original C++ class
    /// -   name:`"infos"`
    /// -   type: `hkArray&lt;struct hkpPoweredChainDataConstraintInfo&gt;`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "infos")]
    Infos(Vec<HkpPoweredChainDataConstraintInfo>),
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
    /// -   name:`"cfmLinAdd"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "cfmLinAdd")]
    CfmLinAdd(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"cfmLinMul"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "cfmLinMul")]
    CfmLinMul(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"cfmAngAdd"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "cfmAngAdd")]
    CfmAngAdd(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"cfmAngMul"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "cfmAngMul")]
    CfmAngMul(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"maxErrorDistance"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxErrorDistance")]
    MaxErrorDistance(Primitive<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpPoweredChainDataHkParam<'de>, "@name",
    ("atoms" => Atoms(HkpBridgeAtoms)),
    ("infos" => Infos(Vec<HkpPoweredChainDataConstraintInfo>)),
    ("tau" => Tau(Primitive<f32>)),
    ("damping" => Damping(Primitive<f32>)),
    ("cfmLinAdd" => CfmLinAdd(Primitive<f32>)),
    ("cfmLinMul" => CfmLinMul(Primitive<f32>)),
    ("cfmAngAdd" => CfmAngAdd(Primitive<f32>)),
    ("cfmAngMul" => CfmAngMul(Primitive<f32>)),
    ("maxErrorDistance" => MaxErrorDistance(Primitive<f32>)),
}
