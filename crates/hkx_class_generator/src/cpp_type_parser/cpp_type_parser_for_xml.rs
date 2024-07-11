//! C++ type => Rust type with wrapper conversion
//!
//! To (De)Serialize a Havok C++ Class as XML, the name attribute value must be a class field, so the only way is to use an internally tagged enum.
//! Therefore, the type is wrapped in a wrapper.
//!
//! # Example
//! `hkUint16` -> `Primitive<i16>`
use super::cpp_type_parser_for_struct::parse_primitive_type_non_wrapper;
use convert_case::{Case, Casing};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{char, digit1, space1},
    combinator::{map, map_res, opt},
};
use std::borrow::Cow;

/// # Examples
/// -   means: `IResult<Remain, ParsedStr>`
/// - example: `IResult<&str, Cow<'_, str>>`
type IResult<I, O, E = nom::error::VerboseError<I>> = Result<(I, O), nom::Err<E>>;

/// C++ type to Rust type conversion with wrapper type
///
/// This creates a wrapper type that is needed for the convenience of (de)serialization by quick_xml.
///
/// Therefore, it is used in the `Visitor`.
///
/// # Example
/// `hkUint16` -> `Primitive<i16>`
pub fn parse_cpp_type(input: &str) -> IResult<&str, Cow<'_, str>> {
    match input {
        "char*" | "hkBool" | "hkChar" | "hkHalf" | "hkInt16" | "hkInt32" | "hkInt8" | "hkReal"
        | "hkUint16" | "hkUint32" | "hkUint64" | "hkUint8" | "hkUlong" | "hkStringPtr"
        | "hkVariant" | "void" => parse_primitive_type(input),

        // ! In quick_xml, vector4 (`<tag>(0.0 0.0 0.0)</tag>`) enclosed in a single tag must be enclosed in a structure.
        "hkMatrix3" | "hkMatrix4" | "hkQsTransform" | "hkQuaternion" | "hkRotation"
        | "hkTransform" | "hkVector4" => {
            let (input, vec_type) = parse_vector(input)?;
            Ok((input, format!("Primitive<{vec_type}>").into()))
        }

        input if input.ends_with('*') => Ok(("", "Primitive<Cow<'a, str>>".into())),

        input if input.starts_with("hkArray&lt;") || input.starts_with("hkSimpleArray&lt;") => {
            parse_hk_array_to_wrapper(input)
        }

        // `unknown` means that the information does not exist and is passed over with `()` and ignored.
        // All `unknown` type C++ class fields have `SKIP_SERIALIZE` flag, so this operation may be safe.
        "flags unknown" | "enum unknown" => Ok(("", "Primitive<()>".into())),

        input if input.starts_with("enum") => parse_enum_type(input),
        input if input.starts_with("flag") => parse_flags_type(input),
        input if input.ends_with(']') => parse_array_to_wrapper_type(input),
        input => {
            let (input, struct_name) = parse_struct_type(input)?;
            let struct_name = format!("SingleClass<{struct_name}>");
            Ok((input, struct_name.into()))
        }
    }
}

/// primitive
fn parse_primitive_type(input: &str) -> IResult<&str, Cow<'_, str>> {
    map(
        alt((
            map(tag("hkBool"), |_| "Primitive<bool>"),
            map(tag("hkChar"), |_| "Primitive<char>"),
            //
            map(tag("hkInt8"), |_| "Primitive<i8>"),
            map(tag("hkInt16"), |_| "Primitive<i16>"),
            map(tag("hkInt32"), |_| "Primitive<i32>"),
            //
            map(tag("hkUint8"), |_| "Primitive<u8>"),
            map(tag("hkUint16"), |_| "Primitive<u16>"),
            map(tag("hkUint32"), |_| "Primitive<u32>"),
            map(tag("hkUint64"), |_| "Primitive<u64>"),
            map(tag("hkUlong"), |_| "Primitive<usize>"),
            //
            map(tag("hkHalf"), |_| "Primitive<f32>"), // f16
            map(tag("hkReal"), |_| "Primitive<f32>"), // C++ float
            //
            map(tag("char*"), |_| "Primitive<Cow<'a, str>>"),
            map(tag("hkStringPtr"), |_| "Primitive<Cow<'a, str>>"),
            //
            map(tag("hkVariant"), |_| "Primitive<u64>"), // Fill in appropriate type for Variant
            map(tag("void"), |_| "Primitive<()>"),
        )),
        Cow::from,
    )(input)
}

