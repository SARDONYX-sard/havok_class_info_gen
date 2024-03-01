//! A Rust structure that implements a serializer/deserializer corresponding to `hkpConstraintAtom`, a class defined in C++
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// In XML, it is enclosed in a `hkobject` tag
/// and the `class` attribute contains the C++ class nam
///
/// # Information on the original C++ class
/// -    size: 2
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpConstraintAtom<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpConstraintAtom"`: Name of this class.
    #[serde(default = "HkpConstraintAtom::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x59d67ef6`: Unique value of this class.
    #[serde(default = "HkpConstraintAtom::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpConstraintAtomHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpConstraintAtomHkParam<'a>>
}

impl HkpConstraintAtom<'_> {
    /// Return `"hkpConstraintAtom"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpConstraintAtom".into()
    }

    /// Return `"0x59d67ef6"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x59d67ef6".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpConstraintAtomHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"type"`
    /// -   type: `enum AtomType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(AtomType),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpConstraintAtomHkParam<'de>, "@name",
    ("type" => Type(AtomType)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AtomType {
    #[serde(rename = "TYPE_INVALID")]
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CallbackRequest {
    #[serde(rename = "CALLBACK_REQUEST_NONE")]
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SolvingMethod {
    #[serde(rename = "METHOD_STABILIZED")]
    MethodStabilized = 0,
    #[serde(rename = "METHOD_OLD")]
    MethodOld = 1,
}
