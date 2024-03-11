//! A Rust structure that implements a serializer/deserializer corresponding to `hkbCharacterControllerModifierInternalState`, a class defined in C++
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
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbCharacterControllerModifierInternalState<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbCharacterControllerModifierInternalState"`: The original C++ class name.
    #[serde(default = "HkbCharacterControllerModifierInternalState::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xf8dfec0d`: Unique value of this class.
    #[serde(default = "HkbCharacterControllerModifierInternalState::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbCharacterControllerModifierInternalStateHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbCharacterControllerModifierInternalStateHkParam<'a>>
}

impl HkbCharacterControllerModifierInternalState<'_> {
    /// Return `"hkbCharacterControllerModifierInternalState"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbCharacterControllerModifierInternalState".into()
    }

    /// Return `"0xf8dfec0d"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xf8dfec0d".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbCharacterControllerModifierInternalStateHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"gravity"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "gravity")]
    Gravity(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"timestep"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "timestep")]
    Timestep(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"isInitialVelocityAdded"`
    /// -   type: `hkBool`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isInitialVelocityAdded")]
    IsInitialVelocityAdded(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"isTouchingGround"`
    /// -   type: `hkBool`
    /// - offset: 37
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isTouchingGround")]
    IsTouchingGround(Primitive<bool>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbCharacterControllerModifierInternalStateHkParam<'de>, "@name",
    ("gravity" => Gravity(Vector4<f32>)),
    ("timestep" => Timestep(Primitive<f32>)),
    ("isInitialVelocityAdded" => IsInitialVelocityAdded(Primitive<bool>)),
    ("isTouchingGround" => IsTouchingGround(Primitive<bool>)),
}
