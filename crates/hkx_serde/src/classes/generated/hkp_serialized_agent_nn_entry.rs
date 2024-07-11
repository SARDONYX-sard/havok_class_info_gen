//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpSerializedAgentNnEntry`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpSerializedAgentNnEntry`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 320
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x49ec7de3`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpSerializedAgentNnEntry<'a> {
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields

    /// # C++ Class Fields Info
    /// -   name:`"bodyA"`
    /// -   type: `struct hkpEntity*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bodyA")]
    BodyA(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"bodyB"`
    /// -   type: `struct hkpEntity*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bodyB")]
    BodyB(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"bodyAId"`
    /// -   type: `hkUlong`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bodyAId")]
    BodyAId(Primitive<usize>),
    /// # C++ Class Fields Info
    /// -   name:`"bodyBId"`
    /// -   type: `hkUlong`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bodyBId")]
    BodyBId(Primitive<usize>),
    /// # C++ Class Fields Info
    /// -   name:`"useEntityIds"`
    /// -   type: `hkBool`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "useEntityIds")]
    UseEntityIds(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"agentType"`
    /// -   type: `enum SerializedAgentType`
    /// - offset: 25
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "agentType")]
    AgentType(Primitive<SerializedAgentType>),
    /// # C++ Class Fields Info
    /// -   name:`"atom"`
    /// -   type: `struct hkpSimpleContactConstraintAtom`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "atom")]
    Atom(HkpSimpleContactConstraintAtom),
    /// # C++ Class Fields Info
    /// -   name:`"propertiesStream"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "propertiesStream")]
    PropertiesStream(HkArrayRef<Primitive<u8>>),
    /// # C++ Class Fields Info
    /// -   name:`"contactPoints"`
    /// -   type: `hkArray&lt;struct hkContactPoint&gt;`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "contactPoints")]
    ContactPoints(HkArrayClass<HkContactPoint>),
    /// # C++ Class Fields Info
    /// -   name:`"cpIdMgr"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "cpIdMgr")]
    CpIdMgr(HkArrayRef<Primitive<u8>>),
    /// # C++ Class Fields Info
    /// -   name:`"nnEntryData"`
    /// -   type: `hkUint8[160]`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nnEntryData")]
    NnEntryData(CStyleArray<[u8; 160]>),
    /// # C++ Class Fields Info
    /// -   name:`"trackInfo"`
    /// -   type: `struct hkpSerializedTrack1nInfo`
    /// - offset: 276
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "trackInfo")]
    TrackInfo(HkpSerializedTrack1NInfo<'a>),
    /// # C++ Class Fields Info
    /// -   name:`"endianCheckBuffer"`
    /// -   type: `hkUint8[4]`
    /// - offset: 300
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "endianCheckBuffer")]
    EndianCheckBuffer(CStyleArray<[u8; 4]>),
    /// # C++ Class Fields Info
    /// -   name:`"version"`
    /// -   type: `hkUint32`
    /// - offset: 304
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "version")]
    Version(Primitive<u32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpSerializedAgentNnEntry<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("bodyA" => BodyA(Primitive<Cow<'de, str>>)),
    ("bodyB" => BodyB(Primitive<Cow<'de, str>>)),
    ("bodyAId" => BodyAId(Primitive<usize>)),
    ("bodyBId" => BodyBId(Primitive<usize>)),
    ("useEntityIds" => UseEntityIds(Primitive<bool>)),
    ("agentType" => AgentType(Primitive<SerializedAgentType>)),
    ("atom" => Atom(HkpSimpleContactConstraintAtom)),
    ("propertiesStream" => PropertiesStream(HkArrayRef<Primitive<u8>>)),
    ("contactPoints" => ContactPoints(HkArrayClass<HkContactPoint>)),
    ("cpIdMgr" => CpIdMgr(HkArrayRef<Primitive<u8>>)),
    ("nnEntryData" => NnEntryData(CStyleArray<[u8; 160]>)),
    ("trackInfo" => TrackInfo(HkpSerializedTrack1NInfo<'de>)),
    ("endianCheckBuffer" => EndianCheckBuffer(CStyleArray<[u8; 4]>)),
    ("version" => Version(Primitive<u32>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
