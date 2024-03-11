// mod generated;
mod hkb_behavior_graph_string_data;
mod hkb_variable_value;
mod hkb_variable_value_set;
mod root;

pub mod hkx_vertex_description;
pub mod hkx_vertex_description_element_decl;

use self::hkb_behavior_graph_string_data::HkbBehaviorGraphStringData;
use self::hkb_variable_value::HkbVariableValue;
use self::hkb_variable_value_set::HkbVariableValueSet;
use crate::impl_deserialize_for_internally_tagged_enum;
use serde::Serialize;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@signature")]
enum AllClass<'a> {
    #[serde(bound(deserialize = "HkbVariableValueSet<'a>: Deserialize<'de>"))]
    HkbVariableValueSet(HkbVariableValueSet<'a>),
    #[serde(rename = "decls")]
    #[serde(bound(deserialize = "HkbVariableValue<'a>: Deserialize<'de>"))]
    HkbVariableValue(HkbVariableValue<'a>),
    #[serde(bound(deserialize = "HkbBehaviorGraphStringData<'a>: Deserialize<'de>"))]
    HkbBehaviorGraphStringData(HkbBehaviorGraphStringData<'a>),
}

impl_deserialize_for_internally_tagged_enum! {
    AllClass<'de>, "@signature",
    ("0x27812d8d" => HkbVariableValueSet(HkbVariableValueSet<'de>)),
    ("0xb99bd6a" => HkbVariableValue(HkbVariableValue<'de>)),
    ("0xc713064e" => HkbBehaviorGraphStringData(HkbBehaviorGraphStringData<'de>)),
}
