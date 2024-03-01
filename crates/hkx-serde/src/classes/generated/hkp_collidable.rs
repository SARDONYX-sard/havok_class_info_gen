//! A Rust structure that implements a serializer/deserializer corresponding to `hkpCollidable`, a class defined in C++
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
/// -    size: 80
/// -  vtable: false
/// -  parent: hkpCdBody/`54a4b841`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpCollidable<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpCollidable"`: Name of this class.
    #[serde(default = "HkpCollidable::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x9a0e42a5`: Unique value of this class.
    #[serde(default = "HkpCollidable::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpCollidableHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpCollidableHkParam<'a>>
}

impl HkpCollidable<'_> {
    /// Return `"hkpCollidable"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpCollidable".into()
    }

    /// Return `"0x9a0e42a5"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x9a0e42a5".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpCollidableHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"ownerOffset"`
    /// -   type: `hkInt8`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "ownerOffset", skip_serializing)]
    OwnerOffset(i8),
    /// # Information on fields in the original C++ class
    /// -   name:`"forceCollideOntoPpu"`
    /// -   type: `hkUint8`
    /// - offset: 17
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "forceCollideOntoPpu")]
    ForceCollideOntoPpu(u8),
    /// # Information on fields in the original C++ class
    /// -   name:`"shapeSizeOnSpu"`
    /// -   type: `hkUint16`
    /// - offset: 18
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "shapeSizeOnSpu", skip_serializing)]
    ShapeSizeOnSpu(u16),
    /// # Information on fields in the original C++ class
    /// -   name:`"broadPhaseHandle"`
    /// -   type: `struct hkpTypedBroadPhaseHandle`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "broadPhaseHandle")]
    BroadPhaseHandle(HkpTypedBroadPhaseHandle),
    /// # Information on fields in the original C++ class
    /// -   name:`"boundingVolumeData"`
    /// -   type: `struct hkpCollidableBoundingVolumeData`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "boundingVolumeData", skip_serializing)]
    BoundingVolumeData(HkpCollidableBoundingVolumeData),
    /// # Information on fields in the original C++ class
    /// -   name:`"allowedPenetrationDepth"`
    /// -   type: `hkReal`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "allowedPenetrationDepth")]
    AllowedPenetrationDepth(f64),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpCollidableHkParam<'de>, "@name",
    ("ownerOffset" => OwnerOffset(i8)),
    ("forceCollideOntoPpu" => ForceCollideOntoPpu(u8)),
    ("shapeSizeOnSpu" => ShapeSizeOnSpu(u16)),
    ("broadPhaseHandle" => BroadPhaseHandle(HkpTypedBroadPhaseHandle)),
    ("boundingVolumeData" => BoundingVolumeData(HkpCollidableBoundingVolumeData)),
    ("allowedPenetrationDepth" => AllowedPenetrationDepth(f64)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ForceCollideOntoPpuReasons {
    #[serde(rename = "FORCE_PPU_USER_REQUEST")]
    ForcePpuUserRequest = 1,
    #[serde(rename = "FORCE_PPU_SHAPE_REQUEST")]
    ForcePpuShapeRequest = 2,
    #[serde(rename = "FORCE_PPU_MODIFIER_REQUEST")]
    ForcePpuModifierRequest = 4,
    #[serde(rename = "FORCE_PPU_SHAPE_UNCHECKED")]
    ForcePpuShapeUnchecked = 8,
}
