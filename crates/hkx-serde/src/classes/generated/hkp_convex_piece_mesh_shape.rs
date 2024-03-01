//! A Rust structure that implements a serializer/deserializer corresponding to `hkpConvexPieceMeshShape`, a class defined in C++
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
/// -    size: 36
/// -  vtable: true
/// -  parent: hkpShapeCollection/`e8c3991d`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpConvexPieceMeshShape<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpConvexPieceMeshShape"`: Name of this class.
    #[serde(default = "HkpConvexPieceMeshShape::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x38fd3d97`: Unique value of this class.
    #[serde(default = "HkpConvexPieceMeshShape::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpConvexPieceMeshShapeHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpConvexPieceMeshShapeHkParam<'a>>
}

impl HkpConvexPieceMeshShape<'_> {
    /// Return `"hkpConvexPieceMeshShape"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpConvexPieceMeshShape".into()
    }

    /// Return `"0x38fd3d97"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x38fd3d97".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpConvexPieceMeshShapeHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"convexPieceStream"`
    /// -   type: `struct hkpConvexPieceStreamData*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "convexPieceStream")]
    ConvexPieceStream(Box<HkpConvexPieceStreamData>),
    /// # Information on fields in the original C++ class
    /// -   name:`"displayMesh"`
    /// -   type: `struct hkpShapeCollection*`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "displayMesh")]
    DisplayMesh(Box<HkpShapeCollection>),
    /// # Information on fields in the original C++ class
    /// -   name:`"radius"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "radius")]
    Radius(f64),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpConvexPieceMeshShapeHkParam<'de>, "@name",
    ("convexPieceStream" => ConvexPieceStream(Box<HkpConvexPieceStreamData>)),
    ("displayMesh" => DisplayMesh(Box<HkpShapeCollection>)),
    ("radius" => Radius(f64)),
}
