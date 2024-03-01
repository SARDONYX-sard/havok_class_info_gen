//! A Rust structure that implements a serializer/deserializer corresponding to `hkVertexFormatElement`, a class defined in C++
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
/// -    size: 8
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkVertexFormatElement<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkVertexFormatElement"`: Name of this class.
    #[serde(default = "HkVertexFormatElement::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x54867cbf`: Unique value of this class.
    #[serde(default = "HkVertexFormatElement::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkVertexFormatElementHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkVertexFormatElementHkParam<'a>>
}

impl HkVertexFormatElement<'_> {
    /// Return `"hkVertexFormatElement"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkVertexFormatElement".into()
    }

    /// Return `"0x54867cbf"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x54867cbf".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkVertexFormatElementHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"dataType"`
    /// -   type: `enum ComponentType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "dataType")]
    DataType(ComponentType),
    /// # Information on fields in the original C++ class
    /// -   name:`"numValues"`
    /// -   type: `hkUint8`
    /// - offset: 1
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numValues")]
    NumValues(u8),
    /// # Information on fields in the original C++ class
    /// -   name:`"usage"`
    /// -   type: `enum ComponentUsage`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "usage")]
    Usage(ComponentUsage),
    /// # Information on fields in the original C++ class
    /// -   name:`"subUsage"`
    /// -   type: `hkUint8`
    /// - offset: 3
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "subUsage")]
    SubUsage(u8),
    /// # Information on fields in the original C++ class
    /// -   name:`"flags"`
    /// -   type: `flags HintFlags`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "flags")]
    Flags(HintFlags),
    /// # Information on fields in the original C++ class
    /// -   name:`"pad"`
    /// -   type: `hkUint8[3]`
    /// - offset: 5
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pad")]
    Pad([u8; 3]),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkVertexFormatElementHkParam<'de>, "@name",
    ("dataType" => DataType(ComponentType)),
    ("numValues" => NumValues(u8)),
    ("usage" => Usage(ComponentUsage)),
    ("subUsage" => SubUsage(u8)),
    ("flags" => Flags(HintFlags)),
    ("pad" => Pad([u8; 3])),
}
