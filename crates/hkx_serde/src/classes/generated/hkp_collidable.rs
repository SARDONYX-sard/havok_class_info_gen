//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpCollidable`
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

/// `hkpCollidable`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 80
/// -    vtable: false
/// -    parent: `hkpCdBody`/`0x54a4b841`
/// - signature: `0x9a0e42a5`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpCollidable<'a> {
    /// # C++ Parent class(`hkpCdBody` => parent: `None`) field Info
    /// -   name:`"shape"`
    /// -   type: `struct hkpShape*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub shape: Cow<'a, str>,
    /// # C++ Parent class(`hkpCdBody` => parent: `None`) field Info
    /// -   name:`"shapeKey"`
    /// -   type: `hkUint32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub shape_key: u32,
    /// # C++ Parent class(`hkpCdBody` => parent: `None`) field Info
    /// -   name:`"motion"`
    /// -   type: `void*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub motion: Cow<'a, str>,
    /// # C++ Parent class(`hkpCdBody` => parent: `None`) field Info
    /// -   name:`"parent"`
    /// -   type: `struct hkpCdBody*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub parent: Cow<'a, str>,

    /// # C++ Class Fields Info
    /// -   name:`"ownerOffset"`
    /// -   type: `hkInt8`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub owner_offset: i8,
    /// # C++ Class Fields Info
    /// -   name:`"forceCollideOntoPpu"`
    /// -   type: `hkUint8`
    /// - offset: 17
    /// -  flags: `FLAGS_NONE`
    pub force_collide_onto_ppu: u8,
    /// # C++ Class Fields Info
    /// -   name:`"shapeSizeOnSpu"`
    /// -   type: `hkUint16`
    /// - offset: 18
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub shape_size_on_spu: u16,
    /// # C++ Class Fields Info
    /// -   name:`"broadPhaseHandle"`
    /// -   type: `struct hkpTypedBroadPhaseHandle`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub broad_phase_handle: SingleClass<HkpTypedBroadPhaseHandle>,
    /// # C++ Class Fields Info
    /// -   name:`"boundingVolumeData"`
    /// -   type: `struct hkpCollidableBoundingVolumeData`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub bounding_volume_data: SingleClass<HkpCollidableBoundingVolumeData<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"allowedPenetrationDepth"`
    /// -   type: `hkReal`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    pub allowed_penetration_depth: f32,
}

