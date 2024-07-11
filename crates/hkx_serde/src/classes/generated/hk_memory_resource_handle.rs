//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkMemoryResourceHandle`
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

/// `hkMemoryResourceHandle`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 28
/// -    vtable: true
/// -    parent: `hkResourceHandle`/`0x4e94146`
/// - signature: `0xbffac086`
/// -   version: 2
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkMemoryResourceHandle<'a> {
    // C++ Parent class(`hkResourceHandle` => parent: `hkResourceBase`) has no fields
    //
    // C++ Parent class(`hkResourceBase` => parent: `hkReferencedObject`) has no fields
    //
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
    /// -   name:`"variant"`
    /// -   type: `struct hkReferencedObject*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub variant: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub name: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"references"`
    /// -   type: `hkArray<struct hkMemoryResourceHandleExternalLink>`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub references: HkArrayClass<HkMemoryResourceHandleExternalLink<'a>>,
}

impl Serialize for HkMemoryResourceHandle<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkMemoryResourceHandleVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkMemoryResourceHandle<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkMemoryResourceHandleVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkMemoryResourceHandleVisitor<'a>>> for HkMemoryResourceHandle<'a> {
    fn from(_values: Vec<HkMemoryResourceHandleVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut variant = None;
            let mut name = None;
            let mut references = None;


        for _value in _values {
            match _value {
                HkMemoryResourceHandleVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkMemoryResourceHandleVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkMemoryResourceHandleVisitor::Variant(m) => variant = Some(m),
                HkMemoryResourceHandleVisitor::Name(m) => name = Some(m),
                HkMemoryResourceHandleVisitor::References(m) => references = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            variant: variant.unwrap_or_default().into_inner(),
            name: name.unwrap_or_default().into_inner(),
            references: references.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkMemoryResourceHandle<'a>> for Vec<HkMemoryResourceHandleVisitor<'a>> {
    fn from(data: &HkMemoryResourceHandle<'a>) -> Self {
        vec![
            HkMemoryResourceHandleVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkMemoryResourceHandleVisitor::ReferenceCount(data.reference_count.into()),
            HkMemoryResourceHandleVisitor::Variant(data.variant.clone().into()),
            HkMemoryResourceHandleVisitor::Name(data.name.clone().into()),
            HkMemoryResourceHandleVisitor::References(data.references.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkMemoryResourceHandle<'de> {
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
enum HkMemoryResourceHandleVisitor<'a> {
    // C++ Parent class(`hkResourceHandle` => parent: `hkResourceBase`) has no fields
    //
    // C++ Parent class(`hkResourceBase` => parent: `hkReferencedObject`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "variant")]
    Variant(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "references")]
    References(HkArrayClass<HkMemoryResourceHandleExternalLink<'a>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMemoryResourceHandleVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("variant" => Variant(Primitive<Cow<'de, str>>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("references" => References(HkArrayClass<HkMemoryResourceHandleExternalLink<'de>>)),
}
