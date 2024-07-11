//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkxScene`
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

/// `hkxScene`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 176
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x5f673ddd`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkxScene<'a> {
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
    /// -   name:`"modeller"`
    /// -   type: `hkStringPtr`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub modeller: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"asset"`
    /// -   type: `hkStringPtr`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub asset: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"sceneLength"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub scene_length: f32,
    /// # C++ Class Fields Info
    /// -   name:`"rootNode"`
    /// -   type: `struct hkxNode*`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub root_node: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"selectionSets"`
    /// -   type: `hkArray<hkxNodeSelectionSet*>`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub selection_sets: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"cameras"`
    /// -   type: `hkArray<hkxCamera*>`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    pub cameras: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"lights"`
    /// -   type: `hkArray<hkxLight*>`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub lights: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"meshes"`
    /// -   type: `hkArray<hkxMesh*>`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    pub meshes: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"materials"`
    /// -   type: `hkArray<hkxMaterial*>`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    pub materials: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"inplaceTextures"`
    /// -   type: `hkArray<hkxTextureInplace*>`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    pub inplace_textures: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"externalTextures"`
    /// -   type: `hkArray<hkxTextureFile*>`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    pub external_textures: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"skinBindings"`
    /// -   type: `hkArray<hkxSkinBinding*>`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    pub skin_bindings: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"appliedTransform"`
    /// -   type: `hkMatrix3`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    pub applied_transform: Matrix3<f32>,
}

impl Serialize for HkxScene<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkxSceneVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkxScene<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkxSceneVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkxSceneVisitor<'a>>> for HkxScene<'a> {
    fn from(_values: Vec<HkxSceneVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut modeller = None;
            let mut asset = None;
            let mut scene_length = None;
            let mut root_node = None;
            let mut selection_sets = None;
            let mut cameras = None;
            let mut lights = None;
            let mut meshes = None;
            let mut materials = None;
            let mut inplace_textures = None;
            let mut external_textures = None;
            let mut skin_bindings = None;
            let mut applied_transform = None;


        for _value in _values {
            match _value {
                HkxSceneVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkxSceneVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkxSceneVisitor::Modeller(m) => modeller = Some(m),
                HkxSceneVisitor::Asset(m) => asset = Some(m),
                HkxSceneVisitor::SceneLength(m) => scene_length = Some(m),
                HkxSceneVisitor::RootNode(m) => root_node = Some(m),
                HkxSceneVisitor::SelectionSets(m) => selection_sets = Some(m),
                HkxSceneVisitor::Cameras(m) => cameras = Some(m),
                HkxSceneVisitor::Lights(m) => lights = Some(m),
                HkxSceneVisitor::Meshes(m) => meshes = Some(m),
                HkxSceneVisitor::Materials(m) => materials = Some(m),
                HkxSceneVisitor::InplaceTextures(m) => inplace_textures = Some(m),
                HkxSceneVisitor::ExternalTextures(m) => external_textures = Some(m),
                HkxSceneVisitor::SkinBindings(m) => skin_bindings = Some(m),
                HkxSceneVisitor::AppliedTransform(m) => applied_transform = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            modeller: modeller.unwrap_or_default().into_inner(),
            asset: asset.unwrap_or_default().into_inner(),
            scene_length: scene_length.unwrap_or_default().into_inner(),
            root_node: root_node.unwrap_or_default().into_inner(),
            selection_sets: selection_sets.unwrap_or_default(),
            cameras: cameras.unwrap_or_default(),
            lights: lights.unwrap_or_default(),
            meshes: meshes.unwrap_or_default(),
            materials: materials.unwrap_or_default(),
            inplace_textures: inplace_textures.unwrap_or_default(),
            external_textures: external_textures.unwrap_or_default(),
            skin_bindings: skin_bindings.unwrap_or_default(),
            applied_transform: applied_transform.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkxScene<'a>> for Vec<HkxSceneVisitor<'a>> {
    fn from(data: &HkxScene<'a>) -> Self {
        vec![
            HkxSceneVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkxSceneVisitor::ReferenceCount(data.reference_count.into()),
            HkxSceneVisitor::Modeller(data.modeller.clone().into()),
            HkxSceneVisitor::Asset(data.asset.clone().into()),
            HkxSceneVisitor::SceneLength(data.scene_length.into()),
            HkxSceneVisitor::RootNode(data.root_node.clone().into()),
            HkxSceneVisitor::SelectionSets(data.selection_sets.clone()),
            HkxSceneVisitor::Cameras(data.cameras.clone()),
            HkxSceneVisitor::Lights(data.lights.clone()),
            HkxSceneVisitor::Meshes(data.meshes.clone()),
            HkxSceneVisitor::Materials(data.materials.clone()),
            HkxSceneVisitor::InplaceTextures(data.inplace_textures.clone()),
            HkxSceneVisitor::ExternalTextures(data.external_textures.clone()),
            HkxSceneVisitor::SkinBindings(data.skin_bindings.clone()),
            HkxSceneVisitor::AppliedTransform(data.applied_transform.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkxScene<'de> {
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
enum HkxSceneVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "modeller")]
    Modeller(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "asset")]
    Asset(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "sceneLength")]
    SceneLength(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "rootNode")]
    RootNode(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "selectionSets")]
    SelectionSets(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "cameras")]
    Cameras(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "lights")]
    Lights(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "meshes")]
    Meshes(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "materials")]
    Materials(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "inplaceTextures")]
    InplaceTextures(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "externalTextures")]
    ExternalTextures(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "skinBindings")]
    SkinBindings(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "appliedTransform")]
    AppliedTransform(Primitive<Matrix3<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxSceneVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("modeller" => Modeller(Primitive<Cow<'de, str>>)),
    ("asset" => Asset(Primitive<Cow<'de, str>>)),
    ("sceneLength" => SceneLength(Primitive<f32>)),
    ("rootNode" => RootNode(Primitive<Cow<'de, str>>)),
    ("selectionSets" => SelectionSets(HkArrayRef<Cow<'de, str>>)),
    ("cameras" => Cameras(HkArrayRef<Cow<'de, str>>)),
    ("lights" => Lights(HkArrayRef<Cow<'de, str>>)),
    ("meshes" => Meshes(HkArrayRef<Cow<'de, str>>)),
    ("materials" => Materials(HkArrayRef<Cow<'de, str>>)),
    ("inplaceTextures" => InplaceTextures(HkArrayRef<Cow<'de, str>>)),
    ("externalTextures" => ExternalTextures(HkArrayRef<Cow<'de, str>>)),
    ("skinBindings" => SkinBindings(HkArrayRef<Cow<'de, str>>)),
    ("appliedTransform" => AppliedTransform(Primitive<Matrix3<f32>>)),
}
