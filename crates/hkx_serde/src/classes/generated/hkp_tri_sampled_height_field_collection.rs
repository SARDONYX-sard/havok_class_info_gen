//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpTriSampledHeightFieldCollection`
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

/// `hkpTriSampledHeightFieldCollection`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: true
/// -    parent: `hkpShapeCollection`/`0xe8c3991d`
/// - signature: `0xc291ddde`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpTriSampledHeightFieldCollection<'a> {
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
    /// -   name:`"heightfield"`
    /// -   type: `struct hkpSampledHeightFieldShape*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub heightfield: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"childSize"`
    /// -   type: `hkInt32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub child_size: i32,
    /// # C++ Class Fields Info
    /// -   name:`"radius"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub radius: f32,
    /// # C++ Class Fields Info
    /// -   name:`"weldingInfo"`
    /// -   type: `hkArray<hkUint16>`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    pub welding_info: HkArrayNum<u16>,
    /// # C++ Class Fields Info
    /// -   name:`"triangleExtrusion"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub triangle_extrusion: Vector4<f32>,
}

impl Serialize for HkpTriSampledHeightFieldCollection<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpTriSampledHeightFieldCollectionVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpTriSampledHeightFieldCollection<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpTriSampledHeightFieldCollectionVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpTriSampledHeightFieldCollectionVisitor<'a>>> for HkpTriSampledHeightFieldCollection<'a> {
    fn from(_values: Vec<HkpTriSampledHeightFieldCollectionVisitor<'a>>) -> Self {
            let mut disable_welding = None;
            let mut collection_type = None;
            let mut user_data = None;
            let mut _type = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut heightfield = None;
            let mut child_size = None;
            let mut radius = None;
            let mut welding_info = None;
            let mut triangle_extrusion = None;


        for _value in _values {
            match _value {
                HkpTriSampledHeightFieldCollectionVisitor::DisableWelding(m) => disable_welding = Some(m),
                HkpTriSampledHeightFieldCollectionVisitor::CollectionType(m) => collection_type = Some(m),
                HkpTriSampledHeightFieldCollectionVisitor::UserData(m) => user_data = Some(m),
                HkpTriSampledHeightFieldCollectionVisitor::Type(m) => _type = Some(m),
                HkpTriSampledHeightFieldCollectionVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpTriSampledHeightFieldCollectionVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpTriSampledHeightFieldCollectionVisitor::Heightfield(m) => heightfield = Some(m),
                HkpTriSampledHeightFieldCollectionVisitor::ChildSize(m) => child_size = Some(m),
                HkpTriSampledHeightFieldCollectionVisitor::Radius(m) => radius = Some(m),
                HkpTriSampledHeightFieldCollectionVisitor::WeldingInfo(m) => welding_info = Some(m),
                HkpTriSampledHeightFieldCollectionVisitor::TriangleExtrusion(m) => triangle_extrusion = Some(m),

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
            heightfield: heightfield.unwrap_or_default().into_inner(),
            child_size: child_size.unwrap_or_default().into_inner(),
            radius: radius.unwrap_or_default().into_inner(),
            welding_info: welding_info.unwrap_or_default(),
            triangle_extrusion: triangle_extrusion.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpTriSampledHeightFieldCollection<'a>> for Vec<HkpTriSampledHeightFieldCollectionVisitor<'a>> {
    fn from(data: &HkpTriSampledHeightFieldCollection<'a>) -> Self {
        vec![
            HkpTriSampledHeightFieldCollectionVisitor::DisableWelding(data.disable_welding.into()),
            HkpTriSampledHeightFieldCollectionVisitor::CollectionType(data.collection_type.clone().into()),
            HkpTriSampledHeightFieldCollectionVisitor::UserData(data.user_data.into()),
            HkpTriSampledHeightFieldCollectionVisitor::Type(data._type.into()),
            HkpTriSampledHeightFieldCollectionVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpTriSampledHeightFieldCollectionVisitor::ReferenceCount(data.reference_count.into()),
            HkpTriSampledHeightFieldCollectionVisitor::Heightfield(data.heightfield.clone().into()),
            HkpTriSampledHeightFieldCollectionVisitor::ChildSize(data.child_size.into()),
            HkpTriSampledHeightFieldCollectionVisitor::Radius(data.radius.into()),
            HkpTriSampledHeightFieldCollectionVisitor::WeldingInfo(data.welding_info.clone()),
            HkpTriSampledHeightFieldCollectionVisitor::TriangleExtrusion(data.triangle_extrusion.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpTriSampledHeightFieldCollection<'de> {
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
enum HkpTriSampledHeightFieldCollectionVisitor<'a> {
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
    #[serde(rename = "heightfield")]
    Heightfield(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "childSize", skip_serializing)]
    ChildSize(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "radius")]
    Radius(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "weldingInfo")]
    WeldingInfo(HkArrayNum<u16>),
    /// Visitor fields
    #[serde(rename = "triangleExtrusion")]
    TriangleExtrusion(Primitive<Vector4<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpTriSampledHeightFieldCollectionVisitor<'de>, "@name",
    ("disableWelding" => DisableWelding(Primitive<bool>)),
    ("collectionType" => CollectionType(Primitive<CollectionType>)),
    ("userData" => UserData(Primitive<usize>)),
    ("type" => Type(Primitive<()>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("heightfield" => Heightfield(Primitive<Cow<'de, str>>)),
    ("childSize" => ChildSize(Primitive<i32>)),
    ("radius" => Radius(Primitive<f32>)),
    ("weldingInfo" => WeldingInfo(HkArrayNum<u16>)),
    ("triangleExtrusion" => TriangleExtrusion(Primitive<Vector4<f32>>)),
}
