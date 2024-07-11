//! # Havok Types
//!
//! - Only used in the `classes` directory.
//! - No other use for it.
mod c_style_array;
mod hk_array;
mod matrix3;
mod matrix4;
mod primitive;
mod qs_transform;
mod quaternion;
mod single_class;
mod transform;
mod vector3;
mod vector4;

pub use c_style_array::*;
pub use hk_array::*;
pub use matrix3::Matrix3;
pub use matrix4::Matrix4;
pub use primitive::Primitive;
pub use qs_transform::QsTransform;
pub use quaternion::Quaternion;
pub use single_class::SingleClass;
pub use transform::Transform;
pub use vector3::Vector3;
pub use vector4::Vector4;

/// Representing a rotation
pub type Rotation<S> = Matrix3<S>;

///3rd party libraries. Generic type for use with auto-generated classes.
pub use quick_xml::impl_deserialize_for_internally_tagged_enum;
pub use serde::{Deserialize, Serialize};
pub use std::borrow::Cow;
