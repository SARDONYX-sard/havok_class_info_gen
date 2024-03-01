//! A Rust structure that implements a serializer/deserializer corresponding to `BSIsActiveModifier`, a class defined in C++
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
/// -    size: 56
/// -  vtable: true
/// -  parent: hkbModifier/`96ec5ced`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct BsIsActiveModifier<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"BSIsActiveModifier"`: Name of this class.
    #[serde(default = "BsIsActiveModifier::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xb0fde45a`: Unique value of this class.
    #[serde(default = "BsIsActiveModifier::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<BsIsActiveModifierHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<BsIsActiveModifierHkParam<'a>>
}

impl BsIsActiveModifier<'_> {
    /// Return `"BSIsActiveModifier"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "BsIsActiveModifier".into()
    }

    /// Return `"0xb0fde45a"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xb0fde45a".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsIsActiveModifierHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"bIsActive0"`
    /// -   type: `hkBool`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bIsActive0")]
    BIsActive0(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"bInvertActive0"`
    /// -   type: `hkBool`
    /// - offset: 45
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bInvertActive0")]
    BInvertActive0(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"bIsActive1"`
    /// -   type: `hkBool`
    /// - offset: 46
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bIsActive1")]
    BIsActive1(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"bInvertActive1"`
    /// -   type: `hkBool`
    /// - offset: 47
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bInvertActive1")]
    BInvertActive1(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"bIsActive2"`
    /// -   type: `hkBool`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bIsActive2")]
    BIsActive2(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"bInvertActive2"`
    /// -   type: `hkBool`
    /// - offset: 49
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bInvertActive2")]
    BInvertActive2(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"bIsActive3"`
    /// -   type: `hkBool`
    /// - offset: 50
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bIsActive3")]
    BIsActive3(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"bInvertActive3"`
    /// -   type: `hkBool`
    /// - offset: 51
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bInvertActive3")]
    BInvertActive3(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"bIsActive4"`
    /// -   type: `hkBool`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bIsActive4")]
    BIsActive4(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"bInvertActive4"`
    /// -   type: `hkBool`
    /// - offset: 53
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bInvertActive4")]
    BInvertActive4(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    BsIsActiveModifierHkParam<'de>, "@name",
    ("bIsActive0" => BIsActive0(bool)),
    ("bInvertActive0" => BInvertActive0(bool)),
    ("bIsActive1" => BIsActive1(bool)),
    ("bInvertActive1" => BInvertActive1(bool)),
    ("bIsActive2" => BIsActive2(bool)),
    ("bInvertActive2" => BInvertActive2(bool)),
    ("bIsActive3" => BIsActive3(bool)),
    ("bInvertActive3" => BInvertActive3(bool)),
    ("bIsActive4" => BIsActive4(bool)),
    ("bInvertActive4" => BInvertActive4(bool)),
}
