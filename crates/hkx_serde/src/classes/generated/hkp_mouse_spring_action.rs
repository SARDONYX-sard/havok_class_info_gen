//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpMouseSpringAction`
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

/// `hkpMouseSpringAction`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 96
/// -    vtable: true
/// -    parent: `hkpUnaryAction`/`0x895532c0`
/// - signature: `0x6e087fd6`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpMouseSpringAction<'a> {
    /// # C++ Parent class(`hkpUnaryAction` => parent: `hkpAction`) field Info
    /// -   name:`"entity"`
    /// -   type: `struct hkpEntity*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub entity: Cow<'a, str>,

    /// # C++ Parent class(`hkpAction` => parent: `hkReferencedObject`) field Info
    /// -   name:`"world"`
    /// -   type: `void*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub world: Cow<'a, str>,
    /// # C++ Parent class(`hkpAction` => parent: `hkReferencedObject`) field Info
    /// -   name:`"island"`
    /// -   type: `void*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub island: Cow<'a, str>,
    /// # C++ Parent class(`hkpAction` => parent: `hkReferencedObject`) field Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub user_data: usize,
    /// # C++ Parent class(`hkpAction` => parent: `hkReferencedObject`) field Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub name: Cow<'a, str>,

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
    /// -   name:`"positionInRbLocal"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub position_in_rb_local: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"mousePositionInWorld"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub mouse_position_in_world: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"springDamping"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub spring_damping: f32,
    /// # C++ Class Fields Info
    /// -   name:`"springElasticity"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    pub spring_elasticity: f32,
    /// # C++ Class Fields Info
    /// -   name:`"maxRelativeForce"`
    /// -   type: `hkReal`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    pub max_relative_force: f32,
    /// # C++ Class Fields Info
    /// -   name:`"objectDamping"`
    /// -   type: `hkReal`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    pub object_damping: f32,
    /// # C++ Class Fields Info
    /// -   name:`"shapeKey"`
    /// -   type: `hkUint32`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub shape_key: u32,
    /// # C++ Class Fields Info
    /// -   name:`"applyCallbacks"`
    /// -   type: `hkArray<void*>`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub apply_callbacks: HkArrayRef<Cow<'a, str>>,
}

