//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpPoweredChainData`
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

/// `hkpPoweredChainData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: true
/// -    parent: `hkpConstraintChainData`/`0x5facc7ff`
/// - signature: `0x38aeafc3`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpPoweredChainData<'a> {
    // C++ Parent class(`hkpConstraintChainData` => parent: `hkpConstraintData`) has no fields
    //
    /// # C++ Class Fields Info
    /// -   name:`"atoms"`
    /// -   type: `struct hkpBridgeAtoms`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub atoms: SingleClass<HkpBridgeAtoms<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"infos"`
    /// -   type: `hkArray<struct hkpPoweredChainDataConstraintInfo>`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub infos: HkArrayClass<HkpPoweredChainDataConstraintInfo<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"tau"`
    /// -   type: `hkReal`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    pub tau: f32,
    /// # C++ Class Fields Info
    /// -   name:`"damping"`
    /// -   type: `hkReal`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub damping: f32,
    /// # C++ Class Fields Info
    /// -   name:`"cfmLinAdd"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub cfm_lin_add: f32,
    /// # C++ Class Fields Info
    /// -   name:`"cfmLinMul"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub cfm_lin_mul: f32,
    /// # C++ Class Fields Info
    /// -   name:`"cfmAngAdd"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub cfm_ang_add: f32,
    /// # C++ Class Fields Info
    /// -   name:`"cfmAngMul"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    pub cfm_ang_mul: f32,
    /// # C++ Class Fields Info
    /// -   name:`"maxErrorDistance"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    pub max_error_distance: f32,
}

impl Serialize for HkpPoweredChainData<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpPoweredChainDataVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpPoweredChainData<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpPoweredChainDataVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpPoweredChainDataVisitor<'a>>> for HkpPoweredChainData<'a> {
    fn from(_values: Vec<HkpPoweredChainDataVisitor<'a>>) -> Self {
            let mut atoms = None;
            let mut infos = None;
            let mut tau = None;
            let mut damping = None;
            let mut cfm_lin_add = None;
            let mut cfm_lin_mul = None;
            let mut cfm_ang_add = None;
            let mut cfm_ang_mul = None;
            let mut max_error_distance = None;


        for _value in _values {
            match _value {
                HkpPoweredChainDataVisitor::Atoms(m) => atoms = Some(m),
                HkpPoweredChainDataVisitor::Infos(m) => infos = Some(m),
                HkpPoweredChainDataVisitor::Tau(m) => tau = Some(m),
                HkpPoweredChainDataVisitor::Damping(m) => damping = Some(m),
                HkpPoweredChainDataVisitor::CfmLinAdd(m) => cfm_lin_add = Some(m),
                HkpPoweredChainDataVisitor::CfmLinMul(m) => cfm_lin_mul = Some(m),
                HkpPoweredChainDataVisitor::CfmAngAdd(m) => cfm_ang_add = Some(m),
                HkpPoweredChainDataVisitor::CfmAngMul(m) => cfm_ang_mul = Some(m),
                HkpPoweredChainDataVisitor::MaxErrorDistance(m) => max_error_distance = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            atoms: atoms.unwrap_or_default(),
            infos: infos.unwrap_or_default(),
            tau: tau.unwrap_or_default().into_inner(),
            damping: damping.unwrap_or_default().into_inner(),
            cfm_lin_add: cfm_lin_add.unwrap_or_default().into_inner(),
            cfm_lin_mul: cfm_lin_mul.unwrap_or_default().into_inner(),
            cfm_ang_add: cfm_ang_add.unwrap_or_default().into_inner(),
            cfm_ang_mul: cfm_ang_mul.unwrap_or_default().into_inner(),
            max_error_distance: max_error_distance.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpPoweredChainData<'a>> for Vec<HkpPoweredChainDataVisitor<'a>> {
    fn from(data: &HkpPoweredChainData<'a>) -> Self {
        vec![
            HkpPoweredChainDataVisitor::Atoms(data.atoms.clone()),
            HkpPoweredChainDataVisitor::Infos(data.infos.clone()),
            HkpPoweredChainDataVisitor::Tau(data.tau.into()),
            HkpPoweredChainDataVisitor::Damping(data.damping.into()),
            HkpPoweredChainDataVisitor::CfmLinAdd(data.cfm_lin_add.into()),
            HkpPoweredChainDataVisitor::CfmLinMul(data.cfm_lin_mul.into()),
            HkpPoweredChainDataVisitor::CfmAngAdd(data.cfm_ang_add.into()),
            HkpPoweredChainDataVisitor::CfmAngMul(data.cfm_ang_mul.into()),
            HkpPoweredChainDataVisitor::MaxErrorDistance(data.max_error_distance.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpPoweredChainData<'de> {
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
enum HkpPoweredChainDataVisitor<'a> {
    // C++ Parent class(`hkpConstraintChainData` => parent: `hkpConstraintData`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "atoms")]
    Atoms(SingleClass<HkpBridgeAtoms<'a>>),
    /// Visitor fields
    #[serde(rename = "infos")]
    Infos(HkArrayClass<HkpPoweredChainDataConstraintInfo<'a>>),
    /// Visitor fields
    #[serde(rename = "tau")]
    Tau(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "damping")]
    Damping(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "cfmLinAdd")]
    CfmLinAdd(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "cfmLinMul")]
    CfmLinMul(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "cfmAngAdd")]
    CfmAngAdd(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "cfmAngMul")]
    CfmAngMul(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "maxErrorDistance")]
    MaxErrorDistance(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpPoweredChainDataVisitor<'de>, "@name",
    ("atoms" => Atoms(SingleClass<HkpBridgeAtoms<'de>>)),
    ("infos" => Infos(HkArrayClass<HkpPoweredChainDataConstraintInfo<'de>>)),
    ("tau" => Tau(Primitive<f32>)),
    ("damping" => Damping(Primitive<f32>)),
    ("cfmLinAdd" => CfmLinAdd(Primitive<f32>)),
    ("cfmLinMul" => CfmLinMul(Primitive<f32>)),
    ("cfmAngAdd" => CfmAngAdd(Primitive<f32>)),
    ("cfmAngMul" => CfmAngMul(Primitive<f32>)),
    ("maxErrorDistance" => MaxErrorDistance(Primitive<f32>)),
}
