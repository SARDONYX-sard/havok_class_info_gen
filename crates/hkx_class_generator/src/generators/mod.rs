//! # Havok Classes => Rust Code
//!
//! - It may be called in `lib` or `test`, but not elsewhere.
//! - Not used except to generate Rust code.
// subs
pub mod aliases;
pub mod class_params;
pub mod lifetime_manager;
pub mod utils;

pub mod all_class;
pub mod one_class;

pub use all_class::generate_classes;
