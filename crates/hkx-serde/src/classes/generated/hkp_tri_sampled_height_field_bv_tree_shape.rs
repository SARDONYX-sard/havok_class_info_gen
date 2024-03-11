//! A Rust structure that implements a serializer/deserializer corresponding to `hkpTriSampledHeightFieldBvTreeShape`, a class defined in C++
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
/// -  parent: hkpBvTreeShape/`a823d623`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpTriSampledHeightFieldBvTreeShape<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpTriSampledHeightFieldBvTreeShape"`: The original C++ class name.
    #[serde(default = "HkpTriSampledHeightFieldBvTreeShape::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x58e1e585`: Unique value of this class.
    #[serde(default = "HkpTriSampledHeightFieldBvTreeShape::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpTriSampledHeightFieldBvTreeShapeHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpTriSampledHeightFieldBvTreeShapeHkParam<'a>>
}

impl HkpTriSampledHeightFieldBvTreeShape<'_> {
    /// Return `"hkpTriSampledHeightFieldBvTreeShape"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpTriSampledHeightFieldBvTreeShape".into()
    }

    /// Return `"0x58e1e585"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x58e1e585".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpTriSampledHeightFieldBvTreeShapeHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"childContainer"`
    /// -   type: `struct hkpSingleShapeContainer`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "childContainer")]
    ChildContainer(HkpSingleShapeContainer),
    /// # Field information in the original C++ class
    /// -   name:`"childSize"`
    /// -   type: `hkInt32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "childSize", skip_serializing)]
    ChildSize(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"wantAabbRejectionTest"`
    /// -   type: `hkBool`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wantAabbRejectionTest")]
    WantAabbRejectionTest(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"padding"`
    /// -   type: `hkUint8[12]`
    /// - offset: 33
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "padding")]
    Padding([Primitive<u8>; 12]),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpTriSampledHeightFieldBvTreeShapeHkParam<'de>, "@name",
    ("childContainer" => ChildContainer(HkpSingleShapeContainer)),
    ("childSize" => ChildSize(Primitive<i32>)),
    ("wantAabbRejectionTest" => WantAabbRejectionTest(Primitive<bool>)),
    ("padding" => Padding([Primitive<u8>; 12])),
}
