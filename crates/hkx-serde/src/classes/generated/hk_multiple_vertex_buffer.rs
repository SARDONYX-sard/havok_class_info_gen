//! A Rust structure that implements a serializer/deserializer corresponding to `hkMultipleVertexBuffer`, a class defined in C++
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
/// -    size: 324
/// -  vtable: true
/// -  parent: hkMeshVertexBuffer/`534b08c8`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkMultipleVertexBuffer<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkMultipleVertexBuffer"`: Name of this class.
    #[serde(default = "HkMultipleVertexBuffer::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xde3ab602`: Unique value of this class.
    #[serde(default = "HkMultipleVertexBuffer::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkMultipleVertexBufferHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkMultipleVertexBufferHkParam<'a>>
}

impl HkMultipleVertexBuffer<'_> {
    /// Return `"hkMultipleVertexBuffer"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkMultipleVertexBuffer".into()
    }

    /// Return `"0xde3ab602"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xde3ab602".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMultipleVertexBufferHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"vertexFormat"`
    /// -   type: `struct hkVertexFormat`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexFormat")]
    VertexFormat(HkVertexFormat),
    /// # Information on fields in the original C++ class
    /// -   name:`"lockedElements"`
    /// -   type: `hkArray&lt;struct hkMultipleVertexBufferLockedElement&gt;`
    /// - offset: 268
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lockedElements")]
    LockedElements(Vec<HkMultipleVertexBufferLockedElement>),
    /// # Information on fields in the original C++ class
    /// -   name:`"lockedBuffer"`
    /// -   type: `struct hkMemoryMeshVertexBuffer*`
    /// - offset: 280
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lockedBuffer")]
    LockedBuffer(Box<HkMemoryMeshVertexBuffer>),
    /// # Information on fields in the original C++ class
    /// -   name:`"elementInfos"`
    /// -   type: `hkArray&lt;struct hkMultipleVertexBufferElementInfo&gt;`
    /// - offset: 284
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "elementInfos")]
    ElementInfos(Vec<HkMultipleVertexBufferElementInfo>),
    /// # Information on fields in the original C++ class
    /// -   name:`"vertexBufferInfos"`
    /// -   type: `hkArray&lt;struct hkMultipleVertexBufferVertexBufferInfo&gt;`
    /// - offset: 296
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexBufferInfos")]
    VertexBufferInfos(Vec<HkMultipleVertexBufferVertexBufferInfo>),
    /// # Information on fields in the original C++ class
    /// -   name:`"numVertices"`
    /// -   type: `hkInt32`
    /// - offset: 308
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numVertices")]
    NumVertices(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"isLocked"`
    /// -   type: `hkBool`
    /// - offset: 312
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isLocked")]
    IsLocked(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"updateCount"`
    /// -   type: `hkUint32`
    /// - offset: 316
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "updateCount")]
    UpdateCount(u32),
    /// # Information on fields in the original C++ class
    /// -   name:`"writeLock"`
    /// -   type: `hkBool`
    /// - offset: 320
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "writeLock")]
    WriteLock(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"isSharable"`
    /// -   type: `hkBool`
    /// - offset: 321
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isSharable")]
    IsSharable(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"constructionComplete"`
    /// -   type: `hkBool`
    /// - offset: 322
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "constructionComplete")]
    ConstructionComplete(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkMultipleVertexBufferHkParam<'de>, "@name",
    ("vertexFormat" => VertexFormat(HkVertexFormat)),
    ("lockedElements" => LockedElements(Vec<HkMultipleVertexBufferLockedElement>)),
    ("lockedBuffer" => LockedBuffer(Box<HkMemoryMeshVertexBuffer>)),
    ("elementInfos" => ElementInfos(Vec<HkMultipleVertexBufferElementInfo>)),
    ("vertexBufferInfos" => VertexBufferInfos(Vec<HkMultipleVertexBufferVertexBufferInfo>)),
    ("numVertices" => NumVertices(i32)),
    ("isLocked" => IsLocked(bool)),
    ("updateCount" => UpdateCount(u32)),
    ("writeLock" => WriteLock(bool)),
    ("isSharable" => IsSharable(bool)),
    ("constructionComplete" => ConstructionComplete(bool)),
}
