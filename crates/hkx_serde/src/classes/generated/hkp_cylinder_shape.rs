//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpCylinderShape`
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

/// `hkpCylinderShape`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 96
/// -    vtable: true
/// -    parent: `hkpConvexShape`/`0xf8f74f85`
/// - signature: `0x3e463c3a`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpCylinderShape {
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
    /// -   name:`"cylRadius"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub cyl_radius: f32,
    /// # C++ Class Fields Info
    /// -   name:`"cylBaseRadiusFactorForHeightFieldCollisions"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub cyl_base_radius_factor_for_height_field_collisions: f32,
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
    /// -   name:`"perpendicular1"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub perpendicular_1: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"perpendicular2"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub perpendicular_2: Vector4<f32>,
}

impl Serialize for HkpCylinderShape {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpCylinderShapeVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpCylinderShape {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpCylinderShapeVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpCylinderShapeVisitor>> for HkpCylinderShape {
    fn from(_values: Vec<HkpCylinderShapeVisitor>) -> Self {
            let mut radius = None;
            let mut user_data = None;
            let mut _type = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut cyl_radius = None;
            let mut cyl_base_radius_factor_for_height_field_collisions = None;
            let mut vertex_a = None;
            let mut vertex_b = None;
            let mut perpendicular_1 = None;
            let mut perpendicular_2 = None;


        for _value in _values {
            match _value {
                HkpCylinderShapeVisitor::Radius(m) => radius = Some(m),
                HkpCylinderShapeVisitor::UserData(m) => user_data = Some(m),
                HkpCylinderShapeVisitor::Type(m) => _type = Some(m),
                HkpCylinderShapeVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpCylinderShapeVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpCylinderShapeVisitor::CylRadius(m) => cyl_radius = Some(m),
                HkpCylinderShapeVisitor::CylBaseRadiusFactorForHeightFieldCollisions(m) => cyl_base_radius_factor_for_height_field_collisions = Some(m),
                HkpCylinderShapeVisitor::VertexA(m) => vertex_a = Some(m),
                HkpCylinderShapeVisitor::VertexB(m) => vertex_b = Some(m),
                HkpCylinderShapeVisitor::Perpendicular1(m) => perpendicular_1 = Some(m),
                HkpCylinderShapeVisitor::Perpendicular2(m) => perpendicular_2 = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            radius: radius.unwrap_or_default().into_inner(),
            user_data: user_data.unwrap_or_default().into_inner(),
            _type: _type.unwrap_or_default().into_inner(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            cyl_radius: cyl_radius.unwrap_or_default().into_inner(),
            cyl_base_radius_factor_for_height_field_collisions: cyl_base_radius_factor_for_height_field_collisions.unwrap_or_default().into_inner(),
            vertex_a: vertex_a.unwrap_or_default().into_inner(),
            vertex_b: vertex_b.unwrap_or_default().into_inner(),
            perpendicular_1: perpendicular_1.unwrap_or_default().into_inner(),
            perpendicular_2: perpendicular_2.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpCylinderShape> for Vec<HkpCylinderShapeVisitor> {
    fn from(data: &HkpCylinderShape) -> Self {
        vec![
            HkpCylinderShapeVisitor::Radius(data.radius.into()),
            HkpCylinderShapeVisitor::UserData(data.user_data.into()),
            HkpCylinderShapeVisitor::Type(data._type.into()),
            HkpCylinderShapeVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpCylinderShapeVisitor::ReferenceCount(data.reference_count.into()),
            HkpCylinderShapeVisitor::CylRadius(data.cyl_radius.into()),
            HkpCylinderShapeVisitor::CylBaseRadiusFactorForHeightFieldCollisions(data.cyl_base_radius_factor_for_height_field_collisions.into()),
            HkpCylinderShapeVisitor::VertexA(data.vertex_a.into()),
            HkpCylinderShapeVisitor::VertexB(data.vertex_b.into()),
            HkpCylinderShapeVisitor::Perpendicular1(data.perpendicular_1.into()),
            HkpCylinderShapeVisitor::Perpendicular2(data.perpendicular_2.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpCylinderShape {
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
enum HkpCylinderShapeVisitor {
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
    #[serde(rename = "cylRadius")]
    CylRadius(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "cylBaseRadiusFactorForHeightFieldCollisions")]
    CylBaseRadiusFactorForHeightFieldCollisions(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "vertexA")]
    VertexA(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "vertexB")]
    VertexB(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "perpendicular1")]
    Perpendicular1(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "perpendicular2")]
    Perpendicular2(Primitive<Vector4<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpCylinderShapeVisitor, "@name",
    ("radius" => Radius(Primitive<f32>)),
    ("userData" => UserData(Primitive<usize>)),
    ("type" => Type(Primitive<()>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("cylRadius" => CylRadius(Primitive<f32>)),
    ("cylBaseRadiusFactorForHeightFieldCollisions" => CylBaseRadiusFactorForHeightFieldCollisions(Primitive<f32>)),
    ("vertexA" => VertexA(Primitive<Vector4<f32>>)),
    ("vertexB" => VertexB(Primitive<Vector4<f32>>)),
    ("perpendicular1" => Perpendicular1(Primitive<Vector4<f32>>)),
    ("perpendicular2" => Perpendicular2(Primitive<Vector4<f32>>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum VertexIdEncoding {
    #[serde(rename = "VERTEX_ID_ENCODING_IS_BASE_A_SHIFT")]
    #[default]
    VertexIdEncodingIsBaseAShift = 7,
    #[serde(rename = "VERTEX_ID_ENCODING_SIN_SIGN_SHIFT")]
    VertexIdEncodingSinSignShift = 6,
    #[serde(rename = "VERTEX_ID_ENCODING_COS_SIGN_SHIFT")]
    VertexIdEncodingCosSignShift = 5,
    #[serde(rename = "VERTEX_ID_ENCODING_IS_SIN_LESSER_SHIFT")]
    VertexIdEncodingIsSinLesserShift = 4,
    #[serde(rename = "VERTEX_ID_ENCODING_VALUE_MASK")]
    VertexIdEncodingValueMask = 15,
}
