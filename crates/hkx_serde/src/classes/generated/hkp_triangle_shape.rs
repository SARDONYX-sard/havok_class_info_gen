//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpTriangleShape`
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

/// `hkpTriangleShape`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 96
/// -    vtable: true
/// -    parent: `hkpConvexShape`/`0xf8f74f85`
/// - signature: `0x95ad1a25`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpTriangleShape {
    /// # C++ Parent class(`hkpConvexShape` => parent: `hkpSphereRepShape`) field Info
    /// -   name:`"radius"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub radius: f32,

    // C++ Parent class(`hkpSphereRepShape` => parent: `hkpShape`) has no fields
    //
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
    /// -   name:`"weldingInfo"`
    /// -   type: `hkUint16`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub welding_info: u16,
    /// # C++ Class Fields Info
    /// -   name:`"weldingType"`
    /// -   type: `enum WeldingType`
    /// - offset: 22
    /// -  flags: `FLAGS_NONE`
    pub welding_type: WeldingType,
    /// # C++ Class Fields Info
    /// -   name:`"isExtruded"`
    /// -   type: `hkUint8`
    /// - offset: 23
    /// -  flags: `FLAGS_NONE`
    pub is_extruded: u8,
    /// # C++ Class Fields Info
    /// -   name:`"vertexA"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub vertex_a: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"vertexB"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub vertex_b: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"vertexC"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub vertex_c: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"extrusion"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub extrusion: Vector4<f32>,
}

impl Serialize for HkpTriangleShape {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpTriangleShapeVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpTriangleShape {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpTriangleShapeVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpTriangleShapeVisitor>> for HkpTriangleShape {
    fn from(_values: Vec<HkpTriangleShapeVisitor>) -> Self {
            let mut radius = None;
            let mut user_data = None;
            let mut _type = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut welding_info = None;
            let mut welding_type = None;
            let mut is_extruded = None;
            let mut vertex_a = None;
            let mut vertex_b = None;
            let mut vertex_c = None;
            let mut extrusion = None;


        for _value in _values {
            match _value {
                HkpTriangleShapeVisitor::Radius(m) => radius = Some(m),
                HkpTriangleShapeVisitor::UserData(m) => user_data = Some(m),
                HkpTriangleShapeVisitor::Type(m) => _type = Some(m),
                HkpTriangleShapeVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpTriangleShapeVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpTriangleShapeVisitor::WeldingInfo(m) => welding_info = Some(m),
                HkpTriangleShapeVisitor::WeldingType(m) => welding_type = Some(m),
                HkpTriangleShapeVisitor::IsExtruded(m) => is_extruded = Some(m),
                HkpTriangleShapeVisitor::VertexA(m) => vertex_a = Some(m),
                HkpTriangleShapeVisitor::VertexB(m) => vertex_b = Some(m),
                HkpTriangleShapeVisitor::VertexC(m) => vertex_c = Some(m),
                HkpTriangleShapeVisitor::Extrusion(m) => extrusion = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            radius: radius.unwrap_or_default().into_inner(),
            user_data: user_data.unwrap_or_default().into_inner(),
            _type: _type.unwrap_or_default().into_inner(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            welding_info: welding_info.unwrap_or_default().into_inner(),
            welding_type: welding_type.unwrap_or_default().into_inner(),
            is_extruded: is_extruded.unwrap_or_default().into_inner(),
            vertex_a: vertex_a.unwrap_or_default().into_inner(),
            vertex_b: vertex_b.unwrap_or_default().into_inner(),
            vertex_c: vertex_c.unwrap_or_default().into_inner(),
            extrusion: extrusion.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpTriangleShape> for Vec<HkpTriangleShapeVisitor> {
    fn from(data: &HkpTriangleShape) -> Self {
        vec![
            HkpTriangleShapeVisitor::Radius(data.radius.into()),
            HkpTriangleShapeVisitor::UserData(data.user_data.into()),
            HkpTriangleShapeVisitor::Type(data._type.into()),
            HkpTriangleShapeVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpTriangleShapeVisitor::ReferenceCount(data.reference_count.into()),
            HkpTriangleShapeVisitor::WeldingInfo(data.welding_info.into()),
            HkpTriangleShapeVisitor::WeldingType(data.welding_type.clone().into()),
            HkpTriangleShapeVisitor::IsExtruded(data.is_extruded.into()),
            HkpTriangleShapeVisitor::VertexA(data.vertex_a.into()),
            HkpTriangleShapeVisitor::VertexB(data.vertex_b.into()),
            HkpTriangleShapeVisitor::VertexC(data.vertex_c.into()),
            HkpTriangleShapeVisitor::Extrusion(data.extrusion.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpTriangleShape {
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
enum HkpTriangleShapeVisitor {
    /// Visitor fields
    #[serde(rename = "radius")]
    Radius(Primitive<f32>),

    // C++ Parent class(`hkpSphereRepShape` => parent: `hkpShape`) has no fields
    //
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
    #[serde(rename = "weldingInfo")]
    WeldingInfo(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "weldingType")]
    WeldingType(Primitive<WeldingType>),
    /// Visitor fields
    #[serde(rename = "isExtruded")]
    IsExtruded(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "vertexA")]
    VertexA(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "vertexB")]
    VertexB(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "vertexC")]
    VertexC(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "extrusion")]
    Extrusion(Primitive<Vector4<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpTriangleShapeVisitor, "@name",
    ("radius" => Radius(Primitive<f32>)),
    ("userData" => UserData(Primitive<usize>)),
    ("type" => Type(Primitive<()>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("weldingInfo" => WeldingInfo(Primitive<u16>)),
    ("weldingType" => WeldingType(Primitive<WeldingType>)),
    ("isExtruded" => IsExtruded(Primitive<u8>)),
    ("vertexA" => VertexA(Primitive<Vector4<f32>>)),
    ("vertexB" => VertexB(Primitive<Vector4<f32>>)),
    ("vertexC" => VertexC(Primitive<Vector4<f32>>)),
    ("extrusion" => Extrusion(Primitive<Vector4<f32>>)),
}
