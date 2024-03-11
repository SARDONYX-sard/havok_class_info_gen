//! A Rust structure that implements a serializer/deserializer corresponding to `hkpSimpleContactConstraintDataInfo`, a class defined in C++
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
/// -    size: 32
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpSimpleContactConstraintDataInfo<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpSimpleContactConstraintDataInfo"`: The original C++ class name.
    #[serde(default = "HkpSimpleContactConstraintDataInfo::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xb59d1734`: Unique value of this class.
    #[serde(default = "HkpSimpleContactConstraintDataInfo::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpSimpleContactConstraintDataInfoHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpSimpleContactConstraintDataInfoHkParam<'a>>
}

impl HkpSimpleContactConstraintDataInfo<'_> {
    /// Return `"hkpSimpleContactConstraintDataInfo"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpSimpleContactConstraintDataInfo".into()
    }

    /// Return `"0xb59d1734"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xb59d1734".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpSimpleContactConstraintDataInfoHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"flags"`
    /// -   type: `hkUint16`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE | ALIGN16`
    #[serde(rename = "flags")]
    Flags(Primitive<u16>),
    /// # Field information in the original C++ class
    /// -   name:`"index"`
    /// -   type: `hkUint16`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "index")]
    Index(Primitive<u16>),
    /// # Field information in the original C++ class
    /// -   name:`"internalData0"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "internalData0")]
    InternalData0(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"rollingFrictionMultiplier"`
    /// -   type: `hkHalf`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rollingFrictionMultiplier")]
    RollingFrictionMultiplier(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"internalData1"`
    /// -   type: `hkHalf`
    /// - offset: 10
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "internalData1")]
    InternalData1(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"data"`
    /// -   type: `hkUint32[5]`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "data")]
    Data([Primitive<u32>; 5]),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpSimpleContactConstraintDataInfoHkParam<'de>, "@name",
    ("flags" => Flags(Primitive<u16>)),
    ("index" => Index(Primitive<u16>)),
    ("internalData0" => InternalData0(Primitive<f32>)),
    ("rollingFrictionMultiplier" => RollingFrictionMultiplier(Primitive<f32>)),
    ("internalData1" => InternalData1(Primitive<f32>)),
    ("data" => Data([Primitive<u32>; 5])),
}
