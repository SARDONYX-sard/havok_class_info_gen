//! A Rust structure that implements a serializer/deserializer corresponding to `hkpGenericConstraintDataSchemeConstraintInfo`, a class defined in C++
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
/// -    size: 16
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpGenericConstraintDataSchemeConstraintInfo<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpGenericConstraintDataSchemeConstraintInfo"`: Name of this class.
    #[serde(default = "HkpGenericConstraintDataSchemeConstraintInfo::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xd6421f19`: Unique value of this class.
    #[serde(default = "HkpGenericConstraintDataSchemeConstraintInfo::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpGenericConstraintDataSchemeConstraintInfoHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpGenericConstraintDataSchemeConstraintInfoHkParam<'a>>
}

impl HkpGenericConstraintDataSchemeConstraintInfo<'_> {
    /// Return `"hkpGenericConstraintDataSchemeConstraintInfo"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpGenericConstraintDataSchemeConstraintInfo".into()
    }

    /// Return `"0xd6421f19"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xd6421f19".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpGenericConstraintDataSchemeConstraintInfoHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"maxSizeOfSchema"`
    /// -   type: `hkInt32`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxSizeOfSchema")]
    MaxSizeOfSchema(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"sizeOfSchemas"`
    /// -   type: `hkInt32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sizeOfSchemas")]
    SizeOfSchemas(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"numSolverResults"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numSolverResults")]
    NumSolverResults(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"numSolverElemTemps"`
    /// -   type: `hkInt32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numSolverElemTemps")]
    NumSolverElemTemps(i32),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpGenericConstraintDataSchemeConstraintInfoHkParam<'de>, "@name",
    ("maxSizeOfSchema" => MaxSizeOfSchema(i32)),
    ("sizeOfSchemas" => SizeOfSchemas(i32)),
    ("numSolverResults" => NumSolverResults(i32)),
    ("numSolverElemTemps" => NumSolverElemTemps(i32)),
}
