//! A Rust structure that implements a serializer/deserializer corresponding to `hkpSerializedAgentNnEntry`, a class defined in C++
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
/// -    size: 320
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpSerializedAgentNnEntry<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpSerializedAgentNnEntry"`: The original C++ class name.
    #[serde(default = "HkpSerializedAgentNnEntry::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x49ec7de3`: Unique value of this class.
    #[serde(default = "HkpSerializedAgentNnEntry::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpSerializedAgentNnEntryHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpSerializedAgentNnEntryHkParam<'a>>
}

impl HkpSerializedAgentNnEntry<'_> {
    /// Return `"hkpSerializedAgentNnEntry"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpSerializedAgentNnEntry".into()
    }

    /// Return `"0x49ec7de3"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x49ec7de3".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpSerializedAgentNnEntryHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"bodyA"`
    /// -   type: `struct hkpEntity*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bodyA")]
    BodyA(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"bodyB"`
    /// -   type: `struct hkpEntity*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bodyB")]
    BodyB(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"bodyAId"`
    /// -   type: `hkUlong`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bodyAId")]
    BodyAId(Primitive<u64>),
    /// # Field information in the original C++ class
    /// -   name:`"bodyBId"`
    /// -   type: `hkUlong`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bodyBId")]
    BodyBId(Primitive<u64>),
    /// # Field information in the original C++ class
    /// -   name:`"useEntityIds"`
    /// -   type: `hkBool`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "useEntityIds")]
    UseEntityIds(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"agentType"`
    /// -   type: `enum SerializedAgentType`
    /// - offset: 25
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "agentType")]
    AgentType(SerializedAgentType),
    /// # Field information in the original C++ class
    /// -   name:`"atom"`
    /// -   type: `struct hkpSimpleContactConstraintAtom`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "atom")]
    Atom(HkpSimpleContactConstraintAtom),
    /// # Field information in the original C++ class
    /// -   name:`"propertiesStream"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "propertiesStream")]
    PropertiesStream(Vec<Primitive<u8>>),
    /// # Field information in the original C++ class
    /// -   name:`"contactPoints"`
    /// -   type: `hkArray&lt;struct hkContactPoint&gt;`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "contactPoints")]
    ContactPoints(Vec<HkContactPoint>),
    /// # Field information in the original C++ class
    /// -   name:`"cpIdMgr"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "cpIdMgr")]
    CpIdMgr(Vec<Primitive<u8>>),
    /// # Field information in the original C++ class
    /// -   name:`"nnEntryData"`
    /// -   type: `hkUint8[160]`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nnEntryData")]
    NnEntryData([Primitive<u8>; 160]),
    /// # Field information in the original C++ class
    /// -   name:`"trackInfo"`
    /// -   type: `struct hkpSerializedTrack1nInfo`
    /// - offset: 276
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "trackInfo")]
    TrackInfo(HkpSerializedTrack1NInfo),
    /// # Field information in the original C++ class
    /// -   name:`"endianCheckBuffer"`
    /// -   type: `hkUint8[4]`
    /// - offset: 300
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "endianCheckBuffer")]
    EndianCheckBuffer([Primitive<u8>; 4]),
    /// # Field information in the original C++ class
    /// -   name:`"version"`
    /// -   type: `hkUint32`
    /// - offset: 304
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "version")]
    Version(Primitive<u32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpSerializedAgentNnEntryHkParam<'de>, "@name",
    ("bodyA" => BodyA(Cow<'a, str>)),
    ("bodyB" => BodyB(Cow<'a, str>)),
    ("bodyAId" => BodyAId(Primitive<u64>)),
    ("bodyBId" => BodyBId(Primitive<u64>)),
    ("useEntityIds" => UseEntityIds(Primitive<bool>)),
    ("agentType" => AgentType(SerializedAgentType)),
    ("atom" => Atom(HkpSimpleContactConstraintAtom)),
    ("propertiesStream" => PropertiesStream(Vec<Primitive<u8>>)),
    ("contactPoints" => ContactPoints(Vec<HkContactPoint>)),
    ("cpIdMgr" => CpIdMgr(Vec<Primitive<u8>>)),
    ("nnEntryData" => NnEntryData([Primitive<u8>; 160])),
    ("trackInfo" => TrackInfo(HkpSerializedTrack1NInfo)),
    ("endianCheckBuffer" => EndianCheckBuffer([Primitive<u8>; 4])),
    ("version" => Version(Primitive<u32>)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SerializedAgentType {
    #[serde(rename = "INVALID_AGENT_TYPE")]
    InvalidAgentType = 0,
    #[serde(rename = "BOX_BOX_AGENT3")]
    BoxBoxAgent3 = 1,
    #[serde(rename = "CAPSULE_TRIANGLE_AGENT3")]
    CapsuleTriangleAgent3 = 2,
    #[serde(rename = "PRED_GSK_AGENT3")]
    PredGskAgent3 = 3,
    #[serde(rename = "PRED_GSK_CYLINDER_AGENT3")]
    PredGskCylinderAgent3 = 4,
    #[serde(rename = "CONVEX_LIST_AGENT3")]
    ConvexListAgent3 = 5,
    #[serde(rename = "LIST_AGENT3")]
    ListAgent3 = 6,
    #[serde(rename = "BV_TREE_AGENT3")]
    BvTreeAgent3 = 7,
    #[serde(rename = "COLLECTION_COLLECTION_AGENT3")]
    CollectionCollectionAgent3 = 8,
    #[serde(rename = "COLLECTION_AGENT3")]
    CollectionAgent3 = 9,
}
