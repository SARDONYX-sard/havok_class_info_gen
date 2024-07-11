/// Enumerator to determine which C++ type from enum
use core::fmt;
use core::str::FromStr;
use num_derive::{FromPrimitive, ToPrimitive};

/// NOTE: The enum usize must be in this order and cannot be added or removed in between.
#[derive(Debug, Clone, Default, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum Type {
    #[default]
    /// No type
    Void = 0,
    /// hkBool, boolean type
    Bool,
    /// hkChar, signed char type
    Char,
    /// hkInt8, 8 bit signed integer type
    Int8,
    /// hkUint8, 8 bit unsigned integer type
    Uint8,
    /// hkInt16, 16 bit signed integer type
    Int16,
    /// hkUint16, 16 bit unsigned integer type
    Uint16,
    /// hkInt32, 32 bit signed integer type
    Int32,
    /// hkUint32, 32 bit unsigned integer type
    Uint32,
    /// hkInt64, 64 bit signed integer type
    Int64,
    /// hkUint64, 64 bit unsigned integer type
    Uint64,
    /// hkReal, float type
    Real,
    /// hkVector4 type
    Vector4,
    /// hkQuaternion type
    Quaternion,
    /// hkMatrix3 type
    Matrix3,
    /// hkRotation type
    Rotation,
    /// hkQsTransform type
    QsTransform,
    /// hkMatrix4 type
    Matrix4,
    /// hkTransform type
    Transform,
    /// Serialize as zero - deprecated.
    Zero,
    /// Generic pointer, see member flags for more info
    Pointer,
    /// Function pointer
    FnPtr,
    /// hkArray<T>, array of items of type T
    Array,
    /// hkInplaceArray<T,N> or hkInplaceArrayAligned16<T,N>, array of N items of type T
    InplaceArray,
    /// hkEnum<ENUM,STORAGE> - enumerated values
    Enum,
    /// Object
    Struct,
    /// Simple array (ptr(typed) and size only)
    SimpleArray,
    /// Simple array of homogeneous types, so is a class id followed by a void* ptr and size
    HomogeneousArray,
    /// hkVariant (void* and hkClass*) type
    Variant,
    /// char*, null terminated string
    CString,
    /// hkUlong, unsigned long, defined to always be the same size as a pointer
    Ulong,
    /// hkFlags<ENUM,STORAGE> - 8,16,32 bits of named values.
    Flags,
    /// hkHalf, 16-bit float value
    Half,
    /// hkStringPtr, c-string
    StringPtr,
    /// hkRelArray<>, attached const array values
    RelArray,
    Max,
}

impl Type {
    pub fn as_str(&self) -> &'static str {
        match &self {
            Self::Void => "TYPE_VOID",
            Self::Bool => "TYPE_BOOL",
            Self::Char => "TYPE_CHAR",
            Self::Int8 => "TYPE_INT8",
            Self::Uint8 => "TYPE_UINT8",
            Self::Int16 => "TYPE_INT16",
            Self::Uint16 => "TYPE_UINT16",
            Self::Int32 => "TYPE_INT32",
            Self::Uint32 => "TYPE_UINT32",
            Self::Int64 => "TYPE_INT64",
            Self::Uint64 => "TYPE_UINT64",
            Self::Real => "TYPE_REAL",
            Self::Vector4 => "TYPE_VECTOR4",
            Self::Quaternion => "TYPE_QUATERNION",
            Self::Matrix3 => "TYPE_MATRIX3",
            Self::Rotation => "TYPE_ROTATION",
            Self::QsTransform => "TYPE_QSTRANSFORM",
            Self::Matrix4 => "TYPE_MATRIX4",
            Self::Transform => "TYPE_TRANSFORM",
            Self::Zero => "TYPE_ZERO",
            Self::Pointer => "TYPE_POINTER",
            Self::FnPtr => "TYPE_FUNCTIONPOINTER",
            Self::Array => "TYPE_ARRAY",
            Self::InplaceArray => "TYPE_INPLACEARRAY",
            Self::Enum => "TYPE_ENUM",
            Self::Struct => "TYPE_STRUCT",
            Self::SimpleArray => "TYPE_SIMPLEARRAY",
            Self::HomogeneousArray => "TYPE_HOMOGENEOUSARRAY",
            Self::Variant => "TYPE_VARIANT",
            Self::CString => "TYPE_CSTRING",
            Self::Ulong => "TYPE_ULONG",
            Self::Flags => "TYPE_FLAGS",
            Self::Half => "TYPE_HALF",
            Self::StringPtr => "TYPE_STRINGPTR",
            Self::RelArray => "TYPE_RELARRAY",
            Self::Max => "TYPE_MAX",
        }
    }
}

impl FromStr for Type {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "TYPE_VOID" => Self::Void,
            "TYPE_BOOL" => Self::Bool,
            "TYPE_CHAR" => Self::Char,
            "TYPE_INT8" => Self::Int8,
            "TYPE_UINT8" => Self::Uint8,
            "TYPE_INT16" => Self::Int16,
            "TYPE_UINT16" => Self::Uint16,
            "TYPE_INT32" => Self::Int32,
            "TYPE_UINT32" => Self::Uint32,
            "TYPE_INT64" => Self::Int64,
            "TYPE_UINT64" => Self::Uint64,
            "TYPE_REAL" => Self::Real,
            "TYPE_VECTOR4" => Self::Vector4,
            "TYPE_QUATERNION" => Self::Quaternion,
            "TYPE_MATRIX3" => Self::Matrix3,
            "TYPE_ROTATION" => Self::Rotation,
            "TYPE_QSTRANSFORM" => Self::QsTransform,
            "TYPE_MATRIX4" => Self::Matrix4,
            "TYPE_TRANSFORM" => Self::Transform,
            "TYPE_ZERO" => Self::Zero,
            "TYPE_POINTER" => Self::Pointer,
            "TYPE_FUNCTIONPOINTER" => Self::FnPtr,
            "TYPE_ARRAY" => Self::Array,
            "TYPE_INPLACEARRAY" => Self::InplaceArray,
            "TYPE_ENUM" => Self::Enum,
            "TYPE_STRUCT" => Self::Struct,
            "TYPE_SIMPLEARRAY" => Self::SimpleArray,
            "TYPE_HOMOGENEOUSARRAY" => Self::HomogeneousArray,
            "TYPE_VARIANT" => Self::Variant,
            "TYPE_CSTRING" => Self::CString,
            "TYPE_ULONG" => Self::Ulong,
            "TYPE_FLAGS" => Self::Flags,
            "TYPE_HALF" => Self::Half,
            "TYPE_STRINGPTR" => Self::StringPtr,
            "TYPE_RELARRAY" => Self::RelArray,
            invalid_type => return Err(format!("Invalid type: {}", invalid_type)),
        })
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // NOTE: `From<Self> for &str` must not be called
        // because an infinite cycle will occur if [into] is called.
        write!(f, "{}", self.as_str())
    }
}

impl From<Type> for &str {
    fn from(value: Type) -> Self {
        value.as_str()
    }
}

impl From<Type> for usize {
    fn from(value: Type) -> Self {
        value as usize
    }
}

impl TryFrom<usize> for Type {
    type Error = String;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        num_traits::FromPrimitive::from_usize(value)
            .ok_or_else(|| format!("Invalid type: {}", value))
    }
}

impl serde::Serialize for Type {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for Type {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Type::from_str(s).map_err(serde::de::Error::custom)
    }
}
