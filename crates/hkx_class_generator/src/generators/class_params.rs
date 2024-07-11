use super::{
    aliases::{ClassMap, LifeTimeMap},
    lifetime_manager::get_lifetime_from_fields,
    one_class::{enum_tagged::tagged_fields::generate_tagged_fields, generate_all_fields},
};
use convert_case::{Case, Casing as _};

/// Generate all havok C++ class enums.
///
/// Pass `{ key: class name, value: class information }` mapping as information.
///
/// The reason we don't take a signature is that this is not a unique value.
///
/// (**It may inherit the signature of the parent class**. e.g. `hkpShapeContainer`, `baseObject`, etc.)
pub fn generate_class_params(class_map: &ClassMap, life_time_map: &LifeTimeMap) -> String {
    let mut class_params_code = String::new();

    let mut enum_variants_code = String::new();
    let mut serialize_match_inner_code = String::new();
    let mut deserialize_match_inner_code = String::new();
    let mut bytes_deserialize_match_inner_code = String::new();
    let mut name_match_inner_code = String::new();
    let mut signature_match_inner_code = String::new();

    // The number of loops is reduced to one by generating the code inside first.
    for (cpp_class_name, class) in class_map {
        #[cfg(debug_assertions)]
        if !matches!(
            class.name.as_str(),
            "hkRootLevelContainer"
                | "hkRootLevelContainerNamedVariant" // depended by `hkRootLevelContainer`
                | "hkbProjectStringData"
                | "hkbProjectData"
                | "hkbTransitionEffect" // For enum EventMode(depended by `hkbProjectData`)
        ) {
            continue;
        }

        let signature = class.signature;
        let rust_enum_name = class.name.to_case(Case::Pascal);

        let (_rust_fields_code, fields) = generate_all_fields(
            class,
            class_map,
            Some(life_time_map),
            generate_tagged_fields,
        );
        let life_time = get_lifetime_from_fields(&fields);
        let rust_class_name_with_life_time = format!("{rust_enum_name}{life_time}");
        // Why use Box?
        // Because some structures are large in size, use Box to avoid wasteful use of memory, although heap cost may cause delays.
        let life_time_bound = if rust_class_name_with_life_time.ends_with("<'a>") {
            format!("\n    #[serde(bound(deserialize = \"Box<{rust_class_name_with_life_time}>: Deserialize<'de>\"))]")
        } else {
            "".into()
        };

        enum_variants_code.push_str(&format!(
            r#"    #[serde(rename = "{signature:#x}")]{life_time_bound}
    {rust_enum_name}(Box<{rust_class_name_with_life_time}>),

"#
        ));

        name_match_inner_code.push_str(&format!(
            r#"            "ClassParams::{rust_enum_name}(_) => "{cpp_class_name}"
"#
        ));
        signature_match_inner_code.push_str(&format!(
            r#"            "ClassParams::{rust_enum_name}(_) => {signature:#x}
"#
        ));

        serialize_match_inner_code.push_str(&format!(
            r#"
            "{cpp_class_name}" => {{
                if let ClassParams::{rust_enum_name}(ref params) = self.hkparams {{
                    state.serialize_field("hkparam", params)?;
                }}
            }}
"#
        ));

        deserialize_match_inner_code.push_str(&format!(
            r#"                                    "{cpp_class_name}" => {{
                                                        ClassParams::{rust_enum_name}(map.next_value()?)

                                    }},
"#
        ));

        bytes_deserialize_match_inner_code.push_str(&format!(
            r#"            "{cpp_class_name}" => ClassParams::{rust_enum_name}(Box::new(
                {rust_enum_name}::from_bytes::<B>(bytes, de)?,
            )),
"#
        ));
    }

    class_params_code.push_str(&generate_class_params_enum(&enum_variants_code));

    class_params_code.push_str(&generate_impl_self(
        &name_match_inner_code,
        &signature_match_inner_code,
    ));

    class_params_code.push_str(&generate_impl_serialize(
        &serialize_match_inner_code,
        class_map.len(),
    ));
    class_params_code.push_str(&generate_impl_deserialize(&deserialize_match_inner_code));
    class_params_code.push_str(&generate_bytes_deserialize(
        &bytes_deserialize_match_inner_code,
    ));

    class_params_code
}

/// Generates `ClassParams` enum.
///
/// This is the set of types of `hkparam` (fields of C++ havok class).
fn generate_class_params_enum(variants_code: &str) -> String {
    let mut class_params_code = String::new();

    class_params_code.push_str(
        r#"//! The type of enumeration of all C++ havok class fields.
use super::*;
use crate::bytes::*;
use crate::classes::{Class, Name, Signature};
use crate::error::{HkxError, Result};
use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::{Deserialize, Serialize};
use std::borrow::Cow

/// Pattern enumeration of all C++ havok class fields.
///
/// In XML, these are the fields of the attribute `hkparam`
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Default, Clone, PartialEq, Serialize)]
#[serde(tag = "@signature")]
pub enum ClassParams<'a> {
    #[default]
    Unknown,

