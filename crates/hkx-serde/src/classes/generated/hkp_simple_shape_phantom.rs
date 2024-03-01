//! A Rust structure that implements a serializer/deserializer corresponding to `hkpSimpleShapePhantom`, a class defined in C++
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
/// -    size: 368
/// -  vtable: true
/// -  parent: hkpShapePhantom/`cb22fbcd`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpSimpleShapePhantom<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpSimpleShapePhantom"`: Name of this class.
    #[serde(default = "HkpSimpleShapePhantom::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x32a2a8a8`: Unique value of this class.
    #[serde(default = "HkpSimpleShapePhantom::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpSimpleShapePhantomHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpSimpleShapePhantomHkParam<'a>>
}

impl HkpSimpleShapePhantom<'_> {
    /// Return `"hkpSimpleShapePhantom"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpSimpleShapePhantom".into()
    }

    /// Return `"0x32a2a8a8"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x32a2a8a8".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpSimpleShapePhantomHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"collisionDetails"`
    /// -   type: `hkArray&lt;struct hkpSimpleShapePhantomCollisionDetail&gt;`
    /// - offset: 352
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "collisionDetails", skip_serializing)]
    CollisionDetails(Vec<HkpSimpleShapePhantomCollisionDetail>),
    /// # Information on fields in the original C++ class
    /// -   name:`"orderDirty"`
    /// -   type: `hkBool`
    /// - offset: 364
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "orderDirty", skip_serializing)]
    OrderDirty(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpSimpleShapePhantomHkParam<'de>, "@name",
    ("collisionDetails" => CollisionDetails(Vec<HkpSimpleShapePhantomCollisionDetail>)),
    ("orderDirty" => OrderDirty(bool)),
}
