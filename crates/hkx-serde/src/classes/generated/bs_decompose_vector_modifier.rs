//! A Rust structure that implements a serializer/deserializer corresponding to `BSDecomposeVectorModifier`, a class defined in C++
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// In XML, it is enclosed in a `hkobject` tag
/// and the `class` attribute contains the C++ class nam
///
/// # Information on the original C++ class
/// -    size: 80
/// -  vtable: true
/// -  parent: hkbModifier/`96ec5ced`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct BsDecomposeVectorModifier<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"BSDecomposeVectorModifier"`: Name of this class.
    #[serde(default = "BsDecomposeVectorModifier::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x31f6b8b6`: Unique value of this class.
    #[serde(default = "BsDecomposeVectorModifier::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<BsDecomposeVectorModifierHkParam>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<BsDecomposeVectorModifierHkParam>,
}

impl BsDecomposeVectorModifier<'_> {
    /// Return `"BSDecomposeVectorModifier"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "BsDecomposeVectorModifier".into()
    }

    /// Return `"0x31f6b8b6"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x31f6b8b6".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsDecomposeVectorModifierHkParam {
    /// # Information on fields in the original C++ class
    /// -   name:`"vector"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vector")]
    Vector(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"x"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "x")]
    X(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"y"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "y")]
    Y(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"z"`
    /// -   type: `hkReal`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "z")]
    Z(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"w"`
    /// -   type: `hkReal`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "w")]
    W(f64),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    BsDecomposeVectorModifierHkParam, "@name",
    ("vector" => Vector(cgmath::Vector4<f32>)),
    ("x" => X(f64)),
    ("y" => Y(f64)),
    ("z" => Z(f64)),
    ("w" => W(f64)),
}
