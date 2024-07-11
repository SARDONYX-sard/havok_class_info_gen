//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkaAnnotationTrackAnnotation`
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

/// `hkaAnnotationTrackAnnotation`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: false
/// - signature: `0x623bf34f`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkaAnnotationTrackAnnotation<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"time"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub time: f32,
    /// # C++ Class Fields Info
    /// -   name:`"text"`
    /// -   type: `hkStringPtr`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub text: Cow<'a, str>,
}

impl Serialize for HkaAnnotationTrackAnnotation<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkaAnnotationTrackAnnotationVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkaAnnotationTrackAnnotation<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkaAnnotationTrackAnnotationVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkaAnnotationTrackAnnotationVisitor<'a>>> for HkaAnnotationTrackAnnotation<'a> {
    fn from(_values: Vec<HkaAnnotationTrackAnnotationVisitor<'a>>) -> Self {
            let mut time = None;
            let mut text = None;


        for _value in _values {
            match _value {
                HkaAnnotationTrackAnnotationVisitor::Time(m) => time = Some(m),
                HkaAnnotationTrackAnnotationVisitor::Text(m) => text = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            time: time.unwrap_or_default().into_inner(),
            text: text.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkaAnnotationTrackAnnotation<'a>> for Vec<HkaAnnotationTrackAnnotationVisitor<'a>> {
    fn from(data: &HkaAnnotationTrackAnnotation<'a>) -> Self {
        vec![
            HkaAnnotationTrackAnnotationVisitor::Time(data.time.into()),
            HkaAnnotationTrackAnnotationVisitor::Text(data.text.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkaAnnotationTrackAnnotation<'de> {
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
enum HkaAnnotationTrackAnnotationVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "time")]
    Time(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "text")]
    Text(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaAnnotationTrackAnnotationVisitor<'de>, "@name",
    ("time" => Time(Primitive<f32>)),
    ("text" => Text(Primitive<Cow<'de, str>>)),
}
