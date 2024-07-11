//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpPointToPathConstraintData`
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

/// `hkpPointToPathConstraintData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 176
/// -    vtable: true
/// -    parent: `hkpConstraintData`/`0x80559a4e`
/// - signature: `0x8e7cb5da`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpPointToPathConstraintData<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"atoms"`
    /// -   type: `struct hkpBridgeAtoms`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub atoms: SingleClass<HkpBridgeAtoms<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"path"`
    /// -   type: `struct hkpParametricCurve*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub path: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"maxFrictionForce"`
    /// -   type: `hkReal`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub max_friction_force: f32,
    /// # C++ Class Fields Info
    /// -   name:`"angularConstrainedDOF"`
    /// -   type: `enum OrientationConstraintType`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub angular_constrained_dof: OrientationConstraintType,
    /// # C++ Class Fields Info
    /// -   name:`"transform_OS_KS"`
    /// -   type: `hkTransform[2]`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub transform_os_ks: CStyleArrayVector<Transform<f32>, 2>,
}

impl Serialize for HkpPointToPathConstraintData<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpPointToPathConstraintDataVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpPointToPathConstraintData<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpPointToPathConstraintDataVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpPointToPathConstraintDataVisitor<'a>>> for HkpPointToPathConstraintData<'a> {
    fn from(_values: Vec<HkpPointToPathConstraintDataVisitor<'a>>) -> Self {
            let mut atoms = None;
            let mut path = None;
            let mut max_friction_force = None;
            let mut angular_constrained_dof = None;
            let mut transform_os_ks = None;


        for _value in _values {
            match _value {
                HkpPointToPathConstraintDataVisitor::Atoms(m) => atoms = Some(m),
                HkpPointToPathConstraintDataVisitor::Path(m) => path = Some(m),
                HkpPointToPathConstraintDataVisitor::MaxFrictionForce(m) => max_friction_force = Some(m),
                HkpPointToPathConstraintDataVisitor::AngularConstrainedDof(m) => angular_constrained_dof = Some(m),
                HkpPointToPathConstraintDataVisitor::TransformOsKs(m) => transform_os_ks = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            atoms: atoms.unwrap_or_default(),
            path: path.unwrap_or_default().into_inner(),
            max_friction_force: max_friction_force.unwrap_or_default().into_inner(),
            angular_constrained_dof: angular_constrained_dof.unwrap_or_default().into_inner(),
            transform_os_ks: transform_os_ks.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpPointToPathConstraintData<'a>> for Vec<HkpPointToPathConstraintDataVisitor<'a>> {
    fn from(data: &HkpPointToPathConstraintData<'a>) -> Self {
        vec![
            HkpPointToPathConstraintDataVisitor::Atoms(data.atoms.clone()),
            HkpPointToPathConstraintDataVisitor::Path(data.path.clone().into()),
            HkpPointToPathConstraintDataVisitor::MaxFrictionForce(data.max_friction_force.into()),
            HkpPointToPathConstraintDataVisitor::AngularConstrainedDof(data.angular_constrained_dof.clone().into()),
            HkpPointToPathConstraintDataVisitor::TransformOsKs(data.transform_os_ks.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpPointToPathConstraintData<'de> {
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
enum HkpPointToPathConstraintDataVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "atoms")]
    Atoms(SingleClass<HkpBridgeAtoms<'a>>),
    /// Visitor fields
    #[serde(rename = "path")]
    Path(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "maxFrictionForce")]
    MaxFrictionForce(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "angularConstrainedDOF")]
    AngularConstrainedDof(Primitive<OrientationConstraintType>),
    /// Visitor fields
    #[serde(rename = "transform_OS_KS")]
    TransformOsKs(CStyleArrayVector<Transform<f32>, 2>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpPointToPathConstraintDataVisitor<'de>, "@name",
    ("atoms" => Atoms(SingleClass<HkpBridgeAtoms<'de>>)),
    ("path" => Path(Primitive<Cow<'de, str>>)),
    ("maxFrictionForce" => MaxFrictionForce(Primitive<f32>)),
    ("angularConstrainedDOF" => AngularConstrainedDof(Primitive<OrientationConstraintType>)),
    ("transform_OS_KS" => TransformOsKs(CStyleArrayVector<Transform<f32>, 2>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum OrientationConstraintType {
    #[serde(rename = "CONSTRAIN_ORIENTATION_INVALID")]
    #[default]
    ConstrainOrientationInvalid = 0,
    #[serde(rename = "CONSTRAIN_ORIENTATION_NONE")]
    ConstrainOrientationNone = 1,
    #[serde(rename = "CONSTRAIN_ORIENTATION_ALLOW_SPIN")]
    ConstrainOrientationAllowSpin = 2,
    #[serde(rename = "CONSTRAIN_ORIENTATION_TO_PATH")]
    ConstrainOrientationToPath = 3,
    #[serde(rename = "CONSTRAIN_ORIENTATION_MAX_ID")]
    ConstrainOrientationMaxId = 4,
}
