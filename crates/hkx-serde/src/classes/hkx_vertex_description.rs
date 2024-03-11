//! A Rust structure that implements a serializer/deserializer corresponding to `hkxVertexDescription`, a class defined in C++
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::hkx_vertex_description_element_decl::HkxVertexDescriptionElementDeclHkParam;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// In XML, it is enclosed in a `hkobject` tag
/// and the `class` attribute contains the C++ class nam
///
/// # Information on the original C++ class
/// -    size: 12
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkxVertexDescription<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkxVertexDescription"`: The original C++ class name.
    #[serde(default = "HkxVertexDescription::class_name")]
    #[serde(rename = "@class", borrow, skip_deserializing)]
    pub class: Cow<'a, str>,

    /// `0x2df6313d`: Unique value of this class.
    #[serde(default = "HkxVertexDescription::signature")]
    #[serde(rename = "@signature", borrow, skip_deserializing)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkxVertexDescriptionHkParam>,
}

impl HkxVertexDescription<'_> {
    /// Return `"hkxVertexDescription"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkxVertexDescription".into()
    }

    /// Return `"0x2df6313d"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x2df6313d".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxVertexDescriptionHkParam {
    /// # Field information in the original C++ class
    /// -   name:`"decls"`
    /// -   type: `hkArray&lt;struct hkxVertexDescriptionElementDecl&gt;`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "decls")]
    Decls(HkArrayClass<HkxVertexDescriptionElementDeclHkParam>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkxVertexDescriptionHkParam, "@name",
    ("decls" => Decls(HkArrayClass<HkxVertexDescriptionElementDeclHkParam>)),
}
