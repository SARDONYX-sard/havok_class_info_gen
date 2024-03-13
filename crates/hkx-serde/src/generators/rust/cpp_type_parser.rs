use convert_case::{Case, Casing};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{char, digit1, space1},
    combinator::{map, map_res, opt},
};
use std::{borrow::Cow, path::Path};

/// # Examples
/// -   means: `IResult<Remain, ParsedStr>`
/// - example: `IResult<&str, Cow<'_, str>>`
type IResult<I, O, E = nom::error::VerboseError<I>> = Result<(I, O), nom::Err<E>>;

/// C++ type to Rust type conversion
pub fn parse_cpp_type(input: &str) -> IResult<&str, Cow<'_, str>> {
    match input {
        input if input.starts_with("struct") => parse_struct_type(input),
        input if input.starts_with("enum") => parse_enum_type(input),
        input if input.starts_with("flag") => parse_flags_type(input),
        input if input.ends_with('*') => Ok(("", "Cow<'a, str>".into())),
        input => alt((
            parse_array_type,
            parse_struct_type,
            parse_hk_array_type,
            parse_primitive_type,
            parse_vector,
        ))(input),
    }
}

fn parse_primitive_type(input: &str) -> IResult<&str, Cow<'_, str>> {
    map(
        alt((
            map(tag("char*"), |_| "Primitive<Cow<'a, str>>"),
            map(tag("hkBool"), |_| "Primitive<bool>"),
            map(tag("hkChar"), |_| "Primitive<char>"),
            map(tag("hkHalf"), |_| "Primitive<f32>"), // f16
            map(tag("hkInt16"), |_| "Primitive<i16>"),
            map(tag("hkInt32"), |_| "Primitive<i32>"),
            map(tag("hkInt8"), |_| "Primitive<i8>"),
            map(tag("hkReal"), |_| "Primitive<f32>"), // C++ float
            map(tag("hkUint16"), |_| "Primitive<u16>"),
            map(tag("hkUint32"), |_| "Primitive<u32>"),
            map(tag("hkUint64"), |_| "Primitive<u64>"),
            map(tag("hkUint8"), |_| "Primitive<u8>"),
            map(tag("hkUlong"), |_| "Primitive<usize>"),
            map(tag("hkStringPtr"), |_| "Primitive<Cow<'a, str>>"),
            map(tag("hkVariant"), |_| "Primitive<u64>"), // Fill in appropriate type for Variant
            map(tag("void"), |_| "()"),
        )),
        Cow::from,
    )(input)
}

fn parse_vector(input: &str) -> IResult<&str, Cow<'_, str>> {
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
fn parse_array_type(input: &str) -> IResult<&str, Cow<'_, str>> {
    let (input, base_type) = alt((parse_struct_type, parse_primitive_type))(input)?;

    fn parse_array_len(input: &str) -> IResult<&str, usize> {
        let (input, _) = tag("[")(input)?;
        let (input, dimensions) = map_res(digit1, str::parse)(input)?;
        let (input, _) = tag("]")(input)?;
        Ok((input, dimensions))
    }
    let (input, dimensions) = parse_array_len(input)?;
    Ok((input, format!("[{}; {}]", base_type, dimensions).into()))
}

fn parse_generics(input: &str) -> IResult<&str, Cow<'_, str>> {
    // NOTE: struct pointer in generics are not prefixed.
    let (input, generics) = alt((parse_cpp_type, parse_generics_struct_type))(input)?;
    Ok((input, generics))
}

/// Convert to [`Vec`] since `hkArray` has no length limit.
fn parse_hk_array_type(input: &str) -> IResult<&str, Cow<'_, str>> {
    let (input, _) = alt((tag("hkArray"), tag("hkSimpleArray")))(input)?;
    let (input, _) = tag("&lt;")(input)?;
    let (input, generics) = take_while(|c| c != '&')(input)?;
    let (_, generics) = parse_generics(generics)?;
    let (input, _) = tag("&gt;")(input)?;

    Ok((input, format!("Vec<{generics}>",).into()))
}

fn parse_enum_type(input: &str) -> IResult<&str, Cow<'_, str>> {
    let (input, _) = tag("enum")(input)?;
    let (input, _) = space1(input)?;
    Ok(("", input.to_case(Case::Pascal).into()))
}

fn parse_generics_struct_type(input: &str) -> IResult<&str, Cow<'_, str>> {
    parse_struct_type_core(input, true)
}

fn parse_struct_type(input: &str) -> IResult<&str, Cow<'_, str>> {
    parse_struct_type_core(input, false)
}

fn parse_struct_type_core(input: &str, is_generics: bool) -> IResult<&str, Cow<'_, str>> {
    let mut input = input;
    if !is_generics {
        let res = tag("struct")(input)?;
        let res = space1(res.0)?;
        input = res.0;
    }

    let (input, struct_name) = take_while(|c| c != '[' && c != '*')(input)?;
    let struct_name = struct_name.to_case(convert_case::Case::Pascal);
    let (input, is_ptr) = opt(char('*'))(input)?;

    if is_ptr.is_some() {
        Ok((input, "Cow<'a, str>".into()))
    } else {
        Ok((input, struct_name.into()))
    }
}

/// enum bit flags
fn parse_flags_type(input: &str) -> IResult<&str, Cow<'_, str>> {
    let (input, _) = tag("flags")(input)?;
    let (input, _) = space1(input)?;
    Ok(("", input.to_case(Case::Pascal).into()))
}

/// Generate rust code that mapping between C++ and rust types.
pub fn generate_all_mapping_types(rpt_dir: impl AsRef<Path>) -> String {
    let mut types = std::collections::HashMap::new();
    for entry in jwalk::WalkDir::new(rpt_dir).into_iter() {
        let path = entry.unwrap().path();
        if !path.is_file() {
            continue;
        }

        let input = std::fs::read_to_string(&path).unwrap();
        let input = input.as_str();
        match crate::parse_rpt::parse_class(input) {
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
        "//! This file is generated by `crate/src/generators/generated/cpp_type_parser.rs`
//! Please do not edit this file.

#[rustfmt::skip]
/// Mapping tuple between C++ Havok alias type-inclusive types and Rust types.
pub const HK_TYPES: [(&str, &str); {types_len}] = {types:#?};"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generators::rust::generated::hk_types::HK_TYPES;
    use crate::helpers::tracing::init_tracing;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_parse_array() {
        let input = "hkArray&lt;BSBoneSwitchGeneratorBoneData*&gt;";
        let (_, rust_array) = parse_hk_array_type(input).unwrap();
        assert_eq!(rust_array, "Vec<Box<BsBoneSwitchGeneratorBoneData>>");
    }

    #[test]
    fn should_parse_c_style_array() {
        let input = "struct hkpVehicleFrictionStatusAxisStatus[2]";
        let (_, rust_array) = parse_array_type(input).unwrap();
        assert_eq!(rust_array, "[HkpVehicleFrictionStatusAxisStatus; 2]");
    }

    #[test]
    fn should_parse_all_type() {
        let _guard = init_tracing(Some("should_parse_all_type"), tracing::Level::DEBUG).unwrap();

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
    fn should_generate_all_mapping_types() {
        let _guard = init_tracing(None, tracing::Level::DEBUG);

        let rpt_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("assets")
            .join("hkxcmd_help")
            .join("rpt");

        let output_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("generators")
            .join("rust")
            .join("generated");
        std::fs::create_dir_all(&output_dir).unwrap();
        let output_file = output_dir.join("hk_types.rs");

        std::fs::write(output_file, generate_all_mapping_types(rpt_dir)).unwrap();
    }
}
