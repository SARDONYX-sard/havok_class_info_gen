pub fn is_copyable(input: &str) -> bool {
    matches!(
        input,
        "bool"
            | "char"
            | "i8"
            | "i16"
            | "i32"
            | "i64"
            | "isize"
            | "u8"
            | "u16"
            | "u32"
            | "u64"
            | "usize"
            | "f32"
            | "f64"
            | "()"
            | "Vector3<f32>"
            | "Vector4<f32>"
            | "Matrix3<f32>"
            | "Matrix4<f32>"
    )
}
