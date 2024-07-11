//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpSampledHeightFieldShape`
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

/// `hkpSampledHeightFieldShape`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 96
/// -    vtable: true
/// -    parent: `hkpHeightFieldShape`/`0xe7eca7eb`
/// - signature: `0x11213421`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpSampledHeightFieldShape {
    // C++ Parent class(`hkpHeightFieldShape` => parent: `hkpShape`) has no fields
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
    /// -   name:`"xRes"`
    /// -   type: `hkInt32`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub x_res: i32,
    /// # C++ Class Fields Info
    /// -   name:`"zRes"`
    /// -   type: `hkInt32`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub z_res: i32,
    /// # C++ Class Fields Info
    /// -   name:`"heightCenter"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub height_center: f32,
    /// # C++ Class Fields Info
    /// -   name:`"useProjectionBasedHeight"`
    /// -   type: `hkBool`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub use_projection_based_height: bool,
    /// # C++ Class Fields Info
    /// -   name:`"heightfieldType"`
    /// -   type: `enum HeightFieldType`
    /// - offset: 29
    /// -  flags: `FLAGS_NONE`
    pub heightfield_type: HeightFieldType,
    /// # C++ Class Fields Info
    /// -   name:`"intToFloatScale"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub int_to_float_scale: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"floatToIntScale"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub float_to_int_scale: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"floatToIntOffsetFloorCorrected"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub float_to_int_offset_floor_corrected: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"extents"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub extents: Vector4<f32>,
}

