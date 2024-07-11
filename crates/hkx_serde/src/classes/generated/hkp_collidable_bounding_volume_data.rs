//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpCollidableBoundingVolumeData`
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

/// `hkpCollidableBoundingVolumeData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 44
/// -    vtable: false
/// - signature: `0xb5f0e6b1`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpCollidableBoundingVolumeData<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"min"`
    /// -   type: `hkUint32[3]`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub min: CStyleArray<[u32; 3]>,
    /// # C++ Class Fields Info
    /// -   name:`"expansionMin"`
    /// -   type: `hkUint8[3]`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub expansion_min: CStyleArray<[u8; 3]>,
    /// # C++ Class Fields Info
    /// -   name:`"expansionShift"`
    /// -   type: `hkUint8`
    /// - offset: 15
    /// -  flags: `FLAGS_NONE`
    pub expansion_shift: u8,
    /// # C++ Class Fields Info
    /// -   name:`"max"`
    /// -   type: `hkUint32[3]`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub max: CStyleArray<[u32; 3]>,
    /// # C++ Class Fields Info
    /// -   name:`"expansionMax"`
    /// -   type: `hkUint8[3]`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub expansion_max: CStyleArray<[u8; 3]>,
    /// # C++ Class Fields Info
    /// -   name:`"padding"`
    /// -   type: `hkUint8`
    /// - offset: 31
    /// -  flags: `FLAGS_NONE`
    pub padding: u8,
    /// # C++ Class Fields Info
    /// -   name:`"numChildShapeAabbs"`
    /// -   type: `hkUint16`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub num_child_shape_aabbs: u16,
    /// # C++ Class Fields Info
    /// -   name:`"capacityChildShapeAabbs"`
    /// -   type: `hkUint16`
    /// - offset: 34
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub capacity_child_shape_aabbs: u16,
    /// # C++ Class Fields Info
    /// -   name:`"childShapeAabbs"`
    /// -   type: `void*`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub child_shape_aabbs: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"childShapeKeys"`
    /// -   type: `void*`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub child_shape_keys: Cow<'a, str>,
}

