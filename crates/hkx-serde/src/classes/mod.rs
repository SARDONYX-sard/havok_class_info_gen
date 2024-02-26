// mod generated;
mod hkb_behavior_graph_string_data;
mod hkb_variable_value_set;

use crate::error::Result;
use roxmltree::Node;

pub trait Deserializer {
    fn deserialize<'a>(node: &Node<'a, 'a>) -> Result<Self>
    where
        Self: Sized;
}