"#,
    );

    class_params_code.push_str(variants_code);
    class_params_code.push('}');

    class_params_code
}

/// Generate name getter & signature getter.
fn generate_impl_self(name_match_inner_code: &str, signature_match_inner_code: &str) -> String {
    format!(
        r##"
impl ClassParams<'_> {{
    pub fn class_name(&self) -> &'static str {{
        match &self {{
            ClassParams::Unknown => "Unknown",
{name_match_inner_code}
        }}
    }}

    pub fn signature(&self) -> u32 {{
        match &self {{
            ClassParams::Unknown => 0x0,
{signature_match_inner_code}
        }}
    }}
}}
"##
    )
}

/// Generate `impl Serialize` code
fn generate_impl_serialize(serialize_match_inner_code: &str, class_map_len: usize) -> String {
    let mut class_params = String::new();

    class_params.push_str(&format!(
        r#"
impl<'a> Serialize for Class<'a> {{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {{
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("Class", {class_map_len})?;
        state.serialize_field("@name", &self.name)?;
        state.serialize_field("@class", &self.class)?;
        state.serialize_field("@signature", &self.signature)?;

        // Serialize hkparam based on class(C++ class name)
        match self.class.as_ref() {{
"#
    ));

    class_params.push_str(serialize_match_inner_code);

    class_params.push_str(
        r#"
            _ => {}
        }
        state.end()
    }
}
"#,
    );

    class_params
}

/// Generate `impl Deserialize` code
fn generate_impl_deserialize(deserialize_match_inner_code: &str) -> String {
    let mut class_params = String::new();

    class_params.push_str(
        r#"
// # Note
// In [`quick_xml::impl_deserialize_for_internally_tagged_enum`], only the first attribute can be deserialized by tag.
// What we need this time is the third attribute, "signature". Therefore, we need to deserialize it on our own initiative.
impl<'de> Deserialize<'de> for Class<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ClassVisitor;

        impl<'de> Visitor<'de> for ClassVisitor {
            type Value = Class<'de>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Class")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut name: Option<Cow<'de, str>> = None;
                let mut class: Option<Cow<'de, str>> = None;
                let mut signature: Option<Cow<'de, str>> = None;
                let mut hkparam: Option<ClassParams<'de>> = None;

                while let Some(key) = map.next_key::<Cow<'_, str>>()? {
                    match key.as_ref() {
                        "@name" => {
                            if name.is_some() {
                                return Err(de::Error::duplicate_field("@name"));
                            }
                            name = Some(map.next_value()?);
                        }
                        "@class" => {
                            if class.is_some() {
                                return Err(de::Error::duplicate_field("@class"));
                            }
                            class = Some(map.next_value()?);
                        }
                        "@signature" => {
                            if signature.is_some() {
                                return Err(de::Error::duplicate_field("@signature"));
                            }
                            signature = Some(map.next_value()?);
                        }
                        "hkparam" => {
                            if let Some(ref class_name) = class {
                                hkparam = Some(Ok(match class_name.as_ref() {
"#,
    );

    class_params.push_str(deserialize_match_inner_code);

    class_params.push_str(
        r#"
                                    unknown => {
                                        return Err(de::Error::custom(format!(
                                            "Unexpected value {unknown}"
                                        )))
                                    }
                                })?);
                            } else {
                                return Err(de::Error::custom("Processing an array of `hkparam` requires identification by `class` attribute first, but non exist"));
                            }
                        }
                        _ => {
                            let _: serde::de::IgnoredAny = map.next_value()?;
                        }
                    }
                }

                let name = name.ok_or_else(|| de::Error::missing_field("@name"))?;
                let class = class.ok_or_else(|| de::Error::missing_field("@class"))?;
                let signature = signature.ok_or_else(|| de::Error::missing_field("@signature"))?;
                let hkparams = hkparam.ok_or_else(|| de::Error::missing_field("hkparam"))?;

                Ok(Class {
                    name,
                    class,
                    signature,
                    hkparams,
                })
            }
        }

        deserializer.deserialize_map(ClassVisitor)
    }
}
"#,
    );

    class_params
}

/// Generate `impl decimalize bytes` code
fn generate_bytes_deserialize(bytes_deserialize_match_inner_code: &str) -> String {
    let mut class_params = String::new();

    class_params.push_str(
        r#"
impl<'a> ClassParams<'a> {
    /// Read the contents of hkparam by C++ havok Class name and create a data structure for rust.
    ///
    /// # Assumptions
    /// - The starting point of `bytes` must be the binary data position of the fields
    ///   of the class(`class_name`) to be deserialized.
    pub fn from_class_name_and_bytes<B>(
        class_name: &str,
        bytes: &'a [u8],
        de: &mut PackFileDeserializer<'a>,
    ) -> Result<Self>
    where
        B: ByteOrder,
    {
        Ok(match class_name {
"#,
    );

    class_params.push_str(bytes_deserialize_match_inner_code);

    // Epilogue
    class_params.push_str(
        r#"
            unknown => return Err(HkxError::UnknownHavokClass(unknown.into())),
        })
    }
}
"#,
    );

    class_params
}
