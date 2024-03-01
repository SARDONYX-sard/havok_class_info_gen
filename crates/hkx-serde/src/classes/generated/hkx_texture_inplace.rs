//! A Rust structure that implements a serializer/deserializer corresponding to `hkxTextureInplace`, a class defined in C++
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
/// -    size: 32
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkxTextureInplace<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkxTextureInplace"`: Name of this class.
    #[serde(default = "HkxTextureInplace::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xd45841d6`: Unique value of this class.
    #[serde(default = "HkxTextureInplace::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkxTextureInplaceHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkxTextureInplaceHkParam<'a>>
}

impl HkxTextureInplace<'_> {
    /// Return `"hkxTextureInplace"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkxTextureInplace".into()
    }

    /// Return `"0xd45841d6"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xd45841d6".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxTextureInplaceHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"fileType"`
    /// -   type: `hkChar[4]`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fileType")]
    FileType([char; 4]),
    /// # Information on fields in the original C++ class
    /// -   name:`"data"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "data")]
    Data(Vec<u8>),
    /// # Information on fields in the original C++ class
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(String),
    /// # Information on fields in the original C++ class
    /// -   name:`"originalFilename"`
    /// -   type: `hkStringPtr`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "originalFilename")]
    OriginalFilename(String),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkxTextureInplaceHkParam<'de>, "@name",
    ("fileType" => FileType([char; 4])),
    ("data" => Data(Vec<u8>)),
    ("name" => Name(String)),
    ("originalFilename" => OriginalFilename(String)),
}
