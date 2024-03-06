//! A Rust structure that implements a serializer/deserializer corresponding to `hkbVariableValueSet`, a class defined in C++
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use self::hkb_variable_value::HkbVariableValueHkParam;
use super::*;
use crate::havok_types::array::Vector4;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{de::IntoDeserializer, Deserialize, Serialize};
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

    /// `"hkbVariableValueSet"`: Name of this class.
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
    /// Return `"hkbVariableValueSet"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbVariableValueSet".into()
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
    Quad(HkArrayValue<Vector4<f32>>),
    /// # Information on fields in the original C++ class
    /// -   name:`"variantVariableValues"`
    /// -   type: `hkArray&lt;hkReferencedObject*&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "variantVariableValues")]
    Variant(HkArrayRef<String>),
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HkArrayClass<T> {
    /// Length of the class
    #[serde(rename = "@numelements")]
    pub numelements: usize,
    #[serde(rename = "hkobject")]
    classes: Vec<Class<T>>,
}

/// One class of `HkArray`
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Class<T> {
    #[serde(rename = "hkparam")]
    hkparam: T,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HkArrayRef<T> {
    #[serde(rename = "@numelements")]
    pub numelements: usize,
    #[serde(rename = "$text")]
    value: Vec<T>,
}

#[derive(Debug, Default, PartialEq, Serialize)]
pub struct HkArrayValue<T> {
    #[serde(rename = "@numelements")]
    pub numelements: usize,
    #[serde(rename = "$text")]
    value: Vec<T>,
}

impl<'de, T> Deserialize<'de> for HkArrayValue<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct HkArrayValueVisitor<T> {
            marker: std::marker::PhantomData<T>,
        }

        impl<'de, T> serde::de::Visitor<'de> for HkArrayValueVisitor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = HkArrayValue<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct HkArrayValue")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                use serde::de::Error;
                let mut numelements = None;
                let mut value = None;

                while let Some(key) = map.next_key::<Cow<'_, str>>()? {
                    match key.as_bytes() {
                        b"@numelements" => {
                            if numelements.is_some() {
                                return Err(Error::duplicate_field("@numelements"));
                            }
                            numelements = Some(map.next_value()?);
                        }
                        b"$text" => {
                            let text: Cow<'_, str> = map.next_value()?;
                            dbg!(&text);
                            let mut value_inner = Vec::new();

                            for line in text.split(['(']).filter(|t| !t.is_empty()) {
                                value_inner.push(T::deserialize(line.into_deserializer())?);
                            }
                            value = Some(value_inner);
                        }
                        unknown => {
                            let content: Cow<'_, str> = map.next_value()?;
                            dbg!("Got unknown field. key: {}, value: {}", unknown, content);
                        }
                    }
                }

                let numelements =
                    numelements.ok_or_else(|| Error::missing_field("@numelements"))?;
                let value = value.unwrap_or_default();

                Ok(HkArrayValue { numelements, value })
            }
        }

        deserializer.deserialize_map(HkArrayValueVisitor {
            marker: std::marker::PhantomData,
        })
    }
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbVariableValueSetHkParam, "@name",
    ("wordVariableValues" => Word(HkArrayClass<HkbVariableValueHkParam>)),
    ("quadVariableValues" => Quad(HkArrayValue<Vector4<f32>>)),
    ("variantVariableValues" => Variant(HkArrayRef<String>)),
}

#[cfg(test)]
mod tests {
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
                        Class {
                            hkparam: HkbVariableValueHkParam::Value(1045220557),
                        },
                        Class {
                            hkparam: HkbVariableValueHkParam::Value(0),
                        },
                        Class {
                            hkparam: HkbVariableValueHkParam::Value(0),
                        },
                    ],
                }),
                HkbVariableValueSetHkParam::Quad(HkArrayValue {
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

        dbg!(result);
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
        (0.000000 0.000000 0.000000 0.000000)
        (0.000000 0.000000 0.000000 0.000000)
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
                class: "HkbVariableValueSet".into(),
                signature: "0x27812d8d".into(),
                hkparams: vec![
                    HkbVariableValueSetHkParam::Word(HkArrayClass {
                        numelements: 3,
                        classes: vec![
                            Class {
                                hkparam: HkbVariableValueHkParam::Value(1045220557,),
                            },
                            Class {
                                hkparam: HkbVariableValueHkParam::Value(0,)
                            },
                            Class {
                                hkparam: HkbVariableValueHkParam::Value(0,)
                            },
                        ],
                    },),
                    HkbVariableValueSetHkParam::Quad(HkArrayValue {
                        numelements: 2,
                        value: vec![
                            (0.000000, 0.000000, 0.000000, 0.000000).into(),
                            (0.000000, 0.000000, 0.000000, 0.000000).into()
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