impl Serialize for HkpCollidable<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpCollidableVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpCollidable<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpCollidableVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpCollidableVisitor<'a>>> for HkpCollidable<'a> {
    fn from(_values: Vec<HkpCollidableVisitor<'a>>) -> Self {
            let mut shape = None;
            let mut shape_key = None;
            let mut motion = None;
            let mut parent = None;
            let mut owner_offset = None;
            let mut force_collide_onto_ppu = None;
            let mut shape_size_on_spu = None;
            let mut broad_phase_handle = None;
            let mut bounding_volume_data = None;
            let mut allowed_penetration_depth = None;


        for _value in _values {
            match _value {
                HkpCollidableVisitor::Shape(m) => shape = Some(m),
                HkpCollidableVisitor::ShapeKey(m) => shape_key = Some(m),
                HkpCollidableVisitor::Motion(m) => motion = Some(m),
                HkpCollidableVisitor::Parent(m) => parent = Some(m),
                HkpCollidableVisitor::OwnerOffset(m) => owner_offset = Some(m),
                HkpCollidableVisitor::ForceCollideOntoPpu(m) => force_collide_onto_ppu = Some(m),
                HkpCollidableVisitor::ShapeSizeOnSpu(m) => shape_size_on_spu = Some(m),
                HkpCollidableVisitor::BroadPhaseHandle(m) => broad_phase_handle = Some(m),
                HkpCollidableVisitor::BoundingVolumeData(m) => bounding_volume_data = Some(m),
                HkpCollidableVisitor::AllowedPenetrationDepth(m) => allowed_penetration_depth = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            shape: shape.unwrap_or_default().into_inner(),
            shape_key: shape_key.unwrap_or_default().into_inner(),
            motion: motion.unwrap_or_default().into_inner(),
            parent: parent.unwrap_or_default().into_inner(),
            owner_offset: owner_offset.unwrap_or_default().into_inner(),
            force_collide_onto_ppu: force_collide_onto_ppu.unwrap_or_default().into_inner(),
            shape_size_on_spu: shape_size_on_spu.unwrap_or_default().into_inner(),
            broad_phase_handle: broad_phase_handle.unwrap_or_default(),
            bounding_volume_data: bounding_volume_data.unwrap_or_default(),
            allowed_penetration_depth: allowed_penetration_depth.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpCollidable<'a>> for Vec<HkpCollidableVisitor<'a>> {
    fn from(data: &HkpCollidable<'a>) -> Self {
        vec![
            HkpCollidableVisitor::Shape(data.shape.clone().into()),
            HkpCollidableVisitor::ShapeKey(data.shape_key.into()),
            HkpCollidableVisitor::Motion(data.motion.clone().into()),
            HkpCollidableVisitor::Parent(data.parent.clone().into()),
            HkpCollidableVisitor::OwnerOffset(data.owner_offset.into()),
            HkpCollidableVisitor::ForceCollideOntoPpu(data.force_collide_onto_ppu.into()),
            HkpCollidableVisitor::ShapeSizeOnSpu(data.shape_size_on_spu.into()),
            HkpCollidableVisitor::BroadPhaseHandle(data.broad_phase_handle.clone()),
            HkpCollidableVisitor::BoundingVolumeData(data.bounding_volume_data.clone()),
            HkpCollidableVisitor::AllowedPenetrationDepth(data.allowed_penetration_depth.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpCollidable<'de> {
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
enum HkpCollidableVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "shape")]
    Shape(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "shapeKey")]
    ShapeKey(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "motion", skip_serializing)]
    Motion(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "parent", skip_serializing)]
    Parent(Primitive<Cow<'a, str>>),

    /// Visitor fields
    #[serde(rename = "ownerOffset", skip_serializing)]
    OwnerOffset(Primitive<i8>),
    /// Visitor fields
    #[serde(rename = "forceCollideOntoPpu")]
    ForceCollideOntoPpu(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "shapeSizeOnSpu", skip_serializing)]
    ShapeSizeOnSpu(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "broadPhaseHandle")]
    BroadPhaseHandle(SingleClass<HkpTypedBroadPhaseHandle>),
    /// Visitor fields
    #[serde(rename = "boundingVolumeData", skip_serializing)]
    BoundingVolumeData(SingleClass<HkpCollidableBoundingVolumeData<'a>>),
    /// Visitor fields
    #[serde(rename = "allowedPenetrationDepth")]
    AllowedPenetrationDepth(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpCollidableVisitor<'de>, "@name",
    ("shape" => Shape(Primitive<Cow<'de, str>>)),
    ("shapeKey" => ShapeKey(Primitive<u32>)),
    ("motion" => Motion(Primitive<Cow<'de, str>>)),
    ("parent" => Parent(Primitive<Cow<'de, str>>)),
    ("ownerOffset" => OwnerOffset(Primitive<i8>)),
    ("forceCollideOntoPpu" => ForceCollideOntoPpu(Primitive<u8>)),
    ("shapeSizeOnSpu" => ShapeSizeOnSpu(Primitive<u16>)),
    ("broadPhaseHandle" => BroadPhaseHandle(SingleClass<HkpTypedBroadPhaseHandle>)),
    ("boundingVolumeData" => BoundingVolumeData(SingleClass<HkpCollidableBoundingVolumeData<'de>>)),
    ("allowedPenetrationDepth" => AllowedPenetrationDepth(Primitive<f32>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum ForceCollideOntoPpuReasons {
    #[serde(rename = "FORCE_PPU_USER_REQUEST")]
    #[default]
    ForcePpuUserRequest = 1,
    #[serde(rename = "FORCE_PPU_SHAPE_REQUEST")]
    ForcePpuShapeRequest = 2,
    #[serde(rename = "FORCE_PPU_MODIFIER_REQUEST")]
    ForcePpuModifierRequest = 4,
    #[serde(rename = "FORCE_PPU_SHAPE_UNCHECKED")]
    ForcePpuShapeUnchecked = 8,
}