/// vector.
///
/// # Examples
/// `hkMatrix3` -> `Matrix3<f32>`
pub fn parse_vector(input: &str) -> IResult<&str, Cow<'_, str>> {
    map(
        alt((
            map(tag("hkMatrix3"), |_| "Matrix3<f32>"),
            map(tag("hkMatrix4"), |_| "Matrix4<f32>"),
            map(tag("hkQsTransform"), |_| "QsTransform<f32>"),
            map(tag("hkQuaternion"), |_| "Quaternion<f32>"),
            map(tag("hkRotation"), |_| "Rotation<f32>"),
            map(tag("hkTransform"), |_| "Transform<f32>"),
            map(tag("hkVector4"), |_| "Vector4<f32>"),
        )),
        Cow::from,
    )(input)
}

/// Has limit array. like `[3]`
fn parse_array_to_wrapper_type(input: &str) -> IResult<&str, Cow<'_, str>> {
    fn parse_array_len(input: &str) -> IResult<&str, usize> {
        let (input, _) = tag("[")(input)?;
        let (input, size) = map_res(digit1, str::parse)(input)?;
        let (input, _) = tag("]")(input)?;
        Ok((input, size))
    }

    let (input, base_type) = alt((
        parse_primitive_type_non_wrapper,
        parse_vector,
        parse_struct_type,
    ))(input)?;
    let (input, size) = parse_array_len(input)?;

    let array_type = match base_type.as_ref() {
        "bool" | "char" | "i8" | "i16" | "i32" | "i64" | "isize" | "u8" | "u16" | "u32" | "u64"
        | "usize" | "f32" | "f64" => format!("CStyleArray<[{base_type}; {size}]>").into(),

        vec_type if vec_type.starts_with("Vector4") || vec_type.starts_with("Transform") => {
            format!("CStyleArrayVector<{vec_type}, {size}>").into()
        }
        vec_type
            if vec_type.starts_with("Matrix3")
                || vec_type.starts_with("Rotation")
                || vec_type.starts_with("QsTransform") =>
        {
            format!("CStyleArrayMatrix3<{vec_type}, {size}>").into()
        }
        vec_type if vec_type.starts_with("Matrix4") || vec_type.starts_with("Quaternion") => {
            format!("CStyleArrayMatrix4<{vec_type}, {size}>").into()
        }

        any => format!("CStyleArrayClass<{any}, {size}>").into(),
    };

    Ok((input, array_type))
}

/// Convert to [`Vec`] since `hkArray` has no length limit.
fn parse_hk_array_to_wrapper(input: &str) -> IResult<&str, Cow<'_, str>> {
    let (input, _) = alt((tag("hkArray&lt;"), tag("hkSimpleArray&lt;")))(input)?;
    let (input, generics) = take_while(|c| c != '&')(input)?;

    let (_, array_type) = match generics {
        "char*" | "hkBool" | "hkChar" | "void" => {
            let (input, v) = parse_primitive_type_non_wrapper(generics)?;
            Ok((input, format!("HkArrayRef<{v}>").into()))
        }

        "hkHalf" | "hkInt16" | "hkInt32" | "hkInt8" | "hkReal" | "hkUint16" | "hkUint32"
        | "hkUint64" | "hkUint8" | "hkUlong" | "hkVariant" => {
            let (input, v) = parse_primitive_type_non_wrapper(generics)?;
            Ok((input, format!("HkArrayNum<{v}>").into()))
        }

        "hkStringPtr" => Ok((input, "HkArrayStringPtr<'a>".into())),

        "hkMatrix3" | "hkQsTransform" | "hkRotation" => {
            let (input, v) = parse_vector(generics)?;
            Ok((input, format!("HkArrayMatrix3<{v}>").into()))
        }

        "hkMatrix4" | "hkTransform" => {
            let (input, v) = parse_vector(generics)?;
            Ok((input, format!("HkArrayMatrix4<{v}>").into()))
        }

        "hkQuaternion" | "hkVector4" => {
            let (input, v) = parse_vector(generics)?;
            Ok((input, format!("HkArrayVector<{v}>").into()))
        }

        s if s.ends_with('*') => Ok(("", "HkArrayRef<Cow<'a, str>>".into())),

        s if s.starts_with("enum") => {
            let (input, e) = parse_enum_type(s)?;
            Ok((input, format!("HkArrayPrimitive<{e}>").into()))
        }

        s if s.starts_with("flag") => {
            let (input, e) = parse_flags_type(s)?;
            Ok((input, format!("HkArrayPrimitive<{e}>").into()))
        }

        any => {
            let (input, class) = parse_struct_type(any)?;
            Ok((input, format!("HkArrayClass<{class}>").into()))
        }
    }?;

    let (input, _) = tag("&gt;")(input)?;
    Ok((input, array_type))
}

