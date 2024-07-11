//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpEntitySpuCollisionCallback`
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

/// `hkpEntitySpuCollisionCallback`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: false
/// - signature: `0x81147f05`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpEntitySpuCollisionCallback<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"util"`
    /// -   type: `void*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub util: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"capacity"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub capacity: u16,
    /// # C++ Class Fields Info
    /// -   name:`"eventFilter"`
    /// -   type: `hkUint8`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE`
    pub event_filter: u8,
    /// # C++ Class Fields Info
    /// -   name:`"userFilter"`
    /// -   type: `hkUint8`
    /// - offset: 7
    /// -  flags: `FLAGS_NONE`
    pub user_filter: u8,
}

impl Serialize for HkpEntitySpuCollisionCallback<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpEntitySpuCollisionCallbackVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpEntitySpuCollisionCallback<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpEntitySpuCollisionCallbackVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpEntitySpuCollisionCallbackVisitor<'a>>> for HkpEntitySpuCollisionCallback<'a> {
    fn from(_values: Vec<HkpEntitySpuCollisionCallbackVisitor<'a>>) -> Self {
            let mut util = None;
            let mut capacity = None;
            let mut event_filter = None;
            let mut user_filter = None;


        for _value in _values {
            match _value {
                HkpEntitySpuCollisionCallbackVisitor::Util(m) => util = Some(m),
                HkpEntitySpuCollisionCallbackVisitor::Capacity(m) => capacity = Some(m),
                HkpEntitySpuCollisionCallbackVisitor::EventFilter(m) => event_filter = Some(m),
                HkpEntitySpuCollisionCallbackVisitor::UserFilter(m) => user_filter = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            util: util.unwrap_or_default().into_inner(),
            capacity: capacity.unwrap_or_default().into_inner(),
            event_filter: event_filter.unwrap_or_default().into_inner(),
            user_filter: user_filter.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpEntitySpuCollisionCallback<'a>> for Vec<HkpEntitySpuCollisionCallbackVisitor<'a>> {
    fn from(data: &HkpEntitySpuCollisionCallback<'a>) -> Self {
        vec![
            HkpEntitySpuCollisionCallbackVisitor::Util(data.util.clone().into()),
            HkpEntitySpuCollisionCallbackVisitor::Capacity(data.capacity.into()),
            HkpEntitySpuCollisionCallbackVisitor::EventFilter(data.event_filter.into()),
            HkpEntitySpuCollisionCallbackVisitor::UserFilter(data.user_filter.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpEntitySpuCollisionCallback<'de> {
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
enum HkpEntitySpuCollisionCallbackVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "util", skip_serializing)]
    Util(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "capacity", skip_serializing)]
    Capacity(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "eventFilter")]
    EventFilter(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "userFilter")]
    UserFilter(Primitive<u8>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpEntitySpuCollisionCallbackVisitor<'de>, "@name",
    ("util" => Util(Primitive<Cow<'de, str>>)),
    ("capacity" => Capacity(Primitive<u16>)),
    ("eventFilter" => EventFilter(Primitive<u8>)),
    ("userFilter" => UserFilter(Primitive<u8>)),
}
