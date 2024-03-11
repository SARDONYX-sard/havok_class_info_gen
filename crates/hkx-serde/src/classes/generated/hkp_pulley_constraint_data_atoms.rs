//! A Rust structure that implements a serializer/deserializer corresponding to `hkpPulleyConstraintDataAtoms`, a class defined in C++
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
/// -    size: 112
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpPulleyConstraintDataAtoms<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpPulleyConstraintDataAtoms"`: The original C++ class name.
    #[serde(default = "HkpPulleyConstraintDataAtoms::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xb149e5a`: Unique value of this class.
    #[serde(default = "HkpPulleyConstraintDataAtoms::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpPulleyConstraintDataAtomsHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpPulleyConstraintDataAtomsHkParam<'a>>
}

impl HkpPulleyConstraintDataAtoms<'_> {
    /// Return `"hkpPulleyConstraintDataAtoms"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpPulleyConstraintDataAtoms".into()
    }

    /// Return `"0xb149e5a"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xb149e5a".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpPulleyConstraintDataAtomsHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"translations"`
    /// -   type: `struct hkpSetLocalTranslationsConstraintAtom`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "translations")]
    Translations(HkpSetLocalTranslationsConstraintAtom),
    /// # Field information in the original C++ class
    /// -   name:`"pulley"`
    /// -   type: `struct hkpPulleyConstraintAtom`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pulley")]
    Pulley(HkpPulleyConstraintAtom),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpPulleyConstraintDataAtomsHkParam<'de>, "@name",
    ("translations" => Translations(HkpSetLocalTranslationsConstraintAtom)),
    ("pulley" => Pulley(HkpPulleyConstraintAtom)),
}
