//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpSimpleMeshShape`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#![allow(
  clippy::clone_on_copy,
  clippy::unit_arg
)]

#[allow(unused)]
use super::*;
#[allow(unused)]
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkpSimpleMeshShape`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 68
/// -    vtable: true
/// -    parent: `hkpShapeCollection`/`0xe8c3991d`
/// - signature: `0x16b3c811`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpSimpleMeshShape {
    /// # C++ Parent class(`hkpShapeCollection` => parent: `hkpShape`) field Info
    /// -   name:`"disableWelding"`
    /// -   type: `hkBool`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub disable_welding: bool,
    /// # C++ Parent class(`hkpShapeCollection` => parent: `hkpShape`) field Info
    /// -   name:`"collectionType"`
    /// -   type: `enum CollectionType`
    /// - offset: 21
    /// -  flags: `FLAGS_NONE`
    pub collection_type: CollectionType,

    /// # C++ Parent class(`hkpShape` => parent: `hkReferencedObject`) field Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub user_data: usize,
    /// # C++ Parent class(`hkpShape` => parent: `hkReferencedObject`) field Info
    /// -   name:`"type"`
    /// -   type: `enum unknown`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub _type: (),

    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub mem_size_and_flags: u16,
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub reference_count: i16,

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// # C++ Class Fields Info
    /// -   name:`"vertices"`
    /// -   type: `hkArray<hkVector4>`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub vertices: HkArrayVector<Vector4<f32>>,
    /// # C++ Class Fields Info
    /// -   name:`"triangles"`
    /// -   type: `hkArray<struct hkpSimpleMeshShapeTriangle>`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    pub triangles: HkArrayClass<HkpSimpleMeshShapeTriangle>,
    /// # C++ Class Fields Info
    /// -   name:`"materialIndices"`
    /// -   type: `hkArray<hkUint8>`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub material_indices: HkArrayNum<u8>,
    /// # C++ Class Fields Info
    /// -   name:`"radius"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    pub radius: f32,
    /// # C++ Class Fields Info
    /// -   name:`"weldingType"`
    /// -   type: `enum WeldingType`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub welding_type: WeldingType,
}

impl Serialize for HkpSimpleMeshShape {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpSimpleMeshShapeVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpSimpleMeshShape {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpSimpleMeshShapeVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpSimpleMeshShapeVisitor>> for HkpSimpleMeshShape {
    fn from(_values: Vec<HkpSimpleMeshShapeVisitor>) -> Self {
            let mut disable_welding = None;
            let mut collection_type = None;
            let mut user_data = None;
            let mut _type = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut vertices = None;
            let mut triangles = None;
            let mut material_indices = None;
            let mut radius = None;
            let mut welding_type = None;


        for _value in _values {
            match _value {
                HkpSimpleMeshShapeVisitor::DisableWelding(m) => disable_welding = Some(m),
                HkpSimpleMeshShapeVisitor::CollectionType(m) => collection_type = Some(m),
                HkpSimpleMeshShapeVisitor::UserData(m) => user_data = Some(m),
                HkpSimpleMeshShapeVisitor::Type(m) => _type = Some(m),
                HkpSimpleMeshShapeVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpSimpleMeshShapeVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpSimpleMeshShapeVisitor::Vertices(m) => vertices = Some(m),
                HkpSimpleMeshShapeVisitor::Triangles(m) => triangles = Some(m),
                HkpSimpleMeshShapeVisitor::MaterialIndices(m) => material_indices = Some(m),
                HkpSimpleMeshShapeVisitor::Radius(m) => radius = Some(m),
                HkpSimpleMeshShapeVisitor::WeldingType(m) => welding_type = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            disable_welding: disable_welding.unwrap_or_default().into_inner(),
            collection_type: collection_type.unwrap_or_default().into_inner(),
            user_data: user_data.unwrap_or_default().into_inner(),
            _type: _type.unwrap_or_default().into_inner(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            vertices: vertices.unwrap_or_default(),
            triangles: triangles.unwrap_or_default(),
            material_indices: material_indices.unwrap_or_default(),
            radius: radius.unwrap_or_default().into_inner(),
            welding_type: welding_type.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpSimpleMeshShape> for Vec<HkpSimpleMeshShapeVisitor> {
    fn from(data: &HkpSimpleMeshShape) -> Self {
        vec![
            HkpSimpleMeshShapeVisitor::DisableWelding(data.disable_welding.into()),
            HkpSimpleMeshShapeVisitor::CollectionType(data.collection_type.clone().into()),
            HkpSimpleMeshShapeVisitor::UserData(data.user_data.into()),
            HkpSimpleMeshShapeVisitor::Type(data._type.into()),
            HkpSimpleMeshShapeVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpSimpleMeshShapeVisitor::ReferenceCount(data.reference_count.into()),
            HkpSimpleMeshShapeVisitor::Vertices(data.vertices.clone()),
            HkpSimpleMeshShapeVisitor::Triangles(data.triangles.clone()),
            HkpSimpleMeshShapeVisitor::MaterialIndices(data.material_indices.clone()),
            HkpSimpleMeshShapeVisitor::Radius(data.radius.into()),
            HkpSimpleMeshShapeVisitor::WeldingType(data.welding_type.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpSimpleMeshShape {
    fn from_bytes<B>(
        _bytes: &'bytes [u8],
        _de: &mut PackFileDeserializer,
    ) -> Result<Self>
    where
        B: ByteOrder,
        Self: Sized + 'de
    {
        todo!()
    }
}


/// # Why use Visitor pattern?
/// Since the C++ field must be deserialized from the `name` attribute name of the `hkparam` in the XML,
/// this is accomplished by having the Visitor process the internally tagged enum and convert it.
/// Leakage of field items may occur if Vec<enum> is left as it is.
///
/// struct -> (De)serialize by visitor -> struct
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
enum HkpSimpleMeshShapeVisitor {
    /// Visitor fields
    #[serde(rename = "disableWelding")]
    DisableWelding(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "collectionType")]
    CollectionType(Primitive<CollectionType>),

    /// Visitor fields
    #[serde(rename = "userData")]
    UserData(Primitive<usize>),
    /// Visitor fields
    #[serde(rename = "type", skip_serializing)]
    Type(Primitive<()>),

    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "vertices")]
    Vertices(HkArrayVector<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "triangles")]
    Triangles(HkArrayClass<HkpSimpleMeshShapeTriangle>),
    /// Visitor fields
    #[serde(rename = "materialIndices")]
    MaterialIndices(HkArrayNum<u8>),
    /// Visitor fields
    #[serde(rename = "radius")]
    Radius(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "weldingType")]
    WeldingType(Primitive<WeldingType>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpSimpleMeshShapeVisitor, "@name",
    ("disableWelding" => DisableWelding(Primitive<bool>)),
    ("collectionType" => CollectionType(Primitive<CollectionType>)),
    ("userData" => UserData(Primitive<usize>)),
    ("type" => Type(Primitive<()>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("vertices" => Vertices(HkArrayVector<Vector4<f32>>)),
    ("triangles" => Triangles(HkArrayClass<HkpSimpleMeshShapeTriangle>)),
    ("materialIndices" => MaterialIndices(HkArrayNum<u8>)),
    ("radius" => Radius(Primitive<f32>)),
    ("weldingType" => WeldingType(Primitive<WeldingType>)),
}
