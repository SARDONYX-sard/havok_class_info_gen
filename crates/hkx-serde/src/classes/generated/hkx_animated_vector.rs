//! A Rust structure that implements a serializer/deserializer corresponding to `hkxAnimatedVector`, a class defined in C++
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
/// -    size: 24
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkxAnimatedVector<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkxAnimatedVector"`: The original C++ class name.
    #[serde(default = "HkxAnimatedVector::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x34b1a197`: Unique value of this class.
    #[serde(default = "HkxAnimatedVector::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkxAnimatedVectorHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkxAnimatedVectorHkParam<'a>>
}

impl HkxAnimatedVector<'_> {
    /// Return `"hkxAnimatedVector"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkxAnimatedVector".into()
    }

    /// Return `"0x34b1a197"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x34b1a197".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxAnimatedVectorHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"vectors"`
    /// -   type: `hkArray&lt;hkVector4&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vectors")]
    Vectors(Vec<Vector4<f32>>),
    /// # Field information in the original C++ class
    /// -   name:`"hint"`
    /// -   type: `enum Hint`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "hint")]
    Hint(Hint),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkxAnimatedVectorHkParam<'de>, "@name",
    ("vectors" => Vectors(Vec<Vector4<f32>>)),
    ("hint" => Hint(Hint)),
}
