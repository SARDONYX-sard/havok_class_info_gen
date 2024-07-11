//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpStiffSpringChainData`
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

/// `hkpStiffSpringChainData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: true
/// -    parent: `hkpConstraintChainData`/`0x5facc7ff`
/// - signature: `0xf170356b`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpStiffSpringChainData<'a> {
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
    /// -   type: `hkArray<struct hkpStiffSpringChainDataConstraintInfo>`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub infos: HkArrayClass<HkpStiffSpringChainDataConstraintInfo>,
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
    /// -   name:`"cfm"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub cfm: f32,
}

impl Serialize for HkpStiffSpringChainData<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpStiffSpringChainDataVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpStiffSpringChainData<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpStiffSpringChainDataVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpStiffSpringChainDataVisitor<'a>>> for HkpStiffSpringChainData<'a> {
    fn from(_values: Vec<HkpStiffSpringChainDataVisitor<'a>>) -> Self {
            let mut atoms = None;
            let mut infos = None;
            let mut tau = None;
            let mut damping = None;
            let mut cfm = None;


        for _value in _values {
            match _value {
                HkpStiffSpringChainDataVisitor::Atoms(m) => atoms = Some(m),
                HkpStiffSpringChainDataVisitor::Infos(m) => infos = Some(m),
                HkpStiffSpringChainDataVisitor::Tau(m) => tau = Some(m),
                HkpStiffSpringChainDataVisitor::Damping(m) => damping = Some(m),
                HkpStiffSpringChainDataVisitor::Cfm(m) => cfm = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            atoms: atoms.unwrap_or_default(),
            infos: infos.unwrap_or_default(),
            tau: tau.unwrap_or_default().into_inner(),
            damping: damping.unwrap_or_default().into_inner(),
            cfm: cfm.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpStiffSpringChainData<'a>> for Vec<HkpStiffSpringChainDataVisitor<'a>> {
    fn from(data: &HkpStiffSpringChainData<'a>) -> Self {
        vec![
            HkpStiffSpringChainDataVisitor::Atoms(data.atoms.clone()),
            HkpStiffSpringChainDataVisitor::Infos(data.infos.clone()),
            HkpStiffSpringChainDataVisitor::Tau(data.tau.into()),
            HkpStiffSpringChainDataVisitor::Damping(data.damping.into()),
            HkpStiffSpringChainDataVisitor::Cfm(data.cfm.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpStiffSpringChainData<'de> {
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
enum HkpStiffSpringChainDataVisitor<'a> {
    // C++ Parent class(`hkpConstraintChainData` => parent: `hkpConstraintData`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "atoms")]
    Atoms(SingleClass<HkpBridgeAtoms<'a>>),
    /// Visitor fields
    #[serde(rename = "infos")]
    Infos(HkArrayClass<HkpStiffSpringChainDataConstraintInfo>),
    /// Visitor fields
    #[serde(rename = "tau")]
    Tau(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "damping")]
    Damping(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "cfm")]
    Cfm(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpStiffSpringChainDataVisitor<'de>, "@name",
    ("atoms" => Atoms(SingleClass<HkpBridgeAtoms<'de>>)),
    ("infos" => Infos(HkArrayClass<HkpStiffSpringChainDataConstraintInfo>)),
    ("tau" => Tau(Primitive<f32>)),
    ("damping" => Damping(Primitive<f32>)),
    ("cfm" => Cfm(Primitive<f32>)),
}
