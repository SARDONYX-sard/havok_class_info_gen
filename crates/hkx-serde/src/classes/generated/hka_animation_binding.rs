//! A Rust structure that implements a serializer/deserializer corresponding to `hkaAnimationBinding`, a class defined in C++
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::hk_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// In XML, it is enclosed in a `hkobject` tag
/// and the `class` attribute contains the C++ class nam
///
/// # Information on the original C++ class
/// -    size: 44
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkaAnimationBinding<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkaAnimationBinding"`: The original C++ class name.
    #[serde(default = "HkaAnimationBinding::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x66eac971`: Unique value of this class.
    #[serde(default = "HkaAnimationBinding::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkaAnimationBindingHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkaAnimationBindingHkParam<'a>>
}

impl HkaAnimationBinding<'_> {
    /// Return `"hkaAnimationBinding"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkaAnimationBinding".into()
    }

    /// Return `"0x66eac971"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x66eac971".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaAnimationBindingHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"originalSkeletonName"`
    /// -   type: `hkStringPtr`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "originalSkeletonName")]
    OriginalSkeletonName(Primitive<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"animation"`
    /// -   type: `struct hkaAnimation*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "animation")]
    Animation(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"transformTrackToBoneIndices"`
    /// -   type: `hkArray&lt;hkInt16&gt;`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transformTrackToBoneIndices")]
    TransformTrackToBoneIndices(Vec<Primitive<i16>>),
    /// # Field information in the original C++ class
    /// -   name:`"floatTrackToFloatSlotIndices"`
    /// -   type: `hkArray&lt;hkInt16&gt;`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "floatTrackToFloatSlotIndices")]
    FloatTrackToFloatSlotIndices(Vec<Primitive<i16>>),
    /// # Field information in the original C++ class
    /// -   name:`"blendHint"`
    /// -   type: `enum BlendHint`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "blendHint")]
    BlendHint(BlendHint),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkaAnimationBindingHkParam<'de>, "@name",
    ("originalSkeletonName" => OriginalSkeletonName(Primitive<Cow<'a, str>>)),
    ("animation" => Animation(Cow<'a, str>)),
    ("transformTrackToBoneIndices" => TransformTrackToBoneIndices(Vec<Primitive<i16>>)),
    ("floatTrackToFloatSlotIndices" => FloatTrackToFloatSlotIndices(Vec<Primitive<i16>>)),
    ("blendHint" => BlendHint(BlendHint)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum BlendHint {
    #[serde(rename = "NORMAL")]
    Normal = 0,
    #[serde(rename = "ADDITIVE")]
    Additive = 1,
}
