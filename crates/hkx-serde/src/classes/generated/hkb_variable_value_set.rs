//! A Rust structure that implements a serializer/deserializer corresponding to `hkbVariableValueSet`, a class defined in C++
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
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbVariableValueSet<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbVariableValueSet"`: The original C++ class name.
    #[serde(default = "HkbVariableValueSet::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x27812d8d`: Unique value of this class.
    #[serde(default = "HkbVariableValueSet::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbVariableValueSetHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbVariableValueSetHkParam<'a>>
}

impl HkbVariableValueSet<'_> {
    /// Return `"hkbVariableValueSet"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbVariableValueSet".into()
    }

    /// Return `"0x27812d8d"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x27812d8d".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbVariableValueSetHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"wordVariableValues"`
    /// -   type: `hkArray&lt;struct hkbVariableValue&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wordVariableValues")]
    WordVariableValues(Vec<HkbVariableValue>),
    /// # Field information in the original C++ class
    /// -   name:`"quadVariableValues"`
    /// -   type: `hkArray&lt;hkVector4&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "quadVariableValues")]
    QuadVariableValues(Vec<Vector4<f32>>),
    /// # Field information in the original C++ class
    /// -   name:`"variantVariableValues"`
    /// -   type: `hkArray&lt;hkReferencedObject*&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "variantVariableValues")]
    VariantVariableValues(Vec<Cow<'a, str>>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbVariableValueSetHkParam<'de>, "@name",
    ("wordVariableValues" => WordVariableValues(Vec<HkbVariableValue>)),
    ("quadVariableValues" => QuadVariableValues(Vec<Vector4<f32>>)),
    ("variantVariableValues" => VariantVariableValues(Vec<Cow<'a, str>>)),
}
