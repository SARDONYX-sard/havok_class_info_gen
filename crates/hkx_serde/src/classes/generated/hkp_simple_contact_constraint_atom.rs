//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpSimpleContactConstraintAtom`
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

/// `hkpSimpleContactConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: false
/// -    parent: `hkpConstraintAtom`/`0x59d67ef6`
/// - signature: `0x920df11a`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpSimpleContactConstraintAtom {
    /// # C++ Parent class(`hkpConstraintAtom` => parent: `None`) field Info
    /// -   name:`"type"`
    /// -   type: `enum AtomType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub _type: AtomType,

    /// # C++ Class Fields Info
    /// -   name:`"sizeOfAllAtoms"`
    /// -   type: `hkUint16`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    pub size_of_all_atoms: u16,
    /// # C++ Class Fields Info
    /// -   name:`"numContactPoints"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub num_contact_points: u16,
    /// # C++ Class Fields Info
    /// -   name:`"numReservedContactPoints"`
    /// -   type: `hkUint16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE`
    pub num_reserved_contact_points: u16,
    /// # C++ Class Fields Info
    /// -   name:`"numUserDatasForBodyA"`
    /// -   type: `hkUint8`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub num_user_datas_for_body_a: u8,
    /// # C++ Class Fields Info
    /// -   name:`"numUserDatasForBodyB"`
    /// -   type: `hkUint8`
    /// - offset: 9
    /// -  flags: `FLAGS_NONE`
    pub num_user_datas_for_body_b: u8,
    /// # C++ Class Fields Info
    /// -   name:`"contactPointPropertiesStriding"`
    /// -   type: `hkUint8`
    /// - offset: 10
    /// -  flags: `FLAGS_NONE`
    pub contact_point_properties_striding: u8,
    /// # C++ Class Fields Info
    /// -   name:`"maxNumContactPoints"`
    /// -   type: `hkUint16`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub max_num_contact_points: u16,
    /// # C++ Class Fields Info
    /// -   name:`"info"`
    /// -   type: `struct hkpSimpleContactConstraintDataInfo`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE|ALIGN16`
    pub info: SingleClass<HkpSimpleContactConstraintDataInfo>,
}

