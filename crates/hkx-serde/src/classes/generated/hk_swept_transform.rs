//! A Rust structure that implements a serializer/deserializer corresponding to `hkSweptTransform`, a class defined in C++
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
/// -    size: 80
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkSweptTransform<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkSweptTransform"`: The original C++ class name.
    #[serde(default = "HkSweptTransform::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xb4e5770`: Unique value of this class.
    #[serde(default = "HkSweptTransform::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkSweptTransformHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkSweptTransformHkParam<'a>>
}

impl HkSweptTransform<'_> {
    /// Return `"hkSweptTransform"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkSweptTransform".into()
    }

    /// Return `"0xb4e5770"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xb4e5770".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkSweptTransformHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"centerOfMass0"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "centerOfMass0")]
    CenterOfMass0(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"centerOfMass1"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "centerOfMass1")]
    CenterOfMass1(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"rotation0"`
    /// -   type: `hkQuaternion`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotation0")]
    Rotation0(Quaternion<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"rotation1"`
    /// -   type: `hkQuaternion`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotation1")]
    Rotation1(Quaternion<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"centerOfMassLocal"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "centerOfMassLocal")]
    CenterOfMassLocal(Vector4<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkSweptTransformHkParam<'de>, "@name",
    ("centerOfMass0" => CenterOfMass0(Vector4<f32>)),
    ("centerOfMass1" => CenterOfMass1(Vector4<f32>)),
    ("rotation0" => Rotation0(Quaternion<f32>)),
    ("rotation1" => Rotation1(Quaternion<f32>)),
    ("centerOfMassLocal" => CenterOfMassLocal(Vector4<f32>)),
}
