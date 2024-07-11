//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkaAnimationContainer`
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

/// `hkaAnimationContainer`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 68
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x8dc20333`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkaAnimationContainer<'a> {
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
    /// -   name:`"skeletons"`
    /// -   type: `hkArray<hkaSkeleton*>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub skeletons: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"animations"`
    /// -   type: `hkArray<hkaAnimation*>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub animations: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"bindings"`
    /// -   type: `hkArray<hkaAnimationBinding*>`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub bindings: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"attachments"`
    /// -   type: `hkArray<hkaBoneAttachment*>`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub attachments: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"skins"`
    /// -   type: `hkArray<hkaMeshBinding*>`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    pub skins: HkArrayRef<Cow<'a, str>>,
}

impl Serialize for HkaAnimationContainer<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkaAnimationContainerVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkaAnimationContainer<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkaAnimationContainerVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkaAnimationContainerVisitor<'a>>> for HkaAnimationContainer<'a> {
    fn from(_values: Vec<HkaAnimationContainerVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut skeletons = None;
            let mut animations = None;
            let mut bindings = None;
            let mut attachments = None;
            let mut skins = None;


        for _value in _values {
            match _value {
                HkaAnimationContainerVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkaAnimationContainerVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkaAnimationContainerVisitor::Skeletons(m) => skeletons = Some(m),
                HkaAnimationContainerVisitor::Animations(m) => animations = Some(m),
                HkaAnimationContainerVisitor::Bindings(m) => bindings = Some(m),
                HkaAnimationContainerVisitor::Attachments(m) => attachments = Some(m),
                HkaAnimationContainerVisitor::Skins(m) => skins = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            skeletons: skeletons.unwrap_or_default(),
            animations: animations.unwrap_or_default(),
            bindings: bindings.unwrap_or_default(),
            attachments: attachments.unwrap_or_default(),
            skins: skins.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkaAnimationContainer<'a>> for Vec<HkaAnimationContainerVisitor<'a>> {
    fn from(data: &HkaAnimationContainer<'a>) -> Self {
        vec![
            HkaAnimationContainerVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkaAnimationContainerVisitor::ReferenceCount(data.reference_count.into()),
            HkaAnimationContainerVisitor::Skeletons(data.skeletons.clone()),
            HkaAnimationContainerVisitor::Animations(data.animations.clone()),
            HkaAnimationContainerVisitor::Bindings(data.bindings.clone()),
            HkaAnimationContainerVisitor::Attachments(data.attachments.clone()),
            HkaAnimationContainerVisitor::Skins(data.skins.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkaAnimationContainer<'de> {
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
enum HkaAnimationContainerVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "skeletons")]
    Skeletons(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "animations")]
    Animations(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "bindings")]
    Bindings(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "attachments")]
    Attachments(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "skins")]
    Skins(HkArrayRef<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaAnimationContainerVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("skeletons" => Skeletons(HkArrayRef<Cow<'de, str>>)),
    ("animations" => Animations(HkArrayRef<Cow<'de, str>>)),
    ("bindings" => Bindings(HkArrayRef<Cow<'de, str>>)),
    ("attachments" => Attachments(HkArrayRef<Cow<'de, str>>)),
    ("skins" => Skins(HkArrayRef<Cow<'de, str>>)),
}