impl Serialize for HkpSampledHeightFieldShape {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpSampledHeightFieldShapeVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpSampledHeightFieldShape {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpSampledHeightFieldShapeVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpSampledHeightFieldShapeVisitor>> for HkpSampledHeightFieldShape {
    fn from(_values: Vec<HkpSampledHeightFieldShapeVisitor>) -> Self {
            let mut user_data = None;
            let mut _type = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut x_res = None;
            let mut z_res = None;
            let mut height_center = None;
            let mut use_projection_based_height = None;
            let mut heightfield_type = None;
            let mut int_to_float_scale = None;
            let mut float_to_int_scale = None;
            let mut float_to_int_offset_floor_corrected = None;
            let mut extents = None;


        for _value in _values {
            match _value {
                HkpSampledHeightFieldShapeVisitor::UserData(m) => user_data = Some(m),
                HkpSampledHeightFieldShapeVisitor::Type(m) => _type = Some(m),
                HkpSampledHeightFieldShapeVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpSampledHeightFieldShapeVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpSampledHeightFieldShapeVisitor::XRes(m) => x_res = Some(m),
                HkpSampledHeightFieldShapeVisitor::ZRes(m) => z_res = Some(m),
                HkpSampledHeightFieldShapeVisitor::HeightCenter(m) => height_center = Some(m),
                HkpSampledHeightFieldShapeVisitor::UseProjectionBasedHeight(m) => use_projection_based_height = Some(m),
                HkpSampledHeightFieldShapeVisitor::HeightfieldType(m) => heightfield_type = Some(m),
                HkpSampledHeightFieldShapeVisitor::IntToFloatScale(m) => int_to_float_scale = Some(m),
                HkpSampledHeightFieldShapeVisitor::FloatToIntScale(m) => float_to_int_scale = Some(m),
                HkpSampledHeightFieldShapeVisitor::FloatToIntOffsetFloorCorrected(m) => float_to_int_offset_floor_corrected = Some(m),
                HkpSampledHeightFieldShapeVisitor::Extents(m) => extents = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            user_data: user_data.unwrap_or_default().into_inner(),
            _type: _type.unwrap_or_default().into_inner(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            x_res: x_res.unwrap_or_default().into_inner(),
            z_res: z_res.unwrap_or_default().into_inner(),
            height_center: height_center.unwrap_or_default().into_inner(),
            use_projection_based_height: use_projection_based_height.unwrap_or_default().into_inner(),
            heightfield_type: heightfield_type.unwrap_or_default().into_inner(),
            int_to_float_scale: int_to_float_scale.unwrap_or_default().into_inner(),
            float_to_int_scale: float_to_int_scale.unwrap_or_default().into_inner(),
            float_to_int_offset_floor_corrected: float_to_int_offset_floor_corrected.unwrap_or_default().into_inner(),
            extents: extents.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpSampledHeightFieldShape> for Vec<HkpSampledHeightFieldShapeVisitor> {
    fn from(data: &HkpSampledHeightFieldShape) -> Self {
        vec![
            HkpSampledHeightFieldShapeVisitor::UserData(data.user_data.into()),
            HkpSampledHeightFieldShapeVisitor::Type(data._type.into()),
            HkpSampledHeightFieldShapeVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpSampledHeightFieldShapeVisitor::ReferenceCount(data.reference_count.into()),
            HkpSampledHeightFieldShapeVisitor::XRes(data.x_res.into()),
            HkpSampledHeightFieldShapeVisitor::ZRes(data.z_res.into()),
            HkpSampledHeightFieldShapeVisitor::HeightCenter(data.height_center.into()),
            HkpSampledHeightFieldShapeVisitor::UseProjectionBasedHeight(data.use_projection_based_height.into()),
            HkpSampledHeightFieldShapeVisitor::HeightfieldType(data.heightfield_type.clone().into()),
            HkpSampledHeightFieldShapeVisitor::IntToFloatScale(data.int_to_float_scale.into()),
            HkpSampledHeightFieldShapeVisitor::FloatToIntScale(data.float_to_int_scale.into()),
            HkpSampledHeightFieldShapeVisitor::FloatToIntOffsetFloorCorrected(data.float_to_int_offset_floor_corrected.into()),
            HkpSampledHeightFieldShapeVisitor::Extents(data.extents.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpSampledHeightFieldShape {
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
enum HkpSampledHeightFieldShapeVisitor {
    // C++ Parent class(`hkpHeightFieldShape` => parent: `hkpShape`) has no fields
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
    #[serde(rename = "xRes")]
    XRes(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "zRes")]
    ZRes(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "heightCenter")]
    HeightCenter(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "useProjectionBasedHeight")]
    UseProjectionBasedHeight(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "heightfieldType")]
    HeightfieldType(Primitive<HeightFieldType>),
    /// Visitor fields
    #[serde(rename = "intToFloatScale")]
    IntToFloatScale(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "floatToIntScale")]
    FloatToIntScale(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "floatToIntOffsetFloorCorrected")]
    FloatToIntOffsetFloorCorrected(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "extents")]
    Extents(Primitive<Vector4<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpSampledHeightFieldShapeVisitor, "@name",
    ("userData" => UserData(Primitive<usize>)),
    ("type" => Type(Primitive<()>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("xRes" => XRes(Primitive<i32>)),
    ("zRes" => ZRes(Primitive<i32>)),
    ("heightCenter" => HeightCenter(Primitive<f32>)),
    ("useProjectionBasedHeight" => UseProjectionBasedHeight(Primitive<bool>)),
    ("heightfieldType" => HeightfieldType(Primitive<HeightFieldType>)),
    ("intToFloatScale" => IntToFloatScale(Primitive<Vector4<f32>>)),
    ("floatToIntScale" => FloatToIntScale(Primitive<Vector4<f32>>)),
    ("floatToIntOffsetFloorCorrected" => FloatToIntOffsetFloorCorrected(Primitive<Vector4<f32>>)),
    ("extents" => Extents(Primitive<Vector4<f32>>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum HeightFieldType {
    #[serde(rename = "HEIGHTFIELD_STORAGE")]
    #[default]
    HeightfieldStorage = 0,
    #[serde(rename = "HEIGHTFIELD_COMPRESSED")]
    HeightfieldCompressed = 1,
    #[serde(rename = "HEIGHTFIELD_USER")]
    HeightfieldUser = 2,
    #[serde(rename = "HEIGHTFIELD_MAX_ID")]
    HeightfieldMaxId = 3,
}
