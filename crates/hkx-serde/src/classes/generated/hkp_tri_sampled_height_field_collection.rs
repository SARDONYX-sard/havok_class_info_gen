//! A Rust structure that implements a serializer/deserializer corresponding to `hkpTriSampledHeightFieldCollection`, a class defined in C++
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
/// -    size: 64
/// -  vtable: true
/// -  parent: hkpShapeCollection/`e8c3991d`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpTriSampledHeightFieldCollection<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpTriSampledHeightFieldCollection"`: Name of this class.
    #[serde(default = "HkpTriSampledHeightFieldCollection::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xc291ddde`: Unique value of this class.
    #[serde(default = "HkpTriSampledHeightFieldCollection::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpTriSampledHeightFieldCollectionHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpTriSampledHeightFieldCollectionHkParam<'a>>
}

impl HkpTriSampledHeightFieldCollection<'_> {
    /// Return `"hkpTriSampledHeightFieldCollection"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpTriSampledHeightFieldCollection".into()
    }

    /// Return `"0xc291ddde"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xc291ddde".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpTriSampledHeightFieldCollectionHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"heightfield"`
    /// -   type: `struct hkpSampledHeightFieldShape*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "heightfield")]
    Heightfield(Box<HkpSampledHeightFieldShape>),
    /// # Information on fields in the original C++ class
    /// -   name:`"childSize"`
    /// -   type: `hkInt32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "childSize", skip_serializing)]
    ChildSize(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"radius"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "radius")]
    Radius(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"weldingInfo"`
    /// -   type: `hkArray&lt;hkUint16&gt;`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "weldingInfo")]
    WeldingInfo(Vec<u16>),
    /// # Information on fields in the original C++ class
    /// -   name:`"triangleExtrusion"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "triangleExtrusion")]
    TriangleExtrusion(cgmath::Vector4<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpTriSampledHeightFieldCollectionHkParam<'de>, "@name",
    ("heightfield" => Heightfield(Box<HkpSampledHeightFieldShape>)),
    ("childSize" => ChildSize(i32)),
    ("radius" => Radius(f64)),
    ("weldingInfo" => WeldingInfo(Vec<u16>)),
    ("triangleExtrusion" => TriangleExtrusion(cgmath::Vector4<f32>)),
}
