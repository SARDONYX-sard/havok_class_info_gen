//! A Rust structure that implements a serializer/deserializer corresponding to `hkTrackerSerializableScanSnapshotAllocation`, a class defined in C++
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
/// -    size: 12
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkTrackerSerializableScanSnapshotAllocation<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkTrackerSerializableScanSnapshotAllocation"`: Name of this class.
    #[serde(default = "HkTrackerSerializableScanSnapshotAllocation::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x9ab3a6ac`: Unique value of this class.
    #[serde(default = "HkTrackerSerializableScanSnapshotAllocation::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkTrackerSerializableScanSnapshotAllocationHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkTrackerSerializableScanSnapshotAllocationHkParam<'a>>
}

impl HkTrackerSerializableScanSnapshotAllocation<'_> {
    /// Return `"hkTrackerSerializableScanSnapshotAllocation"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkTrackerSerializableScanSnapshotAllocation".into()
    }

    /// Return `"0x9ab3a6ac"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x9ab3a6ac".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkTrackerSerializableScanSnapshotAllocationHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"start"`
    /// -   type: `hkUlong`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "start")]
    Start(u64),
    /// # Information on fields in the original C++ class
    /// -   name:`"size"`
    /// -   type: `hkUlong`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "size")]
    Size(u64),
    /// # Information on fields in the original C++ class
    /// -   name:`"traceId"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "traceId")]
    TraceId(i32),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkTrackerSerializableScanSnapshotAllocationHkParam<'de>, "@name",
    ("start" => Start(u64)),
    ("size" => Size(u64)),
    ("traceId" => TraceId(i32)),
}