impl Serialize for HkpCollidableBoundingVolumeData<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpCollidableBoundingVolumeDataVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpCollidableBoundingVolumeData<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpCollidableBoundingVolumeDataVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpCollidableBoundingVolumeDataVisitor<'a>>> for HkpCollidableBoundingVolumeData<'a> {
    fn from(_values: Vec<HkpCollidableBoundingVolumeDataVisitor<'a>>) -> Self {
            let mut min = None;
            let mut expansion_min = None;
            let mut expansion_shift = None;
            let mut max = None;
            let mut expansion_max = None;
            let mut padding = None;
            let mut num_child_shape_aabbs = None;
            let mut capacity_child_shape_aabbs = None;
            let mut child_shape_aabbs = None;
            let mut child_shape_keys = None;


        for _value in _values {
            match _value {
                HkpCollidableBoundingVolumeDataVisitor::Min(m) => min = Some(m),
                HkpCollidableBoundingVolumeDataVisitor::ExpansionMin(m) => expansion_min = Some(m),
                HkpCollidableBoundingVolumeDataVisitor::ExpansionShift(m) => expansion_shift = Some(m),
                HkpCollidableBoundingVolumeDataVisitor::Max(m) => max = Some(m),
                HkpCollidableBoundingVolumeDataVisitor::ExpansionMax(m) => expansion_max = Some(m),
                HkpCollidableBoundingVolumeDataVisitor::Padding(m) => padding = Some(m),
                HkpCollidableBoundingVolumeDataVisitor::NumChildShapeAabbs(m) => num_child_shape_aabbs = Some(m),
                HkpCollidableBoundingVolumeDataVisitor::CapacityChildShapeAabbs(m) => capacity_child_shape_aabbs = Some(m),
                HkpCollidableBoundingVolumeDataVisitor::ChildShapeAabbs(m) => child_shape_aabbs = Some(m),
                HkpCollidableBoundingVolumeDataVisitor::ChildShapeKeys(m) => child_shape_keys = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            min: min.unwrap_or_default(),
            expansion_min: expansion_min.unwrap_or_default(),
            expansion_shift: expansion_shift.unwrap_or_default().into_inner(),
            max: max.unwrap_or_default(),
            expansion_max: expansion_max.unwrap_or_default(),
            padding: padding.unwrap_or_default().into_inner(),
            num_child_shape_aabbs: num_child_shape_aabbs.unwrap_or_default().into_inner(),
            capacity_child_shape_aabbs: capacity_child_shape_aabbs.unwrap_or_default().into_inner(),
            child_shape_aabbs: child_shape_aabbs.unwrap_or_default().into_inner(),
            child_shape_keys: child_shape_keys.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpCollidableBoundingVolumeData<'a>> for Vec<HkpCollidableBoundingVolumeDataVisitor<'a>> {
    fn from(data: &HkpCollidableBoundingVolumeData<'a>) -> Self {
        vec![
            HkpCollidableBoundingVolumeDataVisitor::Min(data.min.clone()),
            HkpCollidableBoundingVolumeDataVisitor::ExpansionMin(data.expansion_min.clone()),
            HkpCollidableBoundingVolumeDataVisitor::ExpansionShift(data.expansion_shift.into()),
            HkpCollidableBoundingVolumeDataVisitor::Max(data.max.clone()),
            HkpCollidableBoundingVolumeDataVisitor::ExpansionMax(data.expansion_max.clone()),
            HkpCollidableBoundingVolumeDataVisitor::Padding(data.padding.into()),
            HkpCollidableBoundingVolumeDataVisitor::NumChildShapeAabbs(data.num_child_shape_aabbs.into()),
            HkpCollidableBoundingVolumeDataVisitor::CapacityChildShapeAabbs(data.capacity_child_shape_aabbs.into()),
            HkpCollidableBoundingVolumeDataVisitor::ChildShapeAabbs(data.child_shape_aabbs.clone().into()),
            HkpCollidableBoundingVolumeDataVisitor::ChildShapeKeys(data.child_shape_keys.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpCollidableBoundingVolumeData<'de> {
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
enum HkpCollidableBoundingVolumeDataVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "min")]
    Min(CStyleArray<[u32; 3]>),
    /// Visitor fields
    #[serde(rename = "expansionMin")]
    ExpansionMin(CStyleArray<[u8; 3]>),
    /// Visitor fields
    #[serde(rename = "expansionShift")]
    ExpansionShift(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "max")]
    Max(CStyleArray<[u32; 3]>),
    /// Visitor fields
    #[serde(rename = "expansionMax")]
    ExpansionMax(CStyleArray<[u8; 3]>),
    /// Visitor fields
    #[serde(rename = "padding")]
    Padding(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "numChildShapeAabbs", skip_serializing)]
    NumChildShapeAabbs(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "capacityChildShapeAabbs", skip_serializing)]
    CapacityChildShapeAabbs(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "childShapeAabbs", skip_serializing)]
    ChildShapeAabbs(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "childShapeKeys", skip_serializing)]
    ChildShapeKeys(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpCollidableBoundingVolumeDataVisitor<'de>, "@name",
    ("min" => Min(CStyleArray<[u32; 3]>)),
    ("expansionMin" => ExpansionMin(CStyleArray<[u8; 3]>)),
    ("expansionShift" => ExpansionShift(Primitive<u8>)),
    ("max" => Max(CStyleArray<[u32; 3]>)),
    ("expansionMax" => ExpansionMax(CStyleArray<[u8; 3]>)),
    ("padding" => Padding(Primitive<u8>)),
    ("numChildShapeAabbs" => NumChildShapeAabbs(Primitive<u16>)),
    ("capacityChildShapeAabbs" => CapacityChildShapeAabbs(Primitive<u16>)),
    ("childShapeAabbs" => ChildShapeAabbs(Primitive<Cow<'de, str>>)),
    ("childShapeKeys" => ChildShapeKeys(Primitive<Cow<'de, str>>)),
}
