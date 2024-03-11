//! A Rust structure that implements a serializer/deserializer corresponding to `hkpSetLocalTransformsConstraintAtom`, a class defined in C++
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
/// -    size: 144
/// -  vtable: false
/// -  parent: hkpConstraintAtom/`59d67ef6`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpSetLocalTransformsConstraintAtom<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpSetLocalTransformsConstraintAtom"`: The original C++ class name.
    #[serde(default = "HkpSetLocalTransformsConstraintAtom::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x6e2a5198`: Unique value of this class.
    #[serde(default = "HkpSetLocalTransformsConstraintAtom::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpSetLocalTransformsConstraintAtomHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpSetLocalTransformsConstraintAtomHkParam<'a>>
}

impl HkpSetLocalTransformsConstraintAtom<'_> {
    /// Return `"hkpSetLocalTransformsConstraintAtom"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpSetLocalTransformsConstraintAtom".into()
    }

    /// Return `"0x6e2a5198"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x6e2a5198".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpSetLocalTransformsConstraintAtomHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"transformA"`
    /// -   type: `hkTransform`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transformA")]
    TransformA(Transform<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"transformB"`
    /// -   type: `hkTransform`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transformB")]
    TransformB(Transform<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpSetLocalTransformsConstraintAtomHkParam<'de>, "@name",
    ("transformA" => TransformA(Transform<f32>)),
    ("transformB" => TransformB(Transform<f32>)),
}
