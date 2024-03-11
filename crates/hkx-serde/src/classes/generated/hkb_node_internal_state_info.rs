//! A Rust structure that implements a serializer/deserializer corresponding to `hkbNodeInternalStateInfo`, a class defined in C++
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
/// -    size: 100
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbNodeInternalStateInfo<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbNodeInternalStateInfo"`: The original C++ class name.
    #[serde(default = "HkbNodeInternalStateInfo::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x7db9971d`: Unique value of this class.
    #[serde(default = "HkbNodeInternalStateInfo::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbNodeInternalStateInfoHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbNodeInternalStateInfoHkParam<'a>>
}

impl HkbNodeInternalStateInfo<'_> {
    /// Return `"hkbNodeInternalStateInfo"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbNodeInternalStateInfo".into()
    }

    /// Return `"0x7db9971d"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x7db9971d".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbNodeInternalStateInfoHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"syncInfo"`
    /// -   type: `struct hkbGeneratorSyncInfo`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "syncInfo")]
    SyncInfo(HkbGeneratorSyncInfo),
    /// # Field information in the original C++ class
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"internalState"`
    /// -   type: `struct hkReferencedObject*`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "internalState")]
    InternalState(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"nodeId"`
    /// -   type: `hkInt16`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nodeId")]
    NodeId(Primitive<i16>),
    /// # Field information in the original C++ class
    /// -   name:`"hasActivateBeenCalled"`
    /// -   type: `hkBool`
    /// - offset: 98
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "hasActivateBeenCalled")]
    HasActivateBeenCalled(Primitive<bool>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbNodeInternalStateInfoHkParam<'de>, "@name",
    ("syncInfo" => SyncInfo(HkbGeneratorSyncInfo)),
    ("name" => Name(Primitive<Cow<'a, str>>)),
    ("internalState" => InternalState(Cow<'a, str>)),
    ("nodeId" => NodeId(Primitive<i16>)),
    ("hasActivateBeenCalled" => HasActivateBeenCalled(Primitive<bool>)),
}
