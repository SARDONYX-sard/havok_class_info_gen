//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkxMaterial`
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

/// `hkxMaterial`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 144
/// -    vtable: true
/// -    parent: `hkxAttributeHolder`/`0x7468cc44`
/// - signature: `0x2954537a`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkxMaterial<'a> {
    /// # C++ Parent class(`hkxAttributeHolder` => parent: `hkReferencedObject`) field Info
    /// -   name:`"attributeGroups"`
    /// -   type: `hkArray<struct hkxAttributeGroup>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub attribute_groups: HkArrayClass<HkxAttributeGroup<'a>>,

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
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub name: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"stages"`
    /// -   type: `hkArray<struct hkxMaterialTextureStage>`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub stages: HkArrayClass<HkxMaterialTextureStage<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"diffuseColor"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub diffuse_color: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"ambientColor"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub ambient_color: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"specularColor"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub specular_color: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"emissiveColor"`
    /// -   type: `hkVector4`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    pub emissive_color: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"subMaterials"`
    /// -   type: `hkArray<hkxMaterial*>`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    pub sub_materials: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"extraData"`
    /// -   type: `struct hkReferencedObject*`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE`
    pub extra_data: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"properties"`
    /// -   type: `hkArray<struct hkxMaterialProperty>`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    pub properties: HkArrayClass<HkxMaterialProperty>,
}

impl Serialize for HkxMaterial<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkxMaterialVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkxMaterial<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkxMaterialVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkxMaterialVisitor<'a>>> for HkxMaterial<'a> {
    fn from(_values: Vec<HkxMaterialVisitor<'a>>) -> Self {
            let mut attribute_groups = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut name = None;
            let mut stages = None;
            let mut diffuse_color = None;
            let mut ambient_color = None;
            let mut specular_color = None;
            let mut emissive_color = None;
            let mut sub_materials = None;
            let mut extra_data = None;
            let mut properties = None;


        for _value in _values {
            match _value {
                HkxMaterialVisitor::AttributeGroups(m) => attribute_groups = Some(m),
                HkxMaterialVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkxMaterialVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkxMaterialVisitor::Name(m) => name = Some(m),
                HkxMaterialVisitor::Stages(m) => stages = Some(m),
                HkxMaterialVisitor::DiffuseColor(m) => diffuse_color = Some(m),
                HkxMaterialVisitor::AmbientColor(m) => ambient_color = Some(m),
                HkxMaterialVisitor::SpecularColor(m) => specular_color = Some(m),
                HkxMaterialVisitor::EmissiveColor(m) => emissive_color = Some(m),
                HkxMaterialVisitor::SubMaterials(m) => sub_materials = Some(m),
                HkxMaterialVisitor::ExtraData(m) => extra_data = Some(m),
                HkxMaterialVisitor::Properties(m) => properties = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            attribute_groups: attribute_groups.unwrap_or_default(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            name: name.unwrap_or_default().into_inner(),
            stages: stages.unwrap_or_default(),
            diffuse_color: diffuse_color.unwrap_or_default().into_inner(),
            ambient_color: ambient_color.unwrap_or_default().into_inner(),
            specular_color: specular_color.unwrap_or_default().into_inner(),
            emissive_color: emissive_color.unwrap_or_default().into_inner(),
            sub_materials: sub_materials.unwrap_or_default(),
            extra_data: extra_data.unwrap_or_default().into_inner(),
            properties: properties.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkxMaterial<'a>> for Vec<HkxMaterialVisitor<'a>> {
    fn from(data: &HkxMaterial<'a>) -> Self {
        vec![
            HkxMaterialVisitor::AttributeGroups(data.attribute_groups.clone()),
            HkxMaterialVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkxMaterialVisitor::ReferenceCount(data.reference_count.into()),
            HkxMaterialVisitor::Name(data.name.clone().into()),
            HkxMaterialVisitor::Stages(data.stages.clone()),
            HkxMaterialVisitor::DiffuseColor(data.diffuse_color.into()),
            HkxMaterialVisitor::AmbientColor(data.ambient_color.into()),
            HkxMaterialVisitor::SpecularColor(data.specular_color.into()),
            HkxMaterialVisitor::EmissiveColor(data.emissive_color.into()),
            HkxMaterialVisitor::SubMaterials(data.sub_materials.clone()),
            HkxMaterialVisitor::ExtraData(data.extra_data.clone().into()),
            HkxMaterialVisitor::Properties(data.properties.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkxMaterial<'de> {
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
enum HkxMaterialVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "attributeGroups")]
    AttributeGroups(HkArrayClass<HkxAttributeGroup<'a>>),

    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "stages")]
    Stages(HkArrayClass<HkxMaterialTextureStage<'a>>),
    /// Visitor fields
    #[serde(rename = "diffuseColor")]
    DiffuseColor(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "ambientColor")]
    AmbientColor(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "specularColor")]
    SpecularColor(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "emissiveColor")]
    EmissiveColor(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "subMaterials")]
    SubMaterials(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "extraData")]
    ExtraData(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "properties")]
    Properties(HkArrayClass<HkxMaterialProperty>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxMaterialVisitor<'de>, "@name",
    ("attributeGroups" => AttributeGroups(HkArrayClass<HkxAttributeGroup<'de>>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("stages" => Stages(HkArrayClass<HkxMaterialTextureStage<'de>>)),
    ("diffuseColor" => DiffuseColor(Primitive<Vector4<f32>>)),
    ("ambientColor" => AmbientColor(Primitive<Vector4<f32>>)),
    ("specularColor" => SpecularColor(Primitive<Vector4<f32>>)),
    ("emissiveColor" => EmissiveColor(Primitive<Vector4<f32>>)),
    ("subMaterials" => SubMaterials(HkArrayRef<Cow<'de, str>>)),
    ("extraData" => ExtraData(Primitive<Cow<'de, str>>)),
    ("properties" => Properties(HkArrayClass<HkxMaterialProperty>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum TextureType {
    #[serde(rename = "TEX_UNKNOWN")]
    #[default]
    TexUnknown = 0,
    #[serde(rename = "TEX_DIFFUSE")]
    TexDiffuse = 1,
    #[serde(rename = "TEX_REFLECTION")]
    TexReflection = 2,
    #[serde(rename = "TEX_BUMP")]
    TexBump = 3,
    #[serde(rename = "TEX_NORMAL")]
    TexNormal = 4,
    #[serde(rename = "TEX_DISPLACEMENT")]
    TexDisplacement = 5,
    #[serde(rename = "TEX_SPECULAR")]
    TexSpecular = 6,
    #[serde(rename = "TEX_SPECULARANDGLOSS")]
    TexSpecularandgloss = 7,
    #[serde(rename = "TEX_OPACITY")]
    TexOpacity = 8,
    #[serde(rename = "TEX_EMISSIVE")]
    TexEmissive = 9,
    #[serde(rename = "TEX_REFRACTION")]
    TexRefraction = 10,
    #[serde(rename = "TEX_GLOSS")]
    TexGloss = 11,
    #[serde(rename = "TEX_NOTEXPORTED")]
    TexNotexported = 12,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum PropertyKey {
    #[serde(rename = "PROPERTY_MTL_TYPE_BLEND")]
    #[default]
    PropertyMtlTypeBlend = 1,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE0")]
    PropertyMtlUvIdStage0 = 256,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE1")]
    PropertyMtlUvIdStage1 = 257,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE2")]
    PropertyMtlUvIdStage2 = 258,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE3")]
    PropertyMtlUvIdStage3 = 259,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE4")]
    PropertyMtlUvIdStage4 = 260,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE5")]
    PropertyMtlUvIdStage5 = 261,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE6")]
    PropertyMtlUvIdStage6 = 262,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE7")]
    PropertyMtlUvIdStage7 = 263,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE8")]
    PropertyMtlUvIdStage8 = 264,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE9")]
    PropertyMtlUvIdStage9 = 265,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE10")]
    PropertyMtlUvIdStage10 = 266,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE11")]
    PropertyMtlUvIdStage11 = 267,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE12")]
    PropertyMtlUvIdStage12 = 268,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE13")]
    PropertyMtlUvIdStage13 = 269,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE14")]
    PropertyMtlUvIdStage14 = 270,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE15")]
    PropertyMtlUvIdStage15 = 271,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE_MAX")]
    PropertyMtlUvIdStageMax = 272,
}
