//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpConstraintAtom`
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

/// `hkpConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 2
/// -    vtable: false
/// - signature: `0x59d67ef6`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpConstraintAtom {
    /// # C++ Class Fields Info
    /// -   name:`"type"`
    /// -   type: `enum AtomType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub _type: AtomType,
}

impl Serialize for HkpConstraintAtom {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpConstraintAtomVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpConstraintAtom {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpConstraintAtomVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpConstraintAtomVisitor>> for HkpConstraintAtom {
    fn from(_values: Vec<HkpConstraintAtomVisitor>) -> Self {
            let mut _type = None;


        for _value in _values {
            match _value {
                HkpConstraintAtomVisitor::Type(m) => _type = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            _type: _type.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpConstraintAtom> for Vec<HkpConstraintAtomVisitor> {
    fn from(data: &HkpConstraintAtom) -> Self {
        vec![
            HkpConstraintAtomVisitor::Type(data._type.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpConstraintAtom {
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
enum HkpConstraintAtomVisitor {
    /// Visitor fields
    #[serde(rename = "type")]
    Type(Primitive<AtomType>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpConstraintAtomVisitor, "@name",
    ("type" => Type(Primitive<AtomType>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum AtomType {
    #[serde(rename = "TYPE_INVALID")]
    #[default]
    TypeInvalid = 0,
    #[serde(rename = "TYPE_BRIDGE")]
    TypeBridge = 1,
    #[serde(rename = "TYPE_SET_LOCAL_TRANSFORMS")]
    TypeSetLocalTransforms = 2,
    #[serde(rename = "TYPE_SET_LOCAL_TRANSLATIONS")]
    TypeSetLocalTranslations = 3,
    #[serde(rename = "TYPE_SET_LOCAL_ROTATIONS")]
    TypeSetLocalRotations = 4,
    #[serde(rename = "TYPE_BALL_SOCKET")]
    TypeBallSocket = 5,
    #[serde(rename = "TYPE_STIFF_SPRING")]
    TypeStiffSpring = 6,
    #[serde(rename = "TYPE_LIN")]
    TypeLin = 7,
    #[serde(rename = "TYPE_LIN_SOFT")]
    TypeLinSoft = 8,
    #[serde(rename = "TYPE_LIN_LIMIT")]
    TypeLinLimit = 9,
    #[serde(rename = "TYPE_LIN_FRICTION")]
    TypeLinFriction = 10,
    #[serde(rename = "TYPE_LIN_MOTOR")]
    TypeLinMotor = 11,
    #[serde(rename = "TYPE_2D_ANG")]
    Type2DAng = 12,
    #[serde(rename = "TYPE_ANG")]
    TypeAng = 13,
    #[serde(rename = "TYPE_ANG_LIMIT")]
    TypeAngLimit = 14,
    #[serde(rename = "TYPE_TWIST_LIMIT")]
    TypeTwistLimit = 15,
    #[serde(rename = "TYPE_CONE_LIMIT")]
    TypeConeLimit = 16,
    #[serde(rename = "TYPE_ANG_FRICTION")]
    TypeAngFriction = 17,
    #[serde(rename = "TYPE_ANG_MOTOR")]
    TypeAngMotor = 18,
    #[serde(rename = "TYPE_RAGDOLL_MOTOR")]
    TypeRagdollMotor = 19,
    #[serde(rename = "TYPE_PULLEY")]
    TypePulley = 20,
    #[serde(rename = "TYPE_RACK_AND_PINION")]
    TypeRackAndPinion = 21,
    #[serde(rename = "TYPE_COG_WHEEL")]
    TypeCogWheel = 22,
    #[serde(rename = "TYPE_SETUP_STABILIZATION")]
    TypeSetupStabilization = 23,
    #[serde(rename = "TYPE_OVERWRITE_PIVOT")]
    TypeOverwritePivot = 24,
    #[serde(rename = "TYPE_CONTACT")]
    TypeContact = 25,
    #[serde(rename = "TYPE_MODIFIER_SOFT_CONTACT")]
    TypeModifierSoftContact = 26,
    #[serde(rename = "TYPE_MODIFIER_MASS_CHANGER")]
    TypeModifierMassChanger = 27,
    #[serde(rename = "TYPE_MODIFIER_VISCOUS_SURFACE")]
    TypeModifierViscousSurface = 28,
    #[serde(rename = "TYPE_MODIFIER_MOVING_SURFACE")]
    TypeModifierMovingSurface = 29,
    #[serde(rename = "TYPE_MODIFIER_IGNORE_CONSTRAINT")]
    TypeModifierIgnoreConstraint = 30,
    #[serde(rename = "TYPE_MODIFIER_CENTER_OF_MASS_CHANGER")]
    TypeModifierCenterOfMassChanger = 31,
    #[serde(rename = "TYPE_MAX")]
    TypeMax = 32,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum CallbackRequest {
    #[serde(rename = "CALLBACK_REQUEST_NONE")]
    #[default]
    CallbackRequestNone = 0,
    #[serde(rename = "CALLBACK_REQUEST_NEW_CONTACT_POINT")]
    CallbackRequestNewContactPoint = 1,
    #[serde(rename = "CALLBACK_REQUEST_SETUP_PPU_ONLY")]
    CallbackRequestSetupPpuOnly = 2,
    #[serde(rename = "CALLBACK_REQUEST_SETUP_CALLBACK")]
    CallbackRequestSetupCallback = 4,
    #[serde(rename = "CALLBACK_REQUEST_CONTACT_POINT_CALLBACK")]
    CallbackRequestContactPointCallback = 8,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum SolvingMethod {
    #[serde(rename = "METHOD_STABILIZED")]
    #[default]
    MethodStabilized = 0,
    #[serde(rename = "METHOD_OLD")]
    MethodOld = 1,
}
