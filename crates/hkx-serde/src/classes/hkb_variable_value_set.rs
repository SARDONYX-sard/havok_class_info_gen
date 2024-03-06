//! A Rust structure that implements a serializer/deserializer corresponding to `hkbVariableValueSet`, a class defined in C++
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::hkb_variable_value::HkbVariableValueHkParam;
use crate::havok_types::{
    hk_array::{HkArrayClass, HkArrayRef, HkArrayVector},
    Vector4,
};
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
/// - version: 0
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbVariableValueSet<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbVariableValueSet"`: Name of this C++ class.
    #[serde(default = "HkbVariableValueSet::class_name")]
    #[serde(rename = "@class", borrow, skip_deserializing)]
    pub class: Cow<'a, str>,

    /// `0x27812d8d`: Unique value of this class.
    #[serde(default = "HkbVariableValueSet::signature")]
    #[serde(rename = "@signature", borrow, skip_deserializing)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbVariableValueSetHkParam>,
}

impl Default for HkbVariableValueSet<'_> {
    fn default() -> Self {
        Self {
            name: Default::default(),
            class: Self::class_name(),
            signature: Self::signature(),
            hkparams: Default::default(),
        }
    }
}

impl HkbVariableValueSet<'_> {
    /// Return `"hkbVariableValueSet"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbVariableValueSet".into()
    }

    /// Return `"0x27812d8d"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x27812d8d".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbVariableValueSetHkParam {
    /// # Information on fields in the original C++ class
    /// -   name:`"wordVariableValues"`
    /// -   type: `hkArray&lt;struct hkbVariableValue&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wordVariableValues")]
    Word(HkArrayClass<HkbVariableValueHkParam>),
    /// # Information on fields in the original C++ class
    /// -   name:`"quadVariableValues"`
    /// -   type: `hkArray&lt;hkVector4&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "quadVariableValues")]
    Quad(HkArrayVector<Vector4<f32>>),
    /// # Information on fields in the original C++ class
    /// -   name:`"variantVariableValues"`
    /// -   type: `hkArray&lt;hkReferencedObject*&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "variantVariableValues")]
    Variant(HkArrayRef<String>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbVariableValueSetHkParam, "@name",
    ("wordVariableValues" => Word(HkArrayClass<HkbVariableValueHkParam>)),
    ("quadVariableValues" => Quad(HkArrayVector<Vector4<f32>>)),
    ("variantVariableValues" => Variant(HkArrayRef<String>)),
}

#[cfg(test)]
mod tests {
    use crate::havok_types::hk_array::HkArrayClassParam;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize() {
        let result = quick_xml::se::to_string(&HkbVariableValueSet {
            name: "#0064".into(),
            class: "hkbVariableValueSet".into(),
            signature: "0x27812d8d".into(),
            hkparams: vec![
                HkbVariableValueSetHkParam::Word(HkArrayClass {
                    numelements: 3,
                    classes: vec![
                        HkArrayClassParam {
                            hkparam: 1045220557.into(),
                        },
                        HkArrayClassParam { hkparam: 0.into() },
                        HkArrayClassParam { hkparam: 0.into() },
                    ],
                }),
                HkbVariableValueSetHkParam::Quad(HkArrayVector {
                    numelements: 2,
                    value: vec![
                        (63.0, 64.0, 65.0, 66.0).into(),
                        (63.0, 64.0, 65.0, 66.0).into(),
                    ],
                }),
                HkbVariableValueSetHkParam::Variant(HkArrayRef {
                    numelements: 2,
                    value: vec!["#0063".into(), "#0064".into()],
                }),
            ],
        })
        .unwrap();

        dbg!(&result);
    }

    #[test]
    fn should_deserialize() {
        let xml = r###"
<hkobject name="#0064" class="hkbVariableValueSet" signature="0x27812d8d">
    <hkparam name="wordVariableValues" numelements="3">
        <hkobject>
            <hkparam name="value">1045220557</hkparam>
        </hkobject>
        <hkobject>
            <hkparam name="value">0</hkparam>
        </hkobject>
        <hkobject>
            <hkparam name="value">0</hkparam>
        </hkobject>
    </hkparam>
    <hkparam name="quadVariableValues" numelements="2">
        (0.000000 1.000000 0.000000 0.000000)
        (0.000000 0.000000 -1.000000 0.000000)
    </hkparam>
    <hkparam name="variantVariableValues" numelements="2">
        #0063 #0064
    </hkparam>
</hkobject>
"###;

        let result: HkbVariableValueSet = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(
            result,
            HkbVariableValueSet {
                name: "#0064".into(),
                class: "hkbVariableValueSet".into(),
                signature: "0x27812d8d".into(),
                hkparams: vec![
                    HkbVariableValueSetHkParam::Word(HkArrayClass {
                        numelements: 3,
                        classes: vec![
                            HkArrayClassParam {
                                hkparam: 1045220557.into(),
                            },
                            HkArrayClassParam { hkparam: 0.into() },
                            HkArrayClassParam { hkparam: 0.into() },
                        ]
                    },),
                    HkbVariableValueSetHkParam::Quad(HkArrayVector {
                        numelements: 2,
                        value: vec![
                            (0.000000, 1.000000, 0.000000, 0.000000).into(),
                            (0.000000, 0.000000, -1.000000, 0.000000).into()
                        ],
                    },),
                    HkbVariableValueSetHkParam::Variant(HkArrayRef {
                        numelements: 2,
                        value: vec!["#0063".into(), "#0064".into()],
                    },),
                ],
            }
        );
    }
}