impl Serialize for HkpSimpleContactConstraintAtom {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpSimpleContactConstraintAtomVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpSimpleContactConstraintAtom {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpSimpleContactConstraintAtomVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpSimpleContactConstraintAtomVisitor>> for HkpSimpleContactConstraintAtom {
    fn from(_values: Vec<HkpSimpleContactConstraintAtomVisitor>) -> Self {
            let mut _type = None;
            let mut size_of_all_atoms = None;
            let mut num_contact_points = None;
            let mut num_reserved_contact_points = None;
            let mut num_user_datas_for_body_a = None;
            let mut num_user_datas_for_body_b = None;
            let mut contact_point_properties_striding = None;
            let mut max_num_contact_points = None;
            let mut info = None;


        for _value in _values {
            match _value {
                HkpSimpleContactConstraintAtomVisitor::Type(m) => _type = Some(m),
                HkpSimpleContactConstraintAtomVisitor::SizeOfAllAtoms(m) => size_of_all_atoms = Some(m),
                HkpSimpleContactConstraintAtomVisitor::NumContactPoints(m) => num_contact_points = Some(m),
                HkpSimpleContactConstraintAtomVisitor::NumReservedContactPoints(m) => num_reserved_contact_points = Some(m),
                HkpSimpleContactConstraintAtomVisitor::NumUserDatasForBodyA(m) => num_user_datas_for_body_a = Some(m),
                HkpSimpleContactConstraintAtomVisitor::NumUserDatasForBodyB(m) => num_user_datas_for_body_b = Some(m),
                HkpSimpleContactConstraintAtomVisitor::ContactPointPropertiesStriding(m) => contact_point_properties_striding = Some(m),
                HkpSimpleContactConstraintAtomVisitor::MaxNumContactPoints(m) => max_num_contact_points = Some(m),
                HkpSimpleContactConstraintAtomVisitor::Info(m) => info = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            _type: _type.unwrap_or_default().into_inner(),
            size_of_all_atoms: size_of_all_atoms.unwrap_or_default().into_inner(),
            num_contact_points: num_contact_points.unwrap_or_default().into_inner(),
            num_reserved_contact_points: num_reserved_contact_points.unwrap_or_default().into_inner(),
            num_user_datas_for_body_a: num_user_datas_for_body_a.unwrap_or_default().into_inner(),
            num_user_datas_for_body_b: num_user_datas_for_body_b.unwrap_or_default().into_inner(),
            contact_point_properties_striding: contact_point_properties_striding.unwrap_or_default().into_inner(),
            max_num_contact_points: max_num_contact_points.unwrap_or_default().into_inner(),
            info: info.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpSimpleContactConstraintAtom> for Vec<HkpSimpleContactConstraintAtomVisitor> {
    fn from(data: &HkpSimpleContactConstraintAtom) -> Self {
        vec![
            HkpSimpleContactConstraintAtomVisitor::Type(data._type.clone().into()),
            HkpSimpleContactConstraintAtomVisitor::SizeOfAllAtoms(data.size_of_all_atoms.into()),
            HkpSimpleContactConstraintAtomVisitor::NumContactPoints(data.num_contact_points.into()),
            HkpSimpleContactConstraintAtomVisitor::NumReservedContactPoints(data.num_reserved_contact_points.into()),
            HkpSimpleContactConstraintAtomVisitor::NumUserDatasForBodyA(data.num_user_datas_for_body_a.into()),
            HkpSimpleContactConstraintAtomVisitor::NumUserDatasForBodyB(data.num_user_datas_for_body_b.into()),
            HkpSimpleContactConstraintAtomVisitor::ContactPointPropertiesStriding(data.contact_point_properties_striding.into()),
            HkpSimpleContactConstraintAtomVisitor::MaxNumContactPoints(data.max_num_contact_points.into()),
            HkpSimpleContactConstraintAtomVisitor::Info(data.info.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpSimpleContactConstraintAtom {
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
enum HkpSimpleContactConstraintAtomVisitor {
    /// Visitor fields
    #[serde(rename = "type")]
    Type(Primitive<AtomType>),

    /// Visitor fields
    #[serde(rename = "sizeOfAllAtoms")]
    SizeOfAllAtoms(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "numContactPoints")]
    NumContactPoints(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "numReservedContactPoints")]
    NumReservedContactPoints(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "numUserDatasForBodyA")]
    NumUserDatasForBodyA(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "numUserDatasForBodyB")]
    NumUserDatasForBodyB(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "contactPointPropertiesStriding")]
    ContactPointPropertiesStriding(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "maxNumContactPoints")]
    MaxNumContactPoints(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "info")]
    Info(SingleClass<HkpSimpleContactConstraintDataInfo>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpSimpleContactConstraintAtomVisitor, "@name",
    ("type" => Type(Primitive<AtomType>)),
    ("sizeOfAllAtoms" => SizeOfAllAtoms(Primitive<u16>)),
    ("numContactPoints" => NumContactPoints(Primitive<u16>)),
    ("numReservedContactPoints" => NumReservedContactPoints(Primitive<u16>)),
    ("numUserDatasForBodyA" => NumUserDatasForBodyA(Primitive<u8>)),
    ("numUserDatasForBodyB" => NumUserDatasForBodyB(Primitive<u8>)),
    ("contactPointPropertiesStriding" => ContactPointPropertiesStriding(Primitive<u8>)),
    ("maxNumContactPoints" => MaxNumContactPoints(Primitive<u16>)),
    ("info" => Info(SingleClass<HkpSimpleContactConstraintDataInfo>)),
}