impl Serialize for HkpMouseSpringAction<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpMouseSpringActionVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpMouseSpringAction<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpMouseSpringActionVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpMouseSpringActionVisitor<'a>>> for HkpMouseSpringAction<'a> {
    fn from(_values: Vec<HkpMouseSpringActionVisitor<'a>>) -> Self {
            let mut entity = None;
            let mut world = None;
            let mut island = None;
            let mut user_data = None;
            let mut name = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut position_in_rb_local = None;
            let mut mouse_position_in_world = None;
            let mut spring_damping = None;
            let mut spring_elasticity = None;
            let mut max_relative_force = None;
            let mut object_damping = None;
            let mut shape_key = None;
            let mut apply_callbacks = None;


        for _value in _values {
            match _value {
                HkpMouseSpringActionVisitor::Entity(m) => entity = Some(m),
                HkpMouseSpringActionVisitor::World(m) => world = Some(m),
                HkpMouseSpringActionVisitor::Island(m) => island = Some(m),
                HkpMouseSpringActionVisitor::UserData(m) => user_data = Some(m),
                HkpMouseSpringActionVisitor::Name(m) => name = Some(m),
                HkpMouseSpringActionVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpMouseSpringActionVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpMouseSpringActionVisitor::PositionInRbLocal(m) => position_in_rb_local = Some(m),
                HkpMouseSpringActionVisitor::MousePositionInWorld(m) => mouse_position_in_world = Some(m),
                HkpMouseSpringActionVisitor::SpringDamping(m) => spring_damping = Some(m),
                HkpMouseSpringActionVisitor::SpringElasticity(m) => spring_elasticity = Some(m),
                HkpMouseSpringActionVisitor::MaxRelativeForce(m) => max_relative_force = Some(m),
                HkpMouseSpringActionVisitor::ObjectDamping(m) => object_damping = Some(m),
                HkpMouseSpringActionVisitor::ShapeKey(m) => shape_key = Some(m),
                HkpMouseSpringActionVisitor::ApplyCallbacks(m) => apply_callbacks = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            entity: entity.unwrap_or_default().into_inner(),
            world: world.unwrap_or_default().into_inner(),
            island: island.unwrap_or_default().into_inner(),
            user_data: user_data.unwrap_or_default().into_inner(),
            name: name.unwrap_or_default().into_inner(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            position_in_rb_local: position_in_rb_local.unwrap_or_default().into_inner(),
            mouse_position_in_world: mouse_position_in_world.unwrap_or_default().into_inner(),
            spring_damping: spring_damping.unwrap_or_default().into_inner(),
            spring_elasticity: spring_elasticity.unwrap_or_default().into_inner(),
            max_relative_force: max_relative_force.unwrap_or_default().into_inner(),
            object_damping: object_damping.unwrap_or_default().into_inner(),
            shape_key: shape_key.unwrap_or_default().into_inner(),
            apply_callbacks: apply_callbacks.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpMouseSpringAction<'a>> for Vec<HkpMouseSpringActionVisitor<'a>> {
    fn from(data: &HkpMouseSpringAction<'a>) -> Self {
        vec![
            HkpMouseSpringActionVisitor::Entity(data.entity.clone().into()),
            HkpMouseSpringActionVisitor::World(data.world.clone().into()),
            HkpMouseSpringActionVisitor::Island(data.island.clone().into()),
            HkpMouseSpringActionVisitor::UserData(data.user_data.into()),
            HkpMouseSpringActionVisitor::Name(data.name.clone().into()),
            HkpMouseSpringActionVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpMouseSpringActionVisitor::ReferenceCount(data.reference_count.into()),
            HkpMouseSpringActionVisitor::PositionInRbLocal(data.position_in_rb_local.into()),
            HkpMouseSpringActionVisitor::MousePositionInWorld(data.mouse_position_in_world.into()),
            HkpMouseSpringActionVisitor::SpringDamping(data.spring_damping.into()),
            HkpMouseSpringActionVisitor::SpringElasticity(data.spring_elasticity.into()),
            HkpMouseSpringActionVisitor::MaxRelativeForce(data.max_relative_force.into()),
            HkpMouseSpringActionVisitor::ObjectDamping(data.object_damping.into()),
            HkpMouseSpringActionVisitor::ShapeKey(data.shape_key.into()),
            HkpMouseSpringActionVisitor::ApplyCallbacks(data.apply_callbacks.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpMouseSpringAction<'de> {
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
enum HkpMouseSpringActionVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "entity")]
    Entity(Primitive<Cow<'a, str>>),

    /// Visitor fields
    #[serde(rename = "world", skip_serializing)]
    World(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "island", skip_serializing)]
    Island(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "userData")]
    UserData(Primitive<usize>),
    /// Visitor fields
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),

    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "positionInRbLocal")]
    PositionInRbLocal(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "mousePositionInWorld")]
    MousePositionInWorld(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "springDamping")]
    SpringDamping(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "springElasticity")]
    SpringElasticity(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "maxRelativeForce")]
    MaxRelativeForce(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "objectDamping")]
    ObjectDamping(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "shapeKey")]
    ShapeKey(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "applyCallbacks", skip_serializing)]
    ApplyCallbacks(HkArrayRef<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpMouseSpringActionVisitor<'de>, "@name",
    ("entity" => Entity(Primitive<Cow<'de, str>>)),
    ("world" => World(Primitive<Cow<'de, str>>)),
    ("island" => Island(Primitive<Cow<'de, str>>)),
    ("userData" => UserData(Primitive<usize>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("positionInRbLocal" => PositionInRbLocal(Primitive<Vector4<f32>>)),
    ("mousePositionInWorld" => MousePositionInWorld(Primitive<Vector4<f32>>)),
    ("springDamping" => SpringDamping(Primitive<f32>)),
    ("springElasticity" => SpringElasticity(Primitive<f32>)),
    ("maxRelativeForce" => MaxRelativeForce(Primitive<f32>)),
    ("objectDamping" => ObjectDamping(Primitive<f32>)),
    ("shapeKey" => ShapeKey(Primitive<u32>)),
    ("applyCallbacks" => ApplyCallbacks(HkArrayRef<Cow<'de, str>>)),
}
