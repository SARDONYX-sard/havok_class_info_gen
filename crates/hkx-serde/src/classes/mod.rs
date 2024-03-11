// mod generated;
mod hkb_behavior_graph_string_data;
mod hkb_variable_value;
mod hkb_variable_value_set;
mod root;

pub mod hkx_vertex_description;
pub mod hkx_vertex_description_element_decl;

use std::borrow::Cow;

use self::hkb_behavior_graph_string_data::HkbBehaviorGraphStringDataHkparam;
use self::hkb_variable_value::HkbVariableValueHkParam;
use self::hkb_variable_value_set::HkbVariableValueSetHkParam;
use crate::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct Class<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    #[serde(default)]
    pub name: Cow<'a, str>,

    /// `"hkbVariableValueSet"`: Name of this C++ class.
    #[serde(rename = "@class", borrow, skip_deserializing)]
    pub class: Cow<'a, str>,

    /// `0x27812d8d`: Unique value of this class.
    #[serde(rename = "@signature", borrow, skip_deserializing)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<ClassParams<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<ClassParams<'a>>,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@signature")]
pub enum ClassParams<'a> {
    HkbVariableValueSet(HkbVariableValueSetHkParam<'a>),
    #[serde(rename = "decls")]
    HkbVariableValue(HkbVariableValueHkParam),
    HkbBehaviorGraphStringData(HkbBehaviorGraphStringDataHkparam<'a>),
}

impl_deserialize_for_internally_tagged_enum! {
    ClassParams<'de>, "@signature",
    ("0x27812d8d" => HkbVariableValueSet(HkbVariableValueSetHkParam<'de>)),
    ("0xb99bd6a" => HkbVariableValue(HkbVariableValueHkParam)),
    ("0xc713064e" => HkbBehaviorGraphStringData(HkbBehaviorGraphStringDataHkparam<'de>)),
}
