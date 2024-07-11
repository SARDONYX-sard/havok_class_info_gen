//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpPairCollisionFilterMapPairFilterKeyOverrideType`
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

/// `hkpPairCollisionFilterMapPairFilterKeyOverrideType`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// - signature: `0x36195969`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpPairCollisionFilterMapPairFilterKeyOverrideType<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"elem"`
    /// -   type: `void*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub elem: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"numElems"`
    /// -   type: `hkInt32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub num_elems: i32,
    /// # C++ Class Fields Info
    /// -   name:`"hashMod"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub hash_mod: i32,
}

impl Serialize for HkpPairCollisionFilterMapPairFilterKeyOverrideType<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpPairCollisionFilterMapPairFilterKeyOverrideTypeVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpPairCollisionFilterMapPairFilterKeyOverrideType<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpPairCollisionFilterMapPairFilterKeyOverrideTypeVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpPairCollisionFilterMapPairFilterKeyOverrideTypeVisitor<'a>>> for HkpPairCollisionFilterMapPairFilterKeyOverrideType<'a> {
    fn from(_values: Vec<HkpPairCollisionFilterMapPairFilterKeyOverrideTypeVisitor<'a>>) -> Self {
            let mut elem = None;
            let mut num_elems = None;
            let mut hash_mod = None;


        for _value in _values {
            match _value {
                HkpPairCollisionFilterMapPairFilterKeyOverrideTypeVisitor::Elem(m) => elem = Some(m),
                HkpPairCollisionFilterMapPairFilterKeyOverrideTypeVisitor::NumElems(m) => num_elems = Some(m),
                HkpPairCollisionFilterMapPairFilterKeyOverrideTypeVisitor::HashMod(m) => hash_mod = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            elem: elem.unwrap_or_default().into_inner(),
            num_elems: num_elems.unwrap_or_default().into_inner(),
            hash_mod: hash_mod.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpPairCollisionFilterMapPairFilterKeyOverrideType<'a>> for Vec<HkpPairCollisionFilterMapPairFilterKeyOverrideTypeVisitor<'a>> {
    fn from(data: &HkpPairCollisionFilterMapPairFilterKeyOverrideType<'a>) -> Self {
        vec![
            HkpPairCollisionFilterMapPairFilterKeyOverrideTypeVisitor::Elem(data.elem.clone().into()),
            HkpPairCollisionFilterMapPairFilterKeyOverrideTypeVisitor::NumElems(data.num_elems.into()),
            HkpPairCollisionFilterMapPairFilterKeyOverrideTypeVisitor::HashMod(data.hash_mod.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpPairCollisionFilterMapPairFilterKeyOverrideType<'de> {
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
enum HkpPairCollisionFilterMapPairFilterKeyOverrideTypeVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "elem", skip_serializing)]
    Elem(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "numElems")]
    NumElems(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "hashMod")]
    HashMod(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpPairCollisionFilterMapPairFilterKeyOverrideTypeVisitor<'de>, "@name",
    ("elem" => Elem(Primitive<Cow<'de, str>>)),
    ("numElems" => NumElems(Primitive<i32>)),
    ("hashMod" => HashMod(Primitive<i32>)),
}
