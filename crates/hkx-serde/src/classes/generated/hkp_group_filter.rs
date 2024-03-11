//! A Rust structure that implements a serializer/deserializer corresponding to `hkpGroupFilter`, a class defined in C++
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
/// -    size: 256
/// -  vtable: true
/// -  parent: hkpCollisionFilter/`60960336`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpGroupFilter<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpGroupFilter"`: The original C++ class name.
    #[serde(default = "HkpGroupFilter::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x65ee88e4`: Unique value of this class.
    #[serde(default = "HkpGroupFilter::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpGroupFilterHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpGroupFilterHkParam<'a>>
}

impl HkpGroupFilter<'_> {
    /// Return `"hkpGroupFilter"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpGroupFilter".into()
    }

    /// Return `"0x65ee88e4"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x65ee88e4".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpGroupFilterHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"nextFreeSystemGroup"`
    /// -   type: `hkInt32`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nextFreeSystemGroup")]
    NextFreeSystemGroup(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"collisionLookupTable"`
    /// -   type: `hkUint32[32]`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "collisionLookupTable")]
    CollisionLookupTable([Primitive<u32>; 32]),
    /// # Field information in the original C++ class
    /// -   name:`"pad256"`
    /// -   type: `hkVector4[4]`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pad256")]
    Pad256(Vector4<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpGroupFilterHkParam<'de>, "@name",
    ("nextFreeSystemGroup" => NextFreeSystemGroup(Primitive<i32>)),
    ("collisionLookupTable" => CollisionLookupTable([Primitive<u32>; 32])),
    ("pad256" => Pad256(Vector4<f32>)),
}
