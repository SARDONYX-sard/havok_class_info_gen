//! A Rust structure that implements a serializer/deserializer corresponding to `hkpTypedBroadPhaseHandle`, a class defined in C++
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
/// -  parent: hkpBroadPhaseHandle/`940569dc`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpTypedBroadPhaseHandle<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpTypedBroadPhaseHandle"`: The original C++ class name.
    #[serde(default = "HkpTypedBroadPhaseHandle::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xf4b0f799`: Unique value of this class.
    #[serde(default = "HkpTypedBroadPhaseHandle::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpTypedBroadPhaseHandleHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpTypedBroadPhaseHandleHkParam<'a>>
}

impl HkpTypedBroadPhaseHandle<'_> {
    /// Return `"hkpTypedBroadPhaseHandle"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpTypedBroadPhaseHandle".into()
    }

    /// Return `"0xf4b0f799"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xf4b0f799".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpTypedBroadPhaseHandleHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"type"`
    /// -   type: `hkInt8`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(Primitive<i8>),
    /// # Field information in the original C++ class
    /// -   name:`"ownerOffset"`
    /// -   type: `hkInt8`
    /// - offset: 5
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "ownerOffset", skip_serializing)]
    OwnerOffset(Primitive<i8>),
    /// # Field information in the original C++ class
    /// -   name:`"objectQualityType"`
    /// -   type: `hkInt8`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "objectQualityType")]
    ObjectQualityType(Primitive<i8>),
    /// # Field information in the original C++ class
    /// -   name:`"collisionFilterInfo"`
    /// -   type: `hkUint32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "collisionFilterInfo")]
    CollisionFilterInfo(Primitive<u32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpTypedBroadPhaseHandleHkParam<'de>, "@name",
    ("type" => Type(Primitive<i8>)),
    ("ownerOffset" => OwnerOffset(Primitive<i8>)),
    ("objectQualityType" => ObjectQualityType(Primitive<i8>)),
    ("collisionFilterInfo" => CollisionFilterInfo(Primitive<u32>)),
}
