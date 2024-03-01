//! A Rust structure that implements a serializer/deserializer corresponding to `hkTrackerSerializableScanSnapshotBlock`, a class defined in C++
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
/// -    size: 24
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkTrackerSerializableScanSnapshotBlock<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkTrackerSerializableScanSnapshotBlock"`: Name of this class.
    #[serde(default = "HkTrackerSerializableScanSnapshotBlock::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xe7f23e6d`: Unique value of this class.
    #[serde(default = "HkTrackerSerializableScanSnapshotBlock::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkTrackerSerializableScanSnapshotBlockHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkTrackerSerializableScanSnapshotBlockHkParam<'a>>
}

impl HkTrackerSerializableScanSnapshotBlock<'_> {
    /// Return `"hkTrackerSerializableScanSnapshotBlock"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkTrackerSerializableScanSnapshotBlock".into()
    }

    /// Return `"0xe7f23e6d"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xe7f23e6d".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkTrackerSerializableScanSnapshotBlockHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"typeIndex"`
    /// -   type: `hkInt32`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "typeIndex")]
    TypeIndex(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"start"`
    /// -   type: `hkUlong`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "start")]
    Start(u64),
    /// # Information on fields in the original C++ class
    /// -   name:`"size"`
    /// -   type: `hkUlong`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "size")]
    Size(u64),
    /// # Information on fields in the original C++ class
    /// -   name:`"arraySize"`
    /// -   type: `hkInt32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "arraySize")]
    ArraySize(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"startReferenceIndex"`
    /// -   type: `hkInt32`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "startReferenceIndex")]
    StartReferenceIndex(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"numReferences"`
    /// -   type: `hkInt32`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numReferences")]
    NumReferences(i32),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkTrackerSerializableScanSnapshotBlockHkParam<'de>, "@name",
    ("typeIndex" => TypeIndex(i32)),
    ("start" => Start(u64)),
    ("size" => Size(u64)),
    ("arraySize" => ArraySize(i32)),
    ("startReferenceIndex" => StartReferenceIndex(i32)),
    ("numReferences" => NumReferences(i32)),
}
