//! A Rust structure that implements a serializer/deserializer corresponding to `BSSynchronizedClipGenerator`, a class defined in C++
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
/// -    size: 256
/// -  vtable: true
/// -  parent: hkbGenerator/`d68aefc`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct BsSynchronizedClipGenerator<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"BSSynchronizedClipGenerator"`: The original C++ class name.
    #[serde(default = "BsSynchronizedClipGenerator::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xd83bea64`: Unique value of this class.
    #[serde(default = "BsSynchronizedClipGenerator::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<BsSynchronizedClipGeneratorHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<BsSynchronizedClipGeneratorHkParam<'a>>
}

impl BsSynchronizedClipGenerator<'_> {
    /// Return `"BSSynchronizedClipGenerator"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "BSSynchronizedClipGenerator".into()
    }

    /// Return `"0xd83bea64"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xd83bea64".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsSynchronizedClipGeneratorHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"pClipGenerator"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE | ALIGN16`
    #[serde(rename = "pClipGenerator")]
    PClipGenerator(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"SyncAnimPrefix"`
    /// -   type: `char*`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "SyncAnimPrefix")]
    SyncAnimPrefix(Primitive<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"bSyncClipIgnoreMarkPlacement"`
    /// -   type: `hkBool`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bSyncClipIgnoreMarkPlacement")]
    BSyncClipIgnoreMarkPlacement(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"fGetToMarkTime"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fGetToMarkTime")]
    FGetToMarkTime(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"fMarkErrorThreshold"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fMarkErrorThreshold")]
    FMarkErrorThreshold(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"bLeadCharacter"`
    /// -   type: `hkBool`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bLeadCharacter")]
    BLeadCharacter(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"bReorientSupportChar"`
    /// -   type: `hkBool`
    /// - offset: 69
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bReorientSupportChar")]
    BReorientSupportChar(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"bApplyMotionFromRoot"`
    /// -   type: `hkBool`
    /// - offset: 70
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bApplyMotionFromRoot")]
    BApplyMotionFromRoot(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"pSyncScene"`
    /// -   type: `void*`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "pSyncScene", skip_serializing)]
    PSyncScene(()),
    /// # Field information in the original C++ class
    /// -   name:`"StartMarkWS"`
    /// -   type: `hkQsTransform`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "StartMarkWS", skip_serializing)]
    StartMarkWs(QsTransform<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"EndMarkWS"`
    /// -   type: `hkQsTransform`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "EndMarkWS", skip_serializing)]
    EndMarkWs(QsTransform<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"StartMarkMS"`
    /// -   type: `hkQsTransform`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "StartMarkMS", skip_serializing)]
    StartMarkMs(QsTransform<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"fCurrentLerp"`
    /// -   type: `hkReal`
    /// - offset: 224
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "fCurrentLerp", skip_serializing)]
    FCurrentLerp(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"pLocalSyncBinding"`
    /// -   type: `void*`
    /// - offset: 228
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "pLocalSyncBinding", skip_serializing)]
    PLocalSyncBinding(()),
    /// # Field information in the original C++ class
    /// -   name:`"pEventMap"`
    /// -   type: `void*`
    /// - offset: 232
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "pEventMap", skip_serializing)]
    PEventMap(()),
    /// # Field information in the original C++ class
    /// -   name:`"sAnimationBindingIndex"`
    /// -   type: `hkInt16`
    /// - offset: 236
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sAnimationBindingIndex")]
    SAnimationBindingIndex(Primitive<i16>),
    /// # Field information in the original C++ class
    /// -   name:`"bAtMark"`
    /// -   type: `hkBool`
    /// - offset: 238
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "bAtMark", skip_serializing)]
    BAtMark(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"bAllCharactersInScene"`
    /// -   type: `hkBool`
    /// - offset: 239
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "bAllCharactersInScene", skip_serializing)]
    BAllCharactersInScene(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"bAllCharactersAtMarks"`
    /// -   type: `hkBool`
    /// - offset: 240
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "bAllCharactersAtMarks", skip_serializing)]
    BAllCharactersAtMarks(Primitive<bool>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    BsSynchronizedClipGeneratorHkParam<'de>, "@name",
    ("pClipGenerator" => PClipGenerator(Cow<'a, str>)),
    ("SyncAnimPrefix" => SyncAnimPrefix(Primitive<Cow<'a, str>>)),
    ("bSyncClipIgnoreMarkPlacement" => BSyncClipIgnoreMarkPlacement(Primitive<bool>)),
    ("fGetToMarkTime" => FGetToMarkTime(Primitive<f32>)),
    ("fMarkErrorThreshold" => FMarkErrorThreshold(Primitive<f32>)),
    ("bLeadCharacter" => BLeadCharacter(Primitive<bool>)),
    ("bReorientSupportChar" => BReorientSupportChar(Primitive<bool>)),
    ("bApplyMotionFromRoot" => BApplyMotionFromRoot(Primitive<bool>)),
    ("pSyncScene" => PSyncScene(())),
    ("StartMarkWS" => StartMarkWs(QsTransform<f32>)),
    ("EndMarkWS" => EndMarkWs(QsTransform<f32>)),
    ("StartMarkMS" => StartMarkMs(QsTransform<f32>)),
    ("fCurrentLerp" => FCurrentLerp(Primitive<f32>)),
    ("pLocalSyncBinding" => PLocalSyncBinding(())),
    ("pEventMap" => PEventMap(())),
    ("sAnimationBindingIndex" => SAnimationBindingIndex(Primitive<i16>)),
    ("bAtMark" => BAtMark(Primitive<bool>)),
    ("bAllCharactersInScene" => BAllCharactersInScene(Primitive<bool>)),
    ("bAllCharactersAtMarks" => BAllCharactersAtMarks(Primitive<bool>)),
}
