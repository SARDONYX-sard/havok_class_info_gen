//! A Rust structure that implements a serializer/deserializer corresponding to `hkpExtendedMeshShapeSubpart`, a class defined in C++
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
/// -    size: 20
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 2
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpExtendedMeshShapeSubpart<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpExtendedMeshShapeSubpart"`: Name of this class.
    #[serde(default = "HkpExtendedMeshShapeSubpart::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xf4608207`: Unique value of this class.
    #[serde(default = "HkpExtendedMeshShapeSubpart::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpExtendedMeshShapeSubpartHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpExtendedMeshShapeSubpartHkParam<'a>>
}

impl HkpExtendedMeshShapeSubpart<'_> {
    /// Return `"hkpExtendedMeshShapeSubpart"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpExtendedMeshShapeSubpart".into()
    }

    /// Return `"0xf4608207"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xf4608207".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpExtendedMeshShapeSubpartHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"type"`
    /// -   type: `enum SubpartType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(SubpartType),
    /// # Information on fields in the original C++ class
    /// -   name:`"materialIndexStridingType"`
    /// -   type: `enum MaterialIndexStridingType`
    /// - offset: 1
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materialIndexStridingType")]
    MaterialIndexStridingType(MaterialIndexStridingType),
    /// # Information on fields in the original C++ class
    /// -   name:`"materialStriding"`
    /// -   type: `hkInt16`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "materialStriding", skip_serializing)]
    MaterialStriding(i16),
    /// # Information on fields in the original C++ class
    /// -   name:`"materialIndexBase"`
    /// -   type: `void*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "materialIndexBase", skip_serializing)]
    MaterialIndexBase(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"materialIndexStriding"`
    /// -   type: `hkUint16`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materialIndexStriding")]
    MaterialIndexStriding(u16),
    /// # Information on fields in the original C++ class
    /// -   name:`"numMaterials"`
    /// -   type: `hkUint16`
    /// - offset: 10
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numMaterials")]
    NumMaterials(u16),
    /// # Information on fields in the original C++ class
    /// -   name:`"materialBase"`
    /// -   type: `void*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "materialBase", skip_serializing)]
    MaterialBase(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData")]
    UserData(u64),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpExtendedMeshShapeSubpartHkParam<'de>, "@name",
    ("type" => Type(SubpartType)),
    ("materialIndexStridingType" => MaterialIndexStridingType(MaterialIndexStridingType)),
    ("materialStriding" => MaterialStriding(i16)),
    ("materialIndexBase" => MaterialIndexBase(())),
    ("materialIndexStriding" => MaterialIndexStriding(u16)),
    ("numMaterials" => NumMaterials(u16)),
    ("materialBase" => MaterialBase(())),
    ("userData" => UserData(u64)),
}
