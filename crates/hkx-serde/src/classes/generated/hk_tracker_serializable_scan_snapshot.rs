//! A Rust structure that implements a serializer/deserializer corresponding to `hkTrackerSerializableScanSnapshot`, a class defined in C++
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
/// -    size: 92
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkTrackerSerializableScanSnapshot<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkTrackerSerializableScanSnapshot"`: Name of this class.
    #[serde(default = "HkTrackerSerializableScanSnapshot::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x875af1d9`: Unique value of this class.
    #[serde(default = "HkTrackerSerializableScanSnapshot::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkTrackerSerializableScanSnapshotHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkTrackerSerializableScanSnapshotHkParam<'a>>
}

impl HkTrackerSerializableScanSnapshot<'_> {
    /// Return `"hkTrackerSerializableScanSnapshot"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkTrackerSerializableScanSnapshot".into()
    }

    /// Return `"0x875af1d9"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x875af1d9".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkTrackerSerializableScanSnapshotHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"allocations"`
    /// -   type: `hkArray&lt;struct hkTrackerSerializableScanSnapshotAllocation&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "allocations")]
    Allocations(Vec<HkTrackerSerializableScanSnapshotAllocation>),
    /// # Information on fields in the original C++ class
    /// -   name:`"blocks"`
    /// -   type: `hkArray&lt;struct hkTrackerSerializableScanSnapshotBlock&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "blocks")]
    Blocks(Vec<HkTrackerSerializableScanSnapshotBlock>),
    /// # Information on fields in the original C++ class
    /// -   name:`"refs"`
    /// -   type: `hkArray&lt;hkInt32&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "refs")]
    Refs(Vec<i32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"typeNames"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "typeNames")]
    TypeNames(Vec<u8>),
    /// # Information on fields in the original C++ class
    /// -   name:`"traceText"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "traceText")]
    TraceText(Vec<u8>),
    /// # Information on fields in the original C++ class
    /// -   name:`"traceAddrs"`
    /// -   type: `hkArray&lt;hkUint64&gt;`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "traceAddrs")]
    TraceAddrs(Vec<u64>),
    /// # Information on fields in the original C++ class
    /// -   name:`"traceParents"`
    /// -   type: `hkArray&lt;hkInt32&gt;`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "traceParents")]
    TraceParents(Vec<i32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkTrackerSerializableScanSnapshotHkParam<'de>, "@name",
    ("allocations" => Allocations(Vec<HkTrackerSerializableScanSnapshotAllocation>)),
    ("blocks" => Blocks(Vec<HkTrackerSerializableScanSnapshotBlock>)),
    ("refs" => Refs(Vec<i32>)),
    ("typeNames" => TypeNames(Vec<u8>)),
    ("traceText" => TraceText(Vec<u8>)),
    ("traceAddrs" => TraceAddrs(Vec<u64>)),
    ("traceParents" => TraceParents(Vec<i32>)),
}
