//! A Rust structure that implements a serializer/deserializer corresponding to `hkpSetLocalRotationsConstraintAtom`, a class defined in C++
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
/// -    size: 112
/// -  vtable: false
/// -  parent: hkpConstraintAtom/`59d67ef6`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpSetLocalRotationsConstraintAtom<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpSetLocalRotationsConstraintAtom"`: The original C++ class name.
    #[serde(default = "HkpSetLocalRotationsConstraintAtom::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xf81db8e`: Unique value of this class.
    #[serde(default = "HkpSetLocalRotationsConstraintAtom::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpSetLocalRotationsConstraintAtomHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpSetLocalRotationsConstraintAtomHkParam<'a>>
}

impl HkpSetLocalRotationsConstraintAtom<'_> {
    /// Return `"hkpSetLocalRotationsConstraintAtom"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpSetLocalRotationsConstraintAtom".into()
    }

    /// Return `"0xf81db8e"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xf81db8e".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpSetLocalRotationsConstraintAtomHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"rotationA"`
    /// -   type: `hkRotation`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotationA")]
    RotationA(Rotation<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"rotationB"`
    /// -   type: `hkRotation`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotationB")]
    RotationB(Rotation<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpSetLocalRotationsConstraintAtomHkParam<'de>, "@name",
    ("rotationA" => RotationA(Rotation<f32>)),
    ("rotationB" => RotationB(Rotation<f32>)),
}
