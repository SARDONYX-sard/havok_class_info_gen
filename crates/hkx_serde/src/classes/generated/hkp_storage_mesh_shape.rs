//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpStorageMeshShape`
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

/// `hkpStorageMeshShape`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: true
/// -    parent: `hkpMeshShape`/`0x3bf12c0f`
/// - signature: `0xbefd8b39`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpStorageMeshShape<'a> {
    /// # C++ Parent class(`hkpMeshShape` => parent: `hkpShapeCollection`) field Info
    /// -   name:`"scaling"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub scaling: Vector4<f32>,
    /// # C++ Parent class(`hkpMeshShape` => parent: `hkpShapeCollection`) field Info
    /// -   name:`"numBitsForSubpartIndex"`
    /// -   type: `hkInt32`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub num_bits_for_subpart_index: i32,
    /// # C++ Parent class(`hkpMeshShape` => parent: `hkpShapeCollection`) field Info
    /// -   name:`"subparts"`
    /// -   type: `hkArray<struct hkpMeshShapeSubpart>`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub subparts: HkArrayClass<HkpMeshShapeSubpart<'a>>,
    /// # C++ Parent class(`hkpMeshShape` => parent: `hkpShapeCollection`) field Info
    /// -   name:`"weldingInfo"`
    /// -   type: `hkArray<hkUint16>`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub welding_info: HkArrayNum<u16>,
    /// # C++ Parent class(`hkpMeshShape` => parent: `hkpShapeCollection`) field Info
    /// -   name:`"weldingType"`
    /// -   type: `enum WeldingType`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    pub welding_type: WeldingType,
    /// # C++ Parent class(`hkpMeshShape` => parent: `hkpShapeCollection`) field Info
    /// -   name:`"radius"`
    /// -   type: `hkReal`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub radius: f32,
    /// # C++ Parent class(`hkpMeshShape` => parent: `hkpShapeCollection`) field Info
    /// -   name:`"pad"`
    /// -   type: `hkInt32[3]`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    pub pad: CStyleArray<[i32; 3]>,

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
    /// -   name:`"storage"`
    /// -   type: `hkArray<hkpStorageMeshShapeSubpartStorage*>`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    pub storage: HkArrayRef<Cow<'a, str>>,
}

