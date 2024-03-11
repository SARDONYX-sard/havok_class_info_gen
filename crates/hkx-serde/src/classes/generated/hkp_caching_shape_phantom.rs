//! A Rust structure that implements a serializer/deserializer corresponding to `hkpCachingShapePhantom`, a class defined in C++
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
/// -    size: 368
/// -  vtable: true
/// -  parent: hkpShapePhantom/`cb22fbcd`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpCachingShapePhantom<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpCachingShapePhantom"`: The original C++ class name.
    #[serde(default = "HkpCachingShapePhantom::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xcf227f58`: Unique value of this class.
    #[serde(default = "HkpCachingShapePhantom::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpCachingShapePhantomHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpCachingShapePhantomHkParam<'a>>
}

impl HkpCachingShapePhantom<'_> {
    /// Return `"hkpCachingShapePhantom"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpCachingShapePhantom".into()
    }

    /// Return `"0xcf227f58"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xcf227f58".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpCachingShapePhantomHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"collisionDetails"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 352
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "collisionDetails", skip_serializing)]
    CollisionDetails(Vec<()>),
    /// # Field information in the original C++ class
    /// -   name:`"orderDirty"`
    /// -   type: `hkBool`
    /// - offset: 364
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "orderDirty", skip_serializing)]
    OrderDirty(Primitive<bool>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpCachingShapePhantomHkParam<'de>, "@name",
    ("collisionDetails" => CollisionDetails(Vec<()>)),
    ("orderDirty" => OrderDirty(Primitive<bool>)),
}
