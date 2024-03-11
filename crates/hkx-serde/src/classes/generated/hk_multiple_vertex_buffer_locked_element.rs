//! A Rust structure that implements a serializer/deserializer corresponding to `hkMultipleVertexBufferLockedElement`, a class defined in C++
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
/// -    size: 7
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkMultipleVertexBufferLockedElement<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkMultipleVertexBufferLockedElement"`: The original C++ class name.
    #[serde(default = "HkMultipleVertexBufferLockedElement::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xa0e22afc`: Unique value of this class.
    #[serde(default = "HkMultipleVertexBufferLockedElement::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkMultipleVertexBufferLockedElementHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkMultipleVertexBufferLockedElementHkParam<'a>>
}

impl HkMultipleVertexBufferLockedElement<'_> {
    /// Return `"hkMultipleVertexBufferLockedElement"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkMultipleVertexBufferLockedElement".into()
    }

    /// Return `"0xa0e22afc"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xa0e22afc".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMultipleVertexBufferLockedElementHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"vertexBufferIndex"`
    /// -   type: `hkUint8`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexBufferIndex")]
    VertexBufferIndex(Primitive<u8>),
    /// # Field information in the original C++ class
    /// -   name:`"elementIndex"`
    /// -   type: `hkUint8`
    /// - offset: 1
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "elementIndex")]
    ElementIndex(Primitive<u8>),
    /// # Field information in the original C++ class
    /// -   name:`"lockedBufferIndex"`
    /// -   type: `hkUint8`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lockedBufferIndex")]
    LockedBufferIndex(Primitive<u8>),
    /// # Field information in the original C++ class
    /// -   name:`"vertexFormatIndex"`
    /// -   type: `hkUint8`
    /// - offset: 3
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexFormatIndex")]
    VertexFormatIndex(Primitive<u8>),
    /// # Field information in the original C++ class
    /// -   name:`"lockFlags"`
    /// -   type: `hkUint8`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lockFlags")]
    LockFlags(Primitive<u8>),
    /// # Field information in the original C++ class
    /// -   name:`"outputBufferIndex"`
    /// -   type: `hkUint8`
    /// - offset: 5
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "outputBufferIndex")]
    OutputBufferIndex(Primitive<u8>),
    /// # Field information in the original C++ class
    /// -   name:`"emulatedIndex"`
    /// -   type: `hkInt8`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "emulatedIndex")]
    EmulatedIndex(Primitive<i8>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkMultipleVertexBufferLockedElementHkParam<'de>, "@name",
    ("vertexBufferIndex" => VertexBufferIndex(Primitive<u8>)),
    ("elementIndex" => ElementIndex(Primitive<u8>)),
    ("lockedBufferIndex" => LockedBufferIndex(Primitive<u8>)),
    ("vertexFormatIndex" => VertexFormatIndex(Primitive<u8>)),
    ("lockFlags" => LockFlags(Primitive<u8>)),
    ("outputBufferIndex" => OutputBufferIndex(Primitive<u8>)),
    ("emulatedIndex" => EmulatedIndex(Primitive<i8>)),
}
