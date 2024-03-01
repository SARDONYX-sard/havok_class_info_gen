//! A Rust structure that implements a serializer/deserializer corresponding to `hkpSimpleContactConstraintAtom`, a class defined in C++
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
/// -    size: 48
/// -  vtable: false
/// -  parent: hkpConstraintAtom/`59d67ef6`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpSimpleContactConstraintAtom<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpSimpleContactConstraintAtom"`: Name of this class.
    #[serde(default = "HkpSimpleContactConstraintAtom::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x920df11a`: Unique value of this class.
    #[serde(default = "HkpSimpleContactConstraintAtom::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpSimpleContactConstraintAtomHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpSimpleContactConstraintAtomHkParam<'a>>
}

impl HkpSimpleContactConstraintAtom<'_> {
    /// Return `"hkpSimpleContactConstraintAtom"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpSimpleContactConstraintAtom".into()
    }

    /// Return `"0x920df11a"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x920df11a".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpSimpleContactConstraintAtomHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"sizeOfAllAtoms"`
    /// -   type: `hkUint16`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sizeOfAllAtoms")]
    SizeOfAllAtoms(u16),
    /// # Information on fields in the original C++ class
    /// -   name:`"numContactPoints"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numContactPoints")]
    NumContactPoints(u16),
    /// # Information on fields in the original C++ class
    /// -   name:`"numReservedContactPoints"`
    /// -   type: `hkUint16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numReservedContactPoints")]
    NumReservedContactPoints(u16),
    /// # Information on fields in the original C++ class
    /// -   name:`"numUserDatasForBodyA"`
    /// -   type: `hkUint8`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numUserDatasForBodyA")]
    NumUserDatasForBodyA(u8),
    /// # Information on fields in the original C++ class
    /// -   name:`"numUserDatasForBodyB"`
    /// -   type: `hkUint8`
    /// - offset: 9
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numUserDatasForBodyB")]
    NumUserDatasForBodyB(u8),
    /// # Information on fields in the original C++ class
    /// -   name:`"contactPointPropertiesStriding"`
    /// -   type: `hkUint8`
    /// - offset: 10
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "contactPointPropertiesStriding")]
    ContactPointPropertiesStriding(u8),
    /// # Information on fields in the original C++ class
    /// -   name:`"maxNumContactPoints"`
    /// -   type: `hkUint16`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxNumContactPoints")]
    MaxNumContactPoints(u16),
    /// # Information on fields in the original C++ class
    /// -   name:`"info"`
    /// -   type: `struct hkpSimpleContactConstraintDataInfo`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE | ALIGN16`
    #[serde(rename = "info")]
    Info(HkpSimpleContactConstraintDataInfo),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpSimpleContactConstraintAtomHkParam<'de>, "@name",
    ("sizeOfAllAtoms" => SizeOfAllAtoms(u16)),
    ("numContactPoints" => NumContactPoints(u16)),
    ("numReservedContactPoints" => NumReservedContactPoints(u16)),
    ("numUserDatasForBodyA" => NumUserDatasForBodyA(u8)),
    ("numUserDatasForBodyB" => NumUserDatasForBodyB(u8)),
    ("contactPointPropertiesStriding" => ContactPointPropertiesStriding(u8)),
    ("maxNumContactPoints" => MaxNumContactPoints(u16)),
    ("info" => Info(HkpSimpleContactConstraintDataInfo)),
}
