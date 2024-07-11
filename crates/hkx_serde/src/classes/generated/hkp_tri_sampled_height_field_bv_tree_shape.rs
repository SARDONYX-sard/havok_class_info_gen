//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpTriSampledHeightFieldBvTreeShape`
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

/// `hkpTriSampledHeightFieldBvTreeShape`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: true
/// -    parent: `hkpBvTreeShape`/`0xa823d623`
/// - signature: `0x58e1e585`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpTriSampledHeightFieldBvTreeShape<'a> {
    /// # C++ Parent class(`hkpBvTreeShape` => parent: `hkpShape`) field Info
    /// -   name:`"bvTreeType"`
    /// -   type: `enum BvTreeType`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub bv_tree_type: BvTreeType,

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
    /// -   name:`"childContainer"`
    /// -   type: `struct hkpSingleShapeContainer`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub child_container: SingleClass<HkpSingleShapeContainer<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"childSize"`
    /// -   type: `hkInt32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub child_size: i32,
    /// # C++ Class Fields Info
    /// -   name:`"wantAabbRejectionTest"`
    /// -   type: `hkBool`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub want_aabb_rejection_test: bool,
    /// # C++ Class Fields Info
    /// -   name:`"padding"`
    /// -   type: `hkUint8[12]`
    /// - offset: 33
    /// -  flags: `FLAGS_NONE`
    pub padding: CStyleArray<[u8; 12]>,
}

impl Serialize for HkpTriSampledHeightFieldBvTreeShape<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpTriSampledHeightFieldBvTreeShapeVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpTriSampledHeightFieldBvTreeShape<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpTriSampledHeightFieldBvTreeShapeVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpTriSampledHeightFieldBvTreeShapeVisitor<'a>>> for HkpTriSampledHeightFieldBvTreeShape<'a> {
    fn from(_values: Vec<HkpTriSampledHeightFieldBvTreeShapeVisitor<'a>>) -> Self {
            let mut bv_tree_type = None;
            let mut user_data = None;
            let mut _type = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut child_container = None;
            let mut child_size = None;
            let mut want_aabb_rejection_test = None;
            let mut padding = None;


        for _value in _values {
            match _value {
                HkpTriSampledHeightFieldBvTreeShapeVisitor::BvTreeType(m) => bv_tree_type = Some(m),
                HkpTriSampledHeightFieldBvTreeShapeVisitor::UserData(m) => user_data = Some(m),
                HkpTriSampledHeightFieldBvTreeShapeVisitor::Type(m) => _type = Some(m),
                HkpTriSampledHeightFieldBvTreeShapeVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpTriSampledHeightFieldBvTreeShapeVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpTriSampledHeightFieldBvTreeShapeVisitor::ChildContainer(m) => child_container = Some(m),
                HkpTriSampledHeightFieldBvTreeShapeVisitor::ChildSize(m) => child_size = Some(m),
                HkpTriSampledHeightFieldBvTreeShapeVisitor::WantAabbRejectionTest(m) => want_aabb_rejection_test = Some(m),
                HkpTriSampledHeightFieldBvTreeShapeVisitor::Padding(m) => padding = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            bv_tree_type: bv_tree_type.unwrap_or_default().into_inner(),
            user_data: user_data.unwrap_or_default().into_inner(),
            _type: _type.unwrap_or_default().into_inner(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            child_container: child_container.unwrap_or_default(),
            child_size: child_size.unwrap_or_default().into_inner(),
            want_aabb_rejection_test: want_aabb_rejection_test.unwrap_or_default().into_inner(),
            padding: padding.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpTriSampledHeightFieldBvTreeShape<'a>> for Vec<HkpTriSampledHeightFieldBvTreeShapeVisitor<'a>> {
    fn from(data: &HkpTriSampledHeightFieldBvTreeShape<'a>) -> Self {
        vec![
            HkpTriSampledHeightFieldBvTreeShapeVisitor::BvTreeType(data.bv_tree_type.clone().into()),
            HkpTriSampledHeightFieldBvTreeShapeVisitor::UserData(data.user_data.into()),
            HkpTriSampledHeightFieldBvTreeShapeVisitor::Type(data._type.into()),
            HkpTriSampledHeightFieldBvTreeShapeVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpTriSampledHeightFieldBvTreeShapeVisitor::ReferenceCount(data.reference_count.into()),
            HkpTriSampledHeightFieldBvTreeShapeVisitor::ChildContainer(data.child_container.clone()),
            HkpTriSampledHeightFieldBvTreeShapeVisitor::ChildSize(data.child_size.into()),
            HkpTriSampledHeightFieldBvTreeShapeVisitor::WantAabbRejectionTest(data.want_aabb_rejection_test.into()),
            HkpTriSampledHeightFieldBvTreeShapeVisitor::Padding(data.padding.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpTriSampledHeightFieldBvTreeShape<'de> {
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
enum HkpTriSampledHeightFieldBvTreeShapeVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "bvTreeType")]
    BvTreeType(Primitive<BvTreeType>),

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
    #[serde(rename = "childContainer")]
    ChildContainer(SingleClass<HkpSingleShapeContainer<'a>>),
    /// Visitor fields
    #[serde(rename = "childSize", skip_serializing)]
    ChildSize(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "wantAabbRejectionTest")]
    WantAabbRejectionTest(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "padding")]
    Padding(CStyleArray<[u8; 12]>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpTriSampledHeightFieldBvTreeShapeVisitor<'de>, "@name",
    ("bvTreeType" => BvTreeType(Primitive<BvTreeType>)),
    ("userData" => UserData(Primitive<usize>)),
    ("type" => Type(Primitive<()>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("childContainer" => ChildContainer(SingleClass<HkpSingleShapeContainer<'de>>)),
    ("childSize" => ChildSize(Primitive<i32>)),
    ("wantAabbRejectionTest" => WantAabbRejectionTest(Primitive<bool>)),
    ("padding" => Padding(CStyleArray<[u8; 12]>)),
}
