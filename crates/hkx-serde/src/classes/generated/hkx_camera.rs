//! A Rust structure that implements a serializer/deserializer corresponding to `hkxCamera`, a class defined in C++
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
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkxCamera<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkxCamera"`: Name of this class.
    #[serde(default = "HkxCamera::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xe3597b02`: Unique value of this class.
    #[serde(default = "HkxCamera::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkxCameraHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkxCameraHkParam<'a>>
}

impl HkxCamera<'_> {
    /// Return `"hkxCamera"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkxCamera".into()
    }

    /// Return `"0xe3597b02"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xe3597b02".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxCameraHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"from"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "from")]
    From(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"focus"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "focus")]
    Focus(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"up"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "up")]
    Up(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"fov"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fov")]
    Fov(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"far"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "far")]
    Far(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"near"`
    /// -   type: `hkReal`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "near")]
    Near(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"leftHanded"`
    /// -   type: `hkBool`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "leftHanded")]
    LeftHanded(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkxCameraHkParam<'de>, "@name",
    ("from" => From(cgmath::Vector4<f32>)),
    ("focus" => Focus(cgmath::Vector4<f32>)),
    ("up" => Up(cgmath::Vector4<f32>)),
    ("fov" => Fov(f64)),
    ("far" => Far(f64)),
    ("near" => Near(f64)),
    ("leftHanded" => LeftHanded(bool)),
}