impl Serialize for HkpStorageMeshShape<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpStorageMeshShapeVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpStorageMeshShape<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpStorageMeshShapeVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpStorageMeshShapeVisitor<'a>>> for HkpStorageMeshShape<'a> {
    fn from(_values: Vec<HkpStorageMeshShapeVisitor<'a>>) -> Self {
            let mut scaling = None;
            let mut num_bits_for_subpart_index = None;
            let mut subparts = None;
            let mut welding_info = None;
            let mut welding_type = None;
            let mut radius = None;
            let mut pad = None;
            let mut disable_welding = None;
            let mut collection_type = None;
            let mut user_data = None;
            let mut _type = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut storage = None;


        for _value in _values {
            match _value {
                HkpStorageMeshShapeVisitor::Scaling(m) => scaling = Some(m),
                HkpStorageMeshShapeVisitor::NumBitsForSubpartIndex(m) => num_bits_for_subpart_index = Some(m),
                HkpStorageMeshShapeVisitor::Subparts(m) => subparts = Some(m),
                HkpStorageMeshShapeVisitor::WeldingInfo(m) => welding_info = Some(m),
                HkpStorageMeshShapeVisitor::WeldingType(m) => welding_type = Some(m),
                HkpStorageMeshShapeVisitor::Radius(m) => radius = Some(m),
                HkpStorageMeshShapeVisitor::Pad(m) => pad = Some(m),
                HkpStorageMeshShapeVisitor::DisableWelding(m) => disable_welding = Some(m),
                HkpStorageMeshShapeVisitor::CollectionType(m) => collection_type = Some(m),
                HkpStorageMeshShapeVisitor::UserData(m) => user_data = Some(m),
                HkpStorageMeshShapeVisitor::Type(m) => _type = Some(m),
                HkpStorageMeshShapeVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpStorageMeshShapeVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpStorageMeshShapeVisitor::Storage(m) => storage = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            scaling: scaling.unwrap_or_default().into_inner(),
            num_bits_for_subpart_index: num_bits_for_subpart_index.unwrap_or_default().into_inner(),
            subparts: subparts.unwrap_or_default(),
            welding_info: welding_info.unwrap_or_default(),
            welding_type: welding_type.unwrap_or_default().into_inner(),
            radius: radius.unwrap_or_default().into_inner(),
            pad: pad.unwrap_or_default(),
            disable_welding: disable_welding.unwrap_or_default().into_inner(),
            collection_type: collection_type.unwrap_or_default().into_inner(),
            user_data: user_data.unwrap_or_default().into_inner(),
            _type: _type.unwrap_or_default().into_inner(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            storage: storage.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpStorageMeshShape<'a>> for Vec<HkpStorageMeshShapeVisitor<'a>> {
    fn from(data: &HkpStorageMeshShape<'a>) -> Self {
        vec![
            HkpStorageMeshShapeVisitor::Scaling(data.scaling.into()),
            HkpStorageMeshShapeVisitor::NumBitsForSubpartIndex(data.num_bits_for_subpart_index.into()),
            HkpStorageMeshShapeVisitor::Subparts(data.subparts.clone()),
            HkpStorageMeshShapeVisitor::WeldingInfo(data.welding_info.clone()),
            HkpStorageMeshShapeVisitor::WeldingType(data.welding_type.clone().into()),
            HkpStorageMeshShapeVisitor::Radius(data.radius.into()),
            HkpStorageMeshShapeVisitor::Pad(data.pad.clone()),
            HkpStorageMeshShapeVisitor::DisableWelding(data.disable_welding.into()),
            HkpStorageMeshShapeVisitor::CollectionType(data.collection_type.clone().into()),
            HkpStorageMeshShapeVisitor::UserData(data.user_data.into()),
            HkpStorageMeshShapeVisitor::Type(data._type.into()),
            HkpStorageMeshShapeVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpStorageMeshShapeVisitor::ReferenceCount(data.reference_count.into()),
            HkpStorageMeshShapeVisitor::Storage(data.storage.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpStorageMeshShape<'de> {
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
enum HkpStorageMeshShapeVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "scaling")]
    Scaling(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "numBitsForSubpartIndex")]
    NumBitsForSubpartIndex(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "subparts")]
    Subparts(HkArrayClass<HkpMeshShapeSubpart<'a>>),
    /// Visitor fields
    #[serde(rename = "weldingInfo")]
    WeldingInfo(HkArrayNum<u16>),
    /// Visitor fields
    #[serde(rename = "weldingType")]
    WeldingType(Primitive<WeldingType>),
    /// Visitor fields
    #[serde(rename = "radius")]
    Radius(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "pad")]
    Pad(CStyleArray<[i32; 3]>),

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
    #[serde(rename = "storage")]
    Storage(HkArrayRef<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpStorageMeshShapeVisitor<'de>, "@name",
    ("scaling" => Scaling(Primitive<Vector4<f32>>)),
    ("numBitsForSubpartIndex" => NumBitsForSubpartIndex(Primitive<i32>)),
    ("subparts" => Subparts(HkArrayClass<HkpMeshShapeSubpart<'de>>)),
    ("weldingInfo" => WeldingInfo(HkArrayNum<u16>)),
    ("weldingType" => WeldingType(Primitive<WeldingType>)),
    ("radius" => Radius(Primitive<f32>)),
    ("pad" => Pad(CStyleArray<[i32; 3]>)),
    ("disableWelding" => DisableWelding(Primitive<bool>)),
    ("collectionType" => CollectionType(Primitive<CollectionType>)),
    ("userData" => UserData(Primitive<usize>)),
    ("type" => Type(Primitive<()>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("storage" => Storage(HkArrayRef<Cow<'de, str>>)),
}
