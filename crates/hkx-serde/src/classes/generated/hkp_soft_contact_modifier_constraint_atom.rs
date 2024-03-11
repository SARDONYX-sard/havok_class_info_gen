//! A Rust structure that implements a serializer/deserializer corresponding to `hkpSoftContactModifierConstraintAtom`, a class defined in C++
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
/// -    size: 48
/// -  vtable: false
/// -  parent: hkpModifierConstraintAtom/`b13fef1f`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpSoftContactModifierConstraintAtom<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpSoftContactModifierConstraintAtom"`: The original C++ class name.
    #[serde(default = "HkpSoftContactModifierConstraintAtom::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xecb34e27`: Unique value of this class.
    #[serde(default = "HkpSoftContactModifierConstraintAtom::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpSoftContactModifierConstraintAtomHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpSoftContactModifierConstraintAtomHkParam<'a>>
}

impl HkpSoftContactModifierConstraintAtom<'_> {
    /// Return `"hkpSoftContactModifierConstraintAtom"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpSoftContactModifierConstraintAtom".into()
    }

    /// Return `"0xecb34e27"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xecb34e27".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpSoftContactModifierConstraintAtomHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"tau"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "tau")]
    Tau(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"maxAcceleration"`
    /// -   type: `hkReal`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxAcceleration")]
    MaxAcceleration(Primitive<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpSoftContactModifierConstraintAtomHkParam<'de>, "@name",
    ("tau" => Tau(Primitive<f32>)),
    ("maxAcceleration" => MaxAcceleration(Primitive<f32>)),
}
