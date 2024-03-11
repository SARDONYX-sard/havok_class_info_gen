//! A Rust structure that implements a serializer/deserializer corresponding to `hkpLinLimitConstraintAtom`, a class defined in C++
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
/// -    size: 12
/// -  vtable: false
/// -  parent: hkpConstraintAtom/`59d67ef6`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpLinLimitConstraintAtom<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpLinLimitConstraintAtom"`: The original C++ class name.
    #[serde(default = "HkpLinLimitConstraintAtom::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xa44d1b07`: Unique value of this class.
    #[serde(default = "HkpLinLimitConstraintAtom::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpLinLimitConstraintAtomHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpLinLimitConstraintAtomHkParam<'a>>
}

impl HkpLinLimitConstraintAtom<'_> {
    /// Return `"hkpLinLimitConstraintAtom"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpLinLimitConstraintAtom".into()
    }

    /// Return `"0xa44d1b07"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xa44d1b07".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpLinLimitConstraintAtomHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"axisIndex"`
    /// -   type: `hkUint8`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "axisIndex")]
    AxisIndex(Primitive<u8>),
    /// # Field information in the original C++ class
    /// -   name:`"min"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "min")]
    Min(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"max"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "max")]
    Max(Primitive<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpLinLimitConstraintAtomHkParam<'de>, "@name",
    ("axisIndex" => AxisIndex(Primitive<u8>)),
    ("min" => Min(Primitive<f32>)),
    ("max" => Max(Primitive<f32>)),
}
