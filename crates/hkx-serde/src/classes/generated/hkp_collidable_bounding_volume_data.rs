//! A Rust structure that implements a serializer/deserializer corresponding to `hkpCollidableBoundingVolumeData`, a class defined in C++
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
/// -    size: 44
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpCollidableBoundingVolumeData<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpCollidableBoundingVolumeData"`: The original C++ class name.
    #[serde(default = "HkpCollidableBoundingVolumeData::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xb5f0e6b1`: Unique value of this class.
    #[serde(default = "HkpCollidableBoundingVolumeData::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpCollidableBoundingVolumeDataHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpCollidableBoundingVolumeDataHkParam<'a>>
}

impl HkpCollidableBoundingVolumeData<'_> {
    /// Return `"hkpCollidableBoundingVolumeData"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpCollidableBoundingVolumeData".into()
    }

    /// Return `"0xb5f0e6b1"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xb5f0e6b1".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpCollidableBoundingVolumeDataHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"min"`
    /// -   type: `hkUint32[3]`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "min")]
    Min([Primitive<u32>; 3]),
    /// # Field information in the original C++ class
    /// -   name:`"expansionMin"`
    /// -   type: `hkUint8[3]`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "expansionMin")]
    ExpansionMin([Primitive<u8>; 3]),
    /// # Field information in the original C++ class
    /// -   name:`"expansionShift"`
    /// -   type: `hkUint8`
    /// - offset: 15
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "expansionShift")]
    ExpansionShift(Primitive<u8>),
    /// # Field information in the original C++ class
    /// -   name:`"max"`
    /// -   type: `hkUint32[3]`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "max")]
    Max([Primitive<u32>; 3]),
    /// # Field information in the original C++ class
    /// -   name:`"expansionMax"`
    /// -   type: `hkUint8[3]`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "expansionMax")]
    ExpansionMax([Primitive<u8>; 3]),
    /// # Field information in the original C++ class
    /// -   name:`"padding"`
    /// -   type: `hkUint8`
    /// - offset: 31
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "padding")]
    Padding(Primitive<u8>),
    /// # Field information in the original C++ class
    /// -   name:`"numChildShapeAabbs"`
    /// -   type: `hkUint16`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "numChildShapeAabbs", skip_serializing)]
    NumChildShapeAabbs(Primitive<u16>),
    /// # Field information in the original C++ class
    /// -   name:`"capacityChildShapeAabbs"`
    /// -   type: `hkUint16`
    /// - offset: 34
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "capacityChildShapeAabbs", skip_serializing)]
    CapacityChildShapeAabbs(Primitive<u16>),
    /// # Field information in the original C++ class
    /// -   name:`"childShapeAabbs"`
    /// -   type: `void*`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "childShapeAabbs", skip_serializing)]
    ChildShapeAabbs(()),
    /// # Field information in the original C++ class
    /// -   name:`"childShapeKeys"`
    /// -   type: `void*`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "childShapeKeys", skip_serializing)]
    ChildShapeKeys(()),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpCollidableBoundingVolumeDataHkParam<'de>, "@name",
    ("min" => Min([Primitive<u32>; 3])),
    ("expansionMin" => ExpansionMin([Primitive<u8>; 3])),
    ("expansionShift" => ExpansionShift(Primitive<u8>)),
    ("max" => Max([Primitive<u32>; 3])),
    ("expansionMax" => ExpansionMax([Primitive<u8>; 3])),
    ("padding" => Padding(Primitive<u8>)),
    ("numChildShapeAabbs" => NumChildShapeAabbs(Primitive<u16>)),
    ("capacityChildShapeAabbs" => CapacityChildShapeAabbs(Primitive<u16>)),
    ("childShapeAabbs" => ChildShapeAabbs(())),
    ("childShapeKeys" => ChildShapeKeys(())),
}
