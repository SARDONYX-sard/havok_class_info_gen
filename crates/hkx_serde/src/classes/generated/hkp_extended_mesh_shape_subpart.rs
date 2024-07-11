//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpExtendedMeshShapeSubpart`
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

/// `hkpExtendedMeshShapeSubpart`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 20
/// -    vtable: false
/// - signature: `0xf4608207`
/// -   version: 2
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpExtendedMeshShapeSubpart<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"type"`
    /// -   type: `enum SubpartType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub _type: SubpartType,
    /// # C++ Class Fields Info
    /// -   name:`"materialIndexStridingType"`
    /// -   type: `enum MaterialIndexStridingType`
    /// - offset: 1
    /// -  flags: `FLAGS_NONE`
    pub material_index_striding_type: MaterialIndexStridingType,
    /// # C++ Class Fields Info
    /// -   name:`"materialStriding"`
    /// -   type: `hkInt16`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub material_striding: i16,
    /// # C++ Class Fields Info
    /// -   name:`"materialIndexBase"`
    /// -   type: `void*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub material_index_base: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"materialIndexStriding"`
    /// -   type: `hkUint16`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub material_index_striding: u16,
    /// # C++ Class Fields Info
    /// -   name:`"numMaterials"`
    /// -   type: `hkUint16`
    /// - offset: 10
    /// -  flags: `FLAGS_NONE`
    pub num_materials: u16,
    /// # C++ Class Fields Info
    /// -   name:`"materialBase"`
    /// -   type: `void*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub material_base: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub user_data: usize,
}

impl Serialize for HkpExtendedMeshShapeSubpart<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpExtendedMeshShapeSubpartVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpExtendedMeshShapeSubpart<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpExtendedMeshShapeSubpartVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpExtendedMeshShapeSubpartVisitor<'a>>> for HkpExtendedMeshShapeSubpart<'a> {
    fn from(_values: Vec<HkpExtendedMeshShapeSubpartVisitor<'a>>) -> Self {
            let mut _type = None;
            let mut material_index_striding_type = None;
            let mut material_striding = None;
            let mut material_index_base = None;
            let mut material_index_striding = None;
            let mut num_materials = None;
            let mut material_base = None;
            let mut user_data = None;


        for _value in _values {
            match _value {
                HkpExtendedMeshShapeSubpartVisitor::Type(m) => _type = Some(m),
                HkpExtendedMeshShapeSubpartVisitor::MaterialIndexStridingType(m) => material_index_striding_type = Some(m),
                HkpExtendedMeshShapeSubpartVisitor::MaterialStriding(m) => material_striding = Some(m),
                HkpExtendedMeshShapeSubpartVisitor::MaterialIndexBase(m) => material_index_base = Some(m),
                HkpExtendedMeshShapeSubpartVisitor::MaterialIndexStriding(m) => material_index_striding = Some(m),
                HkpExtendedMeshShapeSubpartVisitor::NumMaterials(m) => num_materials = Some(m),
                HkpExtendedMeshShapeSubpartVisitor::MaterialBase(m) => material_base = Some(m),
                HkpExtendedMeshShapeSubpartVisitor::UserData(m) => user_data = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            _type: _type.unwrap_or_default().into_inner(),
            material_index_striding_type: material_index_striding_type.unwrap_or_default().into_inner(),
            material_striding: material_striding.unwrap_or_default().into_inner(),
            material_index_base: material_index_base.unwrap_or_default().into_inner(),
            material_index_striding: material_index_striding.unwrap_or_default().into_inner(),
            num_materials: num_materials.unwrap_or_default().into_inner(),
            material_base: material_base.unwrap_or_default().into_inner(),
            user_data: user_data.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpExtendedMeshShapeSubpart<'a>> for Vec<HkpExtendedMeshShapeSubpartVisitor<'a>> {
    fn from(data: &HkpExtendedMeshShapeSubpart<'a>) -> Self {
        vec![
            HkpExtendedMeshShapeSubpartVisitor::Type(data._type.clone().into()),
            HkpExtendedMeshShapeSubpartVisitor::MaterialIndexStridingType(data.material_index_striding_type.clone().into()),
            HkpExtendedMeshShapeSubpartVisitor::MaterialStriding(data.material_striding.into()),
            HkpExtendedMeshShapeSubpartVisitor::MaterialIndexBase(data.material_index_base.clone().into()),
            HkpExtendedMeshShapeSubpartVisitor::MaterialIndexStriding(data.material_index_striding.into()),
            HkpExtendedMeshShapeSubpartVisitor::NumMaterials(data.num_materials.into()),
            HkpExtendedMeshShapeSubpartVisitor::MaterialBase(data.material_base.clone().into()),
            HkpExtendedMeshShapeSubpartVisitor::UserData(data.user_data.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpExtendedMeshShapeSubpart<'de> {
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
enum HkpExtendedMeshShapeSubpartVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "type")]
    Type(Primitive<SubpartType>),
    /// Visitor fields
    #[serde(rename = "materialIndexStridingType")]
    MaterialIndexStridingType(Primitive<MaterialIndexStridingType>),
    /// Visitor fields
    #[serde(rename = "materialStriding", skip_serializing)]
    MaterialStriding(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "materialIndexBase", skip_serializing)]
    MaterialIndexBase(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "materialIndexStriding")]
    MaterialIndexStriding(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "numMaterials")]
    NumMaterials(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "materialBase", skip_serializing)]
    MaterialBase(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "userData")]
    UserData(Primitive<usize>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpExtendedMeshShapeSubpartVisitor<'de>, "@name",
    ("type" => Type(Primitive<SubpartType>)),
    ("materialIndexStridingType" => MaterialIndexStridingType(Primitive<MaterialIndexStridingType>)),
    ("materialStriding" => MaterialStriding(Primitive<i16>)),
    ("materialIndexBase" => MaterialIndexBase(Primitive<Cow<'de, str>>)),
    ("materialIndexStriding" => MaterialIndexStriding(Primitive<u16>)),
    ("numMaterials" => NumMaterials(Primitive<u16>)),
    ("materialBase" => MaterialBase(Primitive<Cow<'de, str>>)),
    ("userData" => UserData(Primitive<usize>)),
}
