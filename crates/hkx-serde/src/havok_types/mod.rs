mod float;
mod hk_array;
mod matrix3;
mod matrix4;
mod qs_transform;
mod quaternion;
mod str;
mod transform;
mod vector3;
mod vector4;

pub use hk_array::{HkArrayClass, HkArrayClassParam, HkArrayRef, HkArrayVector};
pub use matrix3::Matrix3;
pub use matrix4::Matrix4;
pub use qs_transform::QsTransform;
pub use quaternion::Quaternion;
pub use transform::Transform;
pub use vector3::Vector3;
pub use vector4::Vector4;

/// Representing a rotation
pub type Rotation<S> = Matrix3<S>;

pub fn string(text: &str) -> Option<&str> {
    const NULL_CHAR: &str = "\u{2400}";
    match text == NULL_CHAR {
        true => None,
        false => Some(str::trim(text)),
    }
}
