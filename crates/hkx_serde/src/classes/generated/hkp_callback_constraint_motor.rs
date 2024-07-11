//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpCallbackConstraintMotor`
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

/// `hkpCallbackConstraintMotor`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 40
/// -    vtable: true
/// -    parent: `hkpLimitedForceConstraintMotor`/`0x3377b0b0`
/// - signature: `0xafcd79ad`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpCallbackConstraintMotor<'a> {
    /// # C++ Parent class(`hkpLimitedForceConstraintMotor` => parent: `hkpConstraintMotor`) field Info
    /// -   name:`"minForce"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub min_force: f32,
    /// # C++ Parent class(`hkpLimitedForceConstraintMotor` => parent: `hkpConstraintMotor`) field Info
    /// -   name:`"maxForce"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub max_force: f32,

    /// # C++ Parent class(`hkpConstraintMotor` => parent: `hkReferencedObject`) field Info
    /// -   name:`"type"`
    /// -   type: `enum MotorType`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub _type: MotorType,

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
    /// -   name:`"callbackFunc"`
    /// -   type: `void*`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub callback_func: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"callbackType"`
    /// -   type: `enum CallbackType`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub callback_type: CallbackType,
    /// # C++ Class Fields Info
    /// -   name:`"userData0"`
    /// -   type: `hkUlong`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub user_data_0: usize,
    /// # C++ Class Fields Info
    /// -   name:`"userData1"`
    /// -   type: `hkUlong`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub user_data_1: usize,
    /// # C++ Class Fields Info
    /// -   name:`"userData2"`
    /// -   type: `hkUlong`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    pub user_data_2: usize,
}

impl Serialize for HkpCallbackConstraintMotor<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpCallbackConstraintMotorVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpCallbackConstraintMotor<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpCallbackConstraintMotorVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpCallbackConstraintMotorVisitor<'a>>> for HkpCallbackConstraintMotor<'a> {
    fn from(_values: Vec<HkpCallbackConstraintMotorVisitor<'a>>) -> Self {
            let mut min_force = None;
            let mut max_force = None;
            let mut _type = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut callback_func = None;
            let mut callback_type = None;
            let mut user_data_0 = None;
            let mut user_data_1 = None;
            let mut user_data_2 = None;


        for _value in _values {
            match _value {
                HkpCallbackConstraintMotorVisitor::MinForce(m) => min_force = Some(m),
                HkpCallbackConstraintMotorVisitor::MaxForce(m) => max_force = Some(m),
                HkpCallbackConstraintMotorVisitor::Type(m) => _type = Some(m),
                HkpCallbackConstraintMotorVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpCallbackConstraintMotorVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpCallbackConstraintMotorVisitor::CallbackFunc(m) => callback_func = Some(m),
                HkpCallbackConstraintMotorVisitor::CallbackType(m) => callback_type = Some(m),
                HkpCallbackConstraintMotorVisitor::UserData0(m) => user_data_0 = Some(m),
                HkpCallbackConstraintMotorVisitor::UserData1(m) => user_data_1 = Some(m),
                HkpCallbackConstraintMotorVisitor::UserData2(m) => user_data_2 = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            min_force: min_force.unwrap_or_default().into_inner(),
            max_force: max_force.unwrap_or_default().into_inner(),
            _type: _type.unwrap_or_default().into_inner(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            callback_func: callback_func.unwrap_or_default().into_inner(),
            callback_type: callback_type.unwrap_or_default().into_inner(),
            user_data_0: user_data_0.unwrap_or_default().into_inner(),
            user_data_1: user_data_1.unwrap_or_default().into_inner(),
            user_data_2: user_data_2.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpCallbackConstraintMotor<'a>> for Vec<HkpCallbackConstraintMotorVisitor<'a>> {
    fn from(data: &HkpCallbackConstraintMotor<'a>) -> Self {
        vec![
            HkpCallbackConstraintMotorVisitor::MinForce(data.min_force.into()),
            HkpCallbackConstraintMotorVisitor::MaxForce(data.max_force.into()),
            HkpCallbackConstraintMotorVisitor::Type(data._type.clone().into()),
            HkpCallbackConstraintMotorVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpCallbackConstraintMotorVisitor::ReferenceCount(data.reference_count.into()),
            HkpCallbackConstraintMotorVisitor::CallbackFunc(data.callback_func.clone().into()),
            HkpCallbackConstraintMotorVisitor::CallbackType(data.callback_type.clone().into()),
            HkpCallbackConstraintMotorVisitor::UserData0(data.user_data_0.into()),
            HkpCallbackConstraintMotorVisitor::UserData1(data.user_data_1.into()),
            HkpCallbackConstraintMotorVisitor::UserData2(data.user_data_2.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpCallbackConstraintMotor<'de> {
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
enum HkpCallbackConstraintMotorVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "minForce")]
    MinForce(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "maxForce")]
    MaxForce(Primitive<f32>),

    /// Visitor fields
    #[serde(rename = "type")]
    Type(Primitive<MotorType>),

    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "callbackFunc", skip_serializing)]
    CallbackFunc(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "callbackType")]
    CallbackType(Primitive<CallbackType>),
    /// Visitor fields
    #[serde(rename = "userData0")]
    UserData0(Primitive<usize>),
    /// Visitor fields
    #[serde(rename = "userData1")]
    UserData1(Primitive<usize>),
    /// Visitor fields
    #[serde(rename = "userData2")]
    UserData2(Primitive<usize>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpCallbackConstraintMotorVisitor<'de>, "@name",
    ("minForce" => MinForce(Primitive<f32>)),
    ("maxForce" => MaxForce(Primitive<f32>)),
    ("type" => Type(Primitive<MotorType>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("callbackFunc" => CallbackFunc(Primitive<Cow<'de, str>>)),
    ("callbackType" => CallbackType(Primitive<CallbackType>)),
    ("userData0" => UserData0(Primitive<usize>)),
    ("userData1" => UserData1(Primitive<usize>)),
    ("userData2" => UserData2(Primitive<usize>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum CallbackType {
    #[serde(rename = "CALLBACK_MOTOR_TYPE_HAVOK_DEMO_SPRING_DAMPER")]
    #[default]
    CallbackMotorTypeHavokDemoSpringDamper = 0,
    #[serde(rename = "CALLBACK_MOTOR_TYPE_USER_0")]
    CallbackMotorTypeUser0 = 1,
    #[serde(rename = "CALLBACK_MOTOR_TYPE_USER_1")]
    CallbackMotorTypeUser1 = 2,
    #[serde(rename = "CALLBACK_MOTOR_TYPE_USER_2")]
    CallbackMotorTypeUser2 = 3,
    #[serde(rename = "CALLBACK_MOTOR_TYPE_USER_3")]
    CallbackMotorTypeUser3 = 4,
}
