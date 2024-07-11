//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpSerializedDisplayRbTransformsDisplayTransformPair`
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

/// `hkpSerializedDisplayRbTransformsDisplayTransformPair`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 80
/// -    vtable: false
/// - signature: `0x94ac5bec`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpSerializedDisplayRbTransformsDisplayTransformPair<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"rb"`
    /// -   type: `struct hkpRigidBody*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub rb: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"localToDisplay"`
    /// -   type: `hkTransform`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub local_to_display: Transform<f32>,
}

impl Serialize for HkpSerializedDisplayRbTransformsDisplayTransformPair<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpSerializedDisplayRbTransformsDisplayTransformPairVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpSerializedDisplayRbTransformsDisplayTransformPair<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpSerializedDisplayRbTransformsDisplayTransformPairVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpSerializedDisplayRbTransformsDisplayTransformPairVisitor<'a>>> for HkpSerializedDisplayRbTransformsDisplayTransformPair<'a> {
    fn from(_values: Vec<HkpSerializedDisplayRbTransformsDisplayTransformPairVisitor<'a>>) -> Self {
            let mut rb = None;
            let mut local_to_display = None;


        for _value in _values {
            match _value {
                HkpSerializedDisplayRbTransformsDisplayTransformPairVisitor::Rb(m) => rb = Some(m),
                HkpSerializedDisplayRbTransformsDisplayTransformPairVisitor::LocalToDisplay(m) => local_to_display = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            rb: rb.unwrap_or_default().into_inner(),
            local_to_display: local_to_display.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpSerializedDisplayRbTransformsDisplayTransformPair<'a>> for Vec<HkpSerializedDisplayRbTransformsDisplayTransformPairVisitor<'a>> {
    fn from(data: &HkpSerializedDisplayRbTransformsDisplayTransformPair<'a>) -> Self {
        vec![
            HkpSerializedDisplayRbTransformsDisplayTransformPairVisitor::Rb(data.rb.clone().into()),
            HkpSerializedDisplayRbTransformsDisplayTransformPairVisitor::LocalToDisplay(data.local_to_display.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpSerializedDisplayRbTransformsDisplayTransformPair<'de> {
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
enum HkpSerializedDisplayRbTransformsDisplayTransformPairVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "rb")]
    Rb(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "localToDisplay")]
    LocalToDisplay(Primitive<Transform<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpSerializedDisplayRbTransformsDisplayTransformPairVisitor<'de>, "@name",
    ("rb" => Rb(Primitive<Cow<'de, str>>)),
    ("localToDisplay" => LocalToDisplay(Primitive<Transform<f32>>)),
}
