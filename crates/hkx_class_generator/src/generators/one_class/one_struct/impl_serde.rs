use crate::generators::one_class::enum_tagged::ENUM_UNIQUE_NAME;

pub fn generate_impl_serde(rust_struct_name: &str, life_time: &str, fields_len: usize) -> String {
    let mut rust_code = String::new();

    let de_life_time = life_time.replace("<'a>", "<'de>");
    let ser_life_time = life_time.replace("<'a>", "<'_>");

    let struct_ser_with_life_time = format!("{rust_struct_name}{ser_life_time}");
    let struct_de_with_life_time = format!("{rust_struct_name}{de_life_time}");

    let enum_ser_with_life_time = format!("{rust_struct_name}{ENUM_UNIQUE_NAME}{ser_life_time}");
    let enum_de_with_life_time = format!("{rust_struct_name}{ENUM_UNIQUE_NAME}{de_life_time}");

    // serde only supports up to `[T; 32]`, so use `Vec` if it is larger than that.
    let mut enum_array_ser_type = format!("[{rust_struct_name}{ENUM_UNIQUE_NAME}; {fields_len}]");
    let mut enum_array_de_type = format!("[{rust_struct_name}{ENUM_UNIQUE_NAME}; {fields_len}]");
    if fields_len > 0 {
        enum_array_ser_type = format!("Vec<{enum_ser_with_life_time}>");
        enum_array_de_type = format!("Vec<{enum_de_with_life_time}>");
    };

    rust_code.push_str(&format!(
        r#"
impl Serialize for {struct_ser_with_life_time} {{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
        {enum_array_ser_type}::from(self).serialize(serializer)
    }}
}}

impl<'de> Deserialize<'de> for {struct_de_with_life_time} {{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
        Ok(<{enum_array_de_type}>::deserialize(deserializer)?.into())
    }}
}}
"#
    ));

    rust_code
}
