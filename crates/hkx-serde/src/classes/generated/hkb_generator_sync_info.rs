//! A Rust structure that implements a serializer/deserializer corresponding to `hkbGeneratorSyncInfo`, a class defined in C++
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
/// -    size: 80
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbGeneratorSyncInfo<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbGeneratorSyncInfo"`: Name of this class.
    #[serde(default = "HkbGeneratorSyncInfo::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xa3c341f8`: Unique value of this class.
    #[serde(default = "HkbGeneratorSyncInfo::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbGeneratorSyncInfoHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbGeneratorSyncInfoHkParam<'a>>
}

impl HkbGeneratorSyncInfo<'_> {
    /// Return `"hkbGeneratorSyncInfo"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbGeneratorSyncInfo".into()
    }

    /// Return `"0xa3c341f8"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xa3c341f8".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbGeneratorSyncInfoHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"syncPoints"`
    /// -   type: `struct hkbGeneratorSyncInfoSyncPoint[8]`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "syncPoints")]
    SyncPoints(HkbGeneratorSyncInfoSyncPoint[8]),
    /// # Information on fields in the original C++ class
    /// -   name:`"baseFrequency"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "baseFrequency")]
    BaseFrequency(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"localTime"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "localTime")]
    LocalTime(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"playbackSpeed"`
    /// -   type: `hkReal`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "playbackSpeed")]
    PlaybackSpeed(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"numSyncPoints"`
    /// -   type: `hkInt8`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numSyncPoints")]
    NumSyncPoints(i8),
    /// # Information on fields in the original C++ class
    /// -   name:`"isCyclic"`
    /// -   type: `hkBool`
    /// - offset: 77
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isCyclic")]
    IsCyclic(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"isMirrored"`
    /// -   type: `hkBool`
    /// - offset: 78
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isMirrored")]
    IsMirrored(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"isAdditive"`
    /// -   type: `hkBool`
    /// - offset: 79
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isAdditive")]
    IsAdditive(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbGeneratorSyncInfoHkParam<'de>, "@name",
    ("syncPoints" => SyncPoints(HkbGeneratorSyncInfoSyncPoint[8])),
    ("baseFrequency" => BaseFrequency(f64)),
    ("localTime" => LocalTime(f64)),
    ("playbackSpeed" => PlaybackSpeed(f64)),
    ("numSyncPoints" => NumSyncPoints(i8)),
    ("isCyclic" => IsCyclic(bool)),
    ("isMirrored" => IsMirrored(bool)),
    ("isAdditive" => IsAdditive(bool)),
}