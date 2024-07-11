//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpConvexPieceMeshShape`
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

/// `hkpConvexPieceMeshShape`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 36
/// -    vtable: true
/// -    parent: `hkpShapeCollection`/`0xe8c3991d`
/// - signature: `0x38fd3d97`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpConvexPieceMeshShape<'a> {
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
    /// -   name:`"convexPieceStream"`
    /// -   type: `struct hkpConvexPieceStreamData*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub convex_piece_stream: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"displayMesh"`
    /// -   type: `struct hkpShapeCollection*`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub display_mesh: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"radius"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub radius: f32,
}

impl Serialize for HkpConvexPieceMeshShape<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpConvexPieceMeshShapeVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpConvexPieceMeshShape<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpConvexPieceMeshShapeVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpConvexPieceMeshShapeVisitor<'a>>> for HkpConvexPieceMeshShape<'a> {
    fn from(_values: Vec<HkpConvexPieceMeshShapeVisitor<'a>>) -> Self {
            let mut disable_welding = None;
            let mut collection_type = None;
            let mut user_data = None;
            let mut _type = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut convex_piece_stream = None;
            let mut display_mesh = None;
            let mut radius = None;


        for _value in _values {
            match _value {
                HkpConvexPieceMeshShapeVisitor::DisableWelding(m) => disable_welding = Some(m),
                HkpConvexPieceMeshShapeVisitor::CollectionType(m) => collection_type = Some(m),
                HkpConvexPieceMeshShapeVisitor::UserData(m) => user_data = Some(m),
                HkpConvexPieceMeshShapeVisitor::Type(m) => _type = Some(m),
                HkpConvexPieceMeshShapeVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpConvexPieceMeshShapeVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpConvexPieceMeshShapeVisitor::ConvexPieceStream(m) => convex_piece_stream = Some(m),
                HkpConvexPieceMeshShapeVisitor::DisplayMesh(m) => display_mesh = Some(m),
                HkpConvexPieceMeshShapeVisitor::Radius(m) => radius = Some(m),

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
            convex_piece_stream: convex_piece_stream.unwrap_or_default().into_inner(),
            display_mesh: display_mesh.unwrap_or_default().into_inner(),
            radius: radius.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpConvexPieceMeshShape<'a>> for Vec<HkpConvexPieceMeshShapeVisitor<'a>> {
    fn from(data: &HkpConvexPieceMeshShape<'a>) -> Self {
        vec![
            HkpConvexPieceMeshShapeVisitor::DisableWelding(data.disable_welding.into()),
            HkpConvexPieceMeshShapeVisitor::CollectionType(data.collection_type.clone().into()),
            HkpConvexPieceMeshShapeVisitor::UserData(data.user_data.into()),
            HkpConvexPieceMeshShapeVisitor::Type(data._type.into()),
            HkpConvexPieceMeshShapeVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpConvexPieceMeshShapeVisitor::ReferenceCount(data.reference_count.into()),
            HkpConvexPieceMeshShapeVisitor::ConvexPieceStream(data.convex_piece_stream.clone().into()),
            HkpConvexPieceMeshShapeVisitor::DisplayMesh(data.display_mesh.clone().into()),
            HkpConvexPieceMeshShapeVisitor::Radius(data.radius.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpConvexPieceMeshShape<'de> {
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
enum HkpConvexPieceMeshShapeVisitor<'a> {
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
    #[serde(rename = "convexPieceStream")]
    ConvexPieceStream(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "displayMesh")]
    DisplayMesh(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "radius")]
    Radius(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpConvexPieceMeshShapeVisitor<'de>, "@name",
    ("disableWelding" => DisableWelding(Primitive<bool>)),
    ("collectionType" => CollectionType(Primitive<CollectionType>)),
    ("userData" => UserData(Primitive<usize>)),
    ("type" => Type(Primitive<()>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("convexPieceStream" => ConvexPieceStream(Primitive<Cow<'de, str>>)),
    ("displayMesh" => DisplayMesh(Primitive<Cow<'de, str>>)),
    ("radius" => Radius(Primitive<f32>)),
}
