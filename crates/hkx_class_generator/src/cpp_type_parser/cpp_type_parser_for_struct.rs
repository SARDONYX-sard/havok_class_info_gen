//! C++ type => Rust type non wrapper conversion
//!
//! # Example
//! `hkUint16` -> `i16`
use super::cpp_type_parser_for_xml::{parse_struct_type, parse_vector};
use convert_case::{Case, Casing};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{digit1, space1},
    combinator::{map, map_res},
};
use std::borrow::Cow;

/// # Examples
/// -   means: `IResult<Remain, ParsedStr>`
/// - example: `IResult<&str, Cow<'_, str>>`
type IResult<I, O, E = nom::error::VerboseError<I>> = Result<(I, O), nom::Err<E>>;

/// C++ type to Rust type conversion (Non wrapper type)
///
/// # Example
/// `hkUint16` -> `i16`
pub fn parse_cpp_type_to_non_wrapper(input: &str) -> IResult<&str, Cow<'_, str>> {
    match input {
        "char*" | "hkBool" | "hkChar" | "hkHalf" | "hkInt16" | "hkInt32" | "hkInt8" | "hkReal"
        | "hkUint16" | "hkUint32" | "hkUint64" | "hkUint8" | "hkUlong" | "hkStringPtr"
        | "hkVariant" | "void" => parse_primitive_type_non_wrapper(input),

        // ! In quick_xml, vector4 (`<tag>(0.0 0.0 0.0)</tag>`) enclosed in a single tag must be enclosed in a structure.
        "hkMatrix3" | "hkMatrix4" | "hkQsTransform" | "hkQuaternion" | "hkRotation"
        | "hkTransform" | "hkVector4" => {
            let (input, vec_type) = parse_vector(input)?;
            Ok((input, format!("Primitive<{vec_type}>").into()))
        }

        input if input.ends_with('*') => Ok(("", "Cow<'a, str>".into())),

        input if input.starts_with("hkArray&lt;") || input.starts_with("hkSimpleArray&lt;") => {
            parse_hk_array_to_non_wrapper(input)
        }

        // `unknown` means that the information does not exist and is passed over with `()` and ignored.
        // All `unknown` type C++ class fields have `SKIP_SERIALIZE` flag, so this operation may be safe.
        "flags unknown" | "enum unknown" => Ok(("", "()".into())),

        input if input.starts_with("enum") => parse_enum_to_non_wrapper(input),
        input if input.starts_with("flag") => parse_flags_to_non_wrapper(input),
        input if input.ends_with(']') => parse_array_to_non_wrapper(input),
        input => {
            let (input, struct_name) = parse_struct_type(input)?;
            Ok((input, struct_name))
        }
    }
}

/// primitive non wrapper
///
/// Need a wrapper structure to get the value directly under the value tag in quick_xml,
/// but you don't need a wrapper for the fixed array `[T; N]` in CStyle.
pub fn parse_primitive_type_non_wrapper(input: &str) -> IResult<&str, Cow<'_, str>> {
    map(
        alt((
            map(tag("hkBool"), |_| "bool"),
            map(tag("hkChar"), |_| "char"),
            //
            map(tag("hkInt8"), |_| "i8"),
            map(tag("hkInt16"), |_| "i16"),
            map(tag("hkInt32"), |_| "i32"),
            //
            map(tag("hkHalf"), |_| "f32"), // f16
            map(tag("hkReal"), |_| "f32"), // C++ float
            //
            map(tag("hkUint8"), |_| "u8"),
            map(tag("hkUint16"), |_| "u16"),
            map(tag("hkUint32"), |_| "u32"),
            map(tag("hkUint64"), |_| "u64"),
            map(tag("hkUlong"), |_| "usize"),
            //
            map(tag("char*"), |_| "Cow<'a, str>"),
            map(tag("hkStringPtr"), |_| "Cow<'a, str>"),
            //
            map(tag("hkVariant"), |_| "u64"), // Fill in appropriate type for Variant
            map(tag("void"), |_| "()"),
        )),
        Cow::from,
    )(input)
}

/// Has limit array. like `int[3]` to `[i32; 3]`
fn parse_array_to_non_wrapper(input: &str) -> IResult<&str, Cow<'_, str>> {
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

    let array_type = format!("[{base_type}; {size}]");
    Ok((input, array_type.into()))
}

fn parse_hk_array_to_non_wrapper(input: &str) -> IResult<&str, Cow<'_, str>> {
    let (input, _) = alt((tag("hkArray&lt;"), tag("hkSimpleArray&lt;")))(input)?;
    let (input, generics) = take_while(|c| c != '&')(input)?;

    let (_, array_type) = match generics {
        "char*" | "hkBool" | "hkChar" | "void" => {
            let (input, v) = parse_primitive_type_non_wrapper(generics)?;
            Ok((input, format!("Vec<{v}>").into()))
        }

        "hkHalf" | "hkInt16" | "hkInt32" | "hkInt8" | "hkReal" | "hkUint16" | "hkUint32"
        | "hkUint64" | "hkUint8" | "hkUlong" | "hkVariant" => {
            let (input, v) = parse_primitive_type_non_wrapper(generics)?;
            Ok((input, format!("Vec<{v}>").into()))
        }

        "hkStringPtr" => Ok((input, "Vec<Cow<'a, str>>".into())),

        "hkMatrix3" | "hkQsTransform" | "hkRotation" => {
            let (input, v) = parse_vector(generics)?;
            Ok((input, format!("Vec<{v}>").into()))
        }

        "hkMatrix4" | "hkTransform" => {
            let (input, v) = parse_vector(generics)?;
            Ok((input, format!("Vec<{v}>").into()))
        }

        "hkQuaternion" | "hkVector4" => {
            let (input, v) = parse_vector(generics)?;
            Ok((input, format!("Vec<{v}>").into()))
        }

        s if s.ends_with('*') => Ok(("", "Vec<Cow<'a, str>>".into())),

        s if s.starts_with("enum") => {
            let (input, e) = parse_enum_to_non_wrapper(s)?;
            Ok((input, format!("Vec<{e}>").into()))
        }

        s if s.starts_with("flag") => {
            let (input, e) = parse_flags_to_non_wrapper(s)?;
            Ok((input, format!("Vec<{e}>").into()))
        }

        any => {
            let (input, class) = parse_struct_type(any)?;
            Ok((input, format!("Vec<{class}>").into()))
        }
    }?;

    let (input, _) = tag("&gt;")(input)?;
    Ok((input, array_type))
}

fn parse_enum_to_non_wrapper(input: &str) -> IResult<&str, Cow<'_, str>> {
    let (input, _) = tag("enum")(input)?;
    let (input, _) = space1(input)?;
    Ok(("", input.to_case(Case::Pascal).into()))
}

/// enum bit flags
fn parse_flags_to_non_wrapper(input: &str) -> IResult<&str, Cow<'_, str>> {
    let (input, _) = tag("flags")(input)?;
    let (input, _) = space1(input)?;
    Ok(("", input.to_case(Case::Pascal).into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Generate rust code that mapping between C++ and rust non wrapper types.
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
                        let (_, rust_type) = parse_cpp_type_to_non_wrapper(&type_name).unwrap();
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

    #[test]
    #[quick_tracing::init]
    fn should_generate_all_mapping_types() {
        let rpt_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("assets")
            .join("hkxcmd_help")
            .join("rpt");

        let output_file = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("cpp_type_parser")
            .join("generated")
            .join("types_for_struct_fields.rs");
        std::fs::write(output_file, generate_all_mapping_types(rpt_dir)).unwrap();
    }
}
