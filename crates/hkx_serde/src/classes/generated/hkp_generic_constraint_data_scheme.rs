//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpGenericConstraintDataScheme`
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

/// `hkpGenericConstraintDataScheme`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: false
/// - signature: `0x11fd6f6c`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpGenericConstraintDataScheme<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"info"`
    /// -   type: `struct hkpGenericConstraintDataSchemeConstraintInfo`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub info: SingleClass<HkpGenericConstraintDataSchemeConstraintInfo>,
    /// # C++ Class Fields Info
    /// -   name:`"data"`
    /// -   type: `hkArray<hkVector4>`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub data: HkArrayVector<Vector4<f32>>,
    /// # C++ Class Fields Info
    /// -   name:`"commands"`
    /// -   type: `hkArray<hkInt32>`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub commands: HkArrayNum<i32>,
    /// # C++ Class Fields Info
    /// -   name:`"modifiers"`
    /// -   type: `hkArray<void*>`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub modifiers: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"motors"`
    /// -   type: `hkArray<hkpConstraintMotor*>`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub motors: HkArrayRef<Cow<'a, str>>,
}

impl Serialize for HkpGenericConstraintDataScheme<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpGenericConstraintDataSchemeVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpGenericConstraintDataScheme<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpGenericConstraintDataSchemeVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpGenericConstraintDataSchemeVisitor<'a>>> for HkpGenericConstraintDataScheme<'a> {
    fn from(_values: Vec<HkpGenericConstraintDataSchemeVisitor<'a>>) -> Self {
            let mut info = None;
            let mut data = None;
            let mut commands = None;
            let mut modifiers = None;
            let mut motors = None;


        for _value in _values {
            match _value {
                HkpGenericConstraintDataSchemeVisitor::Info(m) => info = Some(m),
                HkpGenericConstraintDataSchemeVisitor::Data(m) => data = Some(m),
                HkpGenericConstraintDataSchemeVisitor::Commands(m) => commands = Some(m),
                HkpGenericConstraintDataSchemeVisitor::Modifiers(m) => modifiers = Some(m),
                HkpGenericConstraintDataSchemeVisitor::Motors(m) => motors = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            info: info.unwrap_or_default(),
            data: data.unwrap_or_default(),
            commands: commands.unwrap_or_default(),
            modifiers: modifiers.unwrap_or_default(),
            motors: motors.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpGenericConstraintDataScheme<'a>> for Vec<HkpGenericConstraintDataSchemeVisitor<'a>> {
    fn from(data: &HkpGenericConstraintDataScheme<'a>) -> Self {
        vec![
            HkpGenericConstraintDataSchemeVisitor::Info(data.info.clone()),
            HkpGenericConstraintDataSchemeVisitor::Data(data.data.clone()),
            HkpGenericConstraintDataSchemeVisitor::Commands(data.commands.clone()),
            HkpGenericConstraintDataSchemeVisitor::Modifiers(data.modifiers.clone()),
            HkpGenericConstraintDataSchemeVisitor::Motors(data.motors.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpGenericConstraintDataScheme<'de> {
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
enum HkpGenericConstraintDataSchemeVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "info", skip_serializing)]
    Info(SingleClass<HkpGenericConstraintDataSchemeConstraintInfo>),
    /// Visitor fields
    #[serde(rename = "data")]
    Data(HkArrayVector<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "commands")]
    Commands(HkArrayNum<i32>),
    /// Visitor fields
    #[serde(rename = "modifiers", skip_serializing)]
    Modifiers(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "motors")]
    Motors(HkArrayRef<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpGenericConstraintDataSchemeVisitor<'de>, "@name",
    ("info" => Info(SingleClass<HkpGenericConstraintDataSchemeConstraintInfo>)),
    ("data" => Data(HkArrayVector<Vector4<f32>>)),
    ("commands" => Commands(HkArrayNum<i32>)),
    ("modifiers" => Modifiers(HkArrayRef<Cow<'de, str>>)),
    ("motors" => Motors(HkArrayRef<Cow<'de, str>>)),
}
