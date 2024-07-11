//! Hkx XML file Root Element
use super::generated::class_params::ClassParams;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Hkx file Root
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkpackfile")]
pub struct Hkx<'a> {
    /// In the case of SkyrimSE, `8`.
    #[serde(rename = "@classversion")]
    pub class_version: i32,

    /// In the case of SkyrimSE, `"hk_2010.2.0-r1"`.
    #[serde(rename = "@contentsversion")]
    pub content_version: Cow<'a, str>,
    #[serde(rename = "@toplevelobject")]

    /// e.g. `#0056`
    pub top_level_object: Cow<'a, str>,

    /// data sections
    #[serde(rename = "hksection")]
    #[serde(bound(deserialize = "HkSection<'a>: Deserialize<'de>"))]
    pub hk_section: HkSection<'a>,
}

/// section name & class vector
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hksection")]
pub struct HkSection<'a> {
    /// e.g. `"__data__"`
    #[serde(rename = "@name", borrow, skip_deserializing)]
    #[serde(default = "default_section_root")]
    pub name: Cow<'a, str>,

    /// C++ class vector for the `hkobject` tag
    #[serde(bound(deserialize = "Vec<Class<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkobject", borrow)]
    pub classes: Vec<Class<'a>>,
}

/// Return `"__data__"`
fn default_section_root() -> Cow<'static, str> {
    "__data__".into()
}

/// In XML, it is enclosed in a `hkobject` tag
/// and the `class` attribute contains the C++ class name
///
///# Note
/// This `Class` [`serde::Serialize`]/[`serde::Deserialize`] is automatically generated by generated::class_params.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Class<'a> {
    /// e.g. `#0106`
    ///
    /// In XML, these names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    pub name: Name,

    /// Name of each C++ class.
    ///
    /// e.g. `"hkbBehaviorGraphStringData"`
    pub class: Cow<'a, str>,

    /// Signature of each class.
    ///
    /// e.g. `0xc713064e`
    ///
    /// #Note
    /// In some cases, the signature of the parent class is inherited, in which case it is duplicated.
    /// Because of the possibility of duplication, use the class name instead of the signature for the serializer/deserializer.
    pub signature: Signature,

    /// The `"hkparam"` tag (C++ fields) vector
    pub hkparams: ClassParams<'a>,
}

impl<'a> From<Vec<ClassParams<'a>>> for HkSection<'a> {
    fn from(value: Vec<ClassParams<'a>>) -> Self {
        let mut classes = Vec::new();
        for (index, param) in value.into_iter().enumerate() {
            classes.push(Class {
                name: Name::new(index + 50),
                class: param.class_name().into(),
                signature: Signature::new(param.signature()),
                hkparams: param,
            });
        }

        HkSection {
            name: "__data__".into(),
            classes,
        }
    }
}

/// Havok C++ Class unique number.
///
/// # Example
/// - `#457`
/// - `#007`
#[repr(transparent)]
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, derive_new::new)]
pub struct Name(usize);

impl core::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:04}", self.0)
    }
}

impl From<usize> for Name {
    fn from(value: usize) -> Self {
        Self::new(value)
    }
}

impl TryFrom<&str> for Name {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self::new(
            parse_int::parse(value.trim_start_matches('#'))
                .map_err(|_| "Invalid integer for Name")?,
        ))
    }
}

/// Havok C++ Class signature hex number.
///
/// # Example
/// - `0x13a39ba7`
#[repr(transparent)]
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, derive_new::new)]
pub struct Signature(u32);

impl core::fmt::Display for Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // NOTE: `0x` is counted length. Therefore, non use alternative(`{:#}`)
        write!(f, "0x{:08x}", self.0)
    }
}

impl From<u32> for Signature {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

impl TryFrom<&str> for Signature {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self::new(
            parse_int::parse(value).map_err(|_| "Invalid integer for Signature")?,
        ))
    }
}

/// - Use [`core::fmt::Display`] for [`serde::Serialize`].
/// - Use `parse_int` for [`serde::DeSerialize`].
macro_rules! impl_serde {
    ($struct_name:ident) => {
        impl Serialize for $struct_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(&self.to_string())
            }
        }

        impl<'de> Deserialize<'de> for $struct_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let s: &str = Deserialize::deserialize(deserializer)?;
                let num = s.try_into().map_err(|_| {
                    serde::de::Error::custom(format!(
                        "Invalid integer for {}",
                        stringify!($struct_name)
                    ))
                })?;
                Ok(num)
            }
        }
    };
}

impl_serde!(Signature);
impl_serde!(Name);