/// struct
///
/// # Examples
/// - `struct Foo` -> `Foo`
/// - `struct Foo*` -> `Foo`
pub fn parse_struct_type(input: &str) -> IResult<&str, Cow<'_, str>> {
    let (input, _) = opt(tag("struct"))(input)?;
    let (input, struct_name) = take_while(|c| c != '[' && c != '*')(input)?;
    let (input, _is_ptr) = opt(char('*'))(input)?;

    let struct_name = struct_name.to_case(convert_case::Case::Pascal);
    Ok((input, struct_name.into()))
}

/// enum
fn parse_enum_type(input: &str) -> IResult<&str, Cow<'_, str>> {
    let (input, _) = tag("enum")(input)?;
    let (input, _) = space1(input)?;
    Ok((
        "",
        format!("Primitive<{}>", input.to_case(Case::Pascal)).into(),
    ))
}

/// enum bit flags
fn parse_flags_type(input: &str) -> IResult<&str, Cow<'_, str>> {
    let (input, _) = tag("flags")(input)?;
    let (input, _) = space1(input)?;
    Ok((
        "",
        format!("Primitive<{}>", input.to_case(Case::Pascal)).into(),
    ))
}

#[cfg(test)]
/// Generate rust code that mapping between C++ and rust types.
///
/// `rpt_dir`: rpt files dir by `hkxcmd Report ./assets/help`
pub fn generate_all_mapping_types(rpt_dir: impl AsRef<std::path::Path>) -> String {
    let mut types = std::collections::HashMap::new();
    for entry in jwalk::WalkDir::new(rpt_dir).into_iter() {
        let path = entry.unwrap().path();
        if !path.is_file() {
            continue;
        }

        let input = std::fs::read_to_string(&path).unwrap();
        let input = input.as_str();
        match crate::hkxcmd_parser::parse_class(input) {
            Ok((_, class_info)) => {
                for m in class_info.members {
                    let type_name = m.type_name.to_string();
                    let (_, rust_type) = parse_cpp_type(&type_name).unwrap();
                    types.insert(type_name.clone(), rust_type.to_string());
                }
                // tracing::debug!("{:#?}", class_info);
            }
            Err(e) => {
                let e = match e {
                    nom::Err::Incomplete(e) => panic!("{:?}", e),
                    nom::Err::Error(err) | nom::Err::Failure(err) => err,
                };
                let e = format!("Error: {}", nom::error::convert_error(input, e));
                let path = dbg!(path);
                tracing::error!("{}", path.display());
                tracing::error!("{}", e);
                panic!("{}", e)
            }
        }
    }

    let mut types = types.into_iter().collect::<Vec<(String, String)>>();
    types.sort();
    let types_len = types.len();

    format!(
        "//! This file is generated. Please do not edit this file.
#[allow(unused)]
#[rustfmt::skip]
/// Mapping tuple between C++ Havok alias type-inclusive types and Rust types.
pub const HK_TYPES: [(&str, &str); {types_len}] = {types:#?};\n"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpp_type_parser::generated::types_for_xml::HK_TYPES;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_parse_array() {
        let input = "hkArray&lt;BSBoneSwitchGeneratorBoneData*&gt;";
        let (_, rust_array) = parse_hk_array_to_wrapper(input).unwrap();
        assert_eq!(rust_array, "HkArrayRef<Cow<'a, str>>");
    }

    #[test]
    fn should_parse_c_style_array() {
        let input = "struct hkpVehicleFrictionStatusAxisStatus[2]";
        let (_, rust_array) = parse_array_to_wrapper_type(input).unwrap();

        let expected = "CStyleArrayClass<HkpVehicleFrictionStatusAxisStatus, 2>";
        assert_eq!(rust_array, expected);
    }

    #[test]
    #[quick_tracing::init(test = "should_parse_all_type", level = "Debug")]
    fn should_parse_all_type() {
        for (cpp_type, expected_rust_type) in HK_TYPES {
            match parse_cpp_type(cpp_type) {
                Ok((_, actual)) => {
                    tracing::debug!("{cpp_type:<75} -> {actual}");
                    assert_eq!(actual, expected_rust_type);
                }
                Err(err) => {
                    let e = match err {
                        nom::Err::Incomplete(e) => panic!("{:?}", e),
                        nom::Err::Error(err) | nom::Err::Failure(err) => {
                            let err = nom::error::convert_error(cpp_type, err);
                            format!("Error parsing {cpp_type}: {err}",)
                        }
                    };
                    panic!("{}", e)
                }
            }
        }
    }

    #[test]
    #[quick_tracing::init(level = "Debug")]
    fn should_generate_all_mapping_types() {
        let rpt_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("assets")
            .join("hkxcmd_help")
            .join("rpt");

        let output_file = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("cpp_type_parser")
            .join("generated")
            .join("types_for_xml.rs");

        std::fs::write(output_file, generate_all_mapping_types(rpt_dir)).unwrap();
    }
}
