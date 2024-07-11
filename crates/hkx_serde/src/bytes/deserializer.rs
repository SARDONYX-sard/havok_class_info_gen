use super::hkx_header::HkxHeader;
use super::sections::{
    class_name_section::ClassNames, section_contents::SectionContents,
    section_header::SectionHeader,
};
use crate::classes::class_params::ClassParams;
use crate::classes::Hkx;
use crate::error::Result;
use crate::havok_types::*;
use crate::helpers::number::align;
use core::mem::size_of;
use indexmap::IndexMap;
use std::borrow::Cow;
use std::ffi::CStr;
use std::str::from_utf8;
use zerocopy::ByteOrder;

/// Serialize trait for HKX binaries for C++ Havok class.
pub trait ByteSerialize {
    /// As bytes slice(HKX binary) from a instance.
    fn as_bytes(&self) -> Result<&[u8]>;
}

/// Deserialize trait for HKX binaries for C++ Havok class.
pub trait ByteDeSerialize<'de> {
    /// Create a new instance from bytes slice(HKX binary).
    fn from_bytes<D>(deserializer: &'de D, position: &mut u32) -> Result<Self>
    where
        D: ByteDeserializer,
        Self: Sized;
}

pub trait ByteDeserializer {
    /// Aligned by the header pointer size.
    fn align_usize(&self, position: &mut u32);
    fn bytes(&self, position: u32) -> &[u8];

    /// Read ptr size.
    /// - 4 => [`ByteOrder::read_u32`]
    /// - 8 => [`ByteOrder::read_u64`]
    ///
    /// # Move position size
    /// [`usize`] (4 or 8 bytes)
    fn read_usize(&self, position: &mut u32) -> Result<usize>;
    /// Read 1 byte as bool.
    fn read_bool(&self, position: &mut u32) -> Result<bool>;
    fn read_bool_array(&self, position: &mut u32) -> Result<Vec<bool>>;

    // Signed
    fn read_i8(&self, position: &mut u32) -> Result<i8>;
    fn read_i16(&self, position: &mut u32) -> Result<i16>;
    fn read_i32(&self, position: &mut u32) -> Result<i32>;
    fn read_i64(&self, position: &mut u32) -> Result<i64>;
    fn read_i8_array(&self, position: &mut u32) -> Result<Vec<i8>>;
    fn read_i16_array(&self, position: &mut u32) -> Result<Vec<i16>>;
    fn read_i32_array(&self, position: &mut u32) -> Result<Vec<i32>>;
    fn read_i64_array(&self, position: &mut u32) -> Result<Vec<i64>>;

    // Unsigned
    fn read_u8(&self, position: &mut u32) -> Result<u8>;
    fn read_u16(&self, position: &mut u32) -> Result<u16>;
    fn read_u32(&self, position: &mut u32) -> Result<u32>;
    fn read_u64(&self, position: &mut u32) -> Result<u64>;
    fn read_u8_array(&self, position: &mut u32) -> Result<Vec<u8>>;
    fn read_u16_array(&self, position: &mut u32) -> Result<Vec<u16>>;
    fn read_u32_array(&self, position: &mut u32) -> Result<Vec<u32>>;
    fn read_u64_array(&self, position: &mut u32) -> Result<Vec<u64>>;

    // Float
    fn read_f32(&self, position: &mut u32) -> Result<f32>;
    fn read_f64(&self, position: &mut u32) -> Result<f64>;
    fn read_f32_array(&self, position: &mut u32) -> Result<Vec<f32>>;
    fn read_f64_array(&self, position: &mut u32) -> Result<Vec<f64>>;

    fn read_vector3(&self, position: &mut u32) -> Result<Vector3<f32>>;
    fn read_vector4(&self, position: &mut u32) -> Result<Vector4<f32>>;
    fn read_matrix4(&self, position: &mut u32) -> Result<Matrix4<f32>>;
    fn read_quaternion(&self, position: &mut u32) -> Result<Quaternion<f32>>;
    fn read_qs_transform(&self, position: &mut u32) -> Result<QsTransform<f32>>;

    /// Read class ptr
    /// - move usize
    fn read_class_ptr<'a>(&'a self, position: &mut u32) -> Result<Cow<'a, str>>;

    /// # Expected bytes
    /// Jump to local_map.dst by current position, then read null terminated string.
    /// - move usize
    fn read_string_ptr<'a>(&'a self, position: &mut u32) -> Result<Cow<'a, str>>;

    /// Reads array information & Advance position(usize + 8bytes)
    ///
    /// This is the common code part that is first read by other Array types (except Matrix, etc.).
    ///
    /// # Move position bytes size
    /// - 32bit => 12bytes
    /// - 64bit => 16bytes
    ///
    /// # Returns
    /// Array size
    fn read_array_size(&self, position: &mut u32) -> Result<u32>;
    fn read_string_ptr_array<'a>(&'a self, position: &mut u32) -> Result<HkArrayStringPtr<'a>>;
    fn read_class_array<'a, T>(&'a self, position: &mut u32) -> Result<HkArrayClass<T>>
    where
        T: ByteDeSerialize<'a> + 'a;
}

macro_rules! impl_array {
    ($method:ident, $func:ident, $ret:ty) => {
        fn $method(&self, position: &mut u32) -> Result<Vec<$ret>> {
            let current_start = *position;
            let mut array = Vec::new();

            let size = self.read_array_size(position)?;
            if size > 0 {
                let mut local_dst = self.data_section.local_map[&current_start].dst;
                for _ in 0..size {
                    array.push(self.$func(&mut local_dst)?);
                }
            }

            Ok(array)
        }
    };
}

macro_rules! impl_primitive {
    ($func:ident, $ret:ty, $bytes:literal) => {
        fn $func(&self, position: &mut u32) -> Result<$ret> {
            let num = B::$func(self.bytes(*position));
            *position += $bytes;
            Ok(num)
        }
    };
}

impl<'de, B: ByteOrder> ByteDeserializer for HkxDeserializer<'de, B> {
    fn align_usize(&self, position: &mut u32) {
        *position = align(*position, self.header.pointer_size as u32);
    }

    fn bytes(&self, position: u32) -> &[u8] {
        &self.data_section.section_data[position as usize..]
    }

    fn read_i8(&self, position: &mut u32) -> Result<i8> {
        let num = self.bytes(*position)[0] as i8;
        *position += 1;
        Ok(num)
    }

    fn read_u8(&self, position: &mut u32) -> Result<u8> {
        let num = self.bytes(*position)[0];
        *position += 1;
        Ok(num)
    }

    fn read_bool(&self, position: &mut u32) -> Result<bool> {
        Ok(self.read_u8(position)? == 1)
    }

    fn read_usize(&self, position: &mut u32) -> Result<usize> {
        let num = match self.header.pointer_size {
            4 => B::read_u32(self.bytes(*position)) as usize,
            8 => B::read_u64(self.bytes(*position)) as usize,
            _ => return Err(BytesDeError::InvalidPtrSize(self.header.pointer_size).into()),
        };
        *position += self.header.pointer_size as u32;
        Ok(num)
    }

    impl_primitive!(read_i16, i16, 2);
    impl_primitive!(read_i32, i32, 4);
    impl_primitive!(read_i64, i64, 8);

    impl_primitive!(read_u16, u16, 2);
    impl_primitive!(read_u32, u32, 4);
    impl_primitive!(read_u64, u64, 8);

    impl_primitive!(read_f32, f32, 4);
    impl_primitive!(read_f64, f64, 8);

    impl_array!(read_bool_array, read_bool, bool);
    impl_array!(read_i8_array, read_i8, i8);
    impl_array!(read_i16_array, read_i16, i16);
    impl_array!(read_i32_array, read_i32, i32);
    impl_array!(read_i64_array, read_i64, i64);

    impl_array!(read_u8_array, read_u8, u8);
    impl_array!(read_u16_array, read_u16, u16);
    impl_array!(read_u32_array, read_u32, u32);
    impl_array!(read_u64_array, read_u64, u64);

    impl_array!(read_f32_array, read_f32, f32);
    impl_array!(read_f64_array, read_f64, f64);

    fn read_vector3(&self, position: &mut u32) -> Result<Vector3<f32>> {
        Ok(Vector3::new(
            self.read_f32(position)?,
            self.read_f32(position)?,
            self.read_f32(position)?,
        ))
    }

    fn read_vector4(&self, position: &mut u32) -> Result<Vector4<f32>> {
        Ok(Vector4::new(
            self.read_f32(position)?,
            self.read_f32(position)?,
            self.read_f32(position)?,
            self.read_f32(position)?,
        ))
    }

    fn read_matrix4(&self, position: &mut u32) -> Result<Matrix4<f32>> {
        Ok(Matrix4::from_cols(
            self.read_vector4(position)?,
            self.read_vector4(position)?,
            self.read_vector4(position)?,
            self.read_vector4(position)?,
        ))
    }

    fn read_quaternion(&self, position: &mut u32) -> Result<Quaternion<f32>> {
        Ok(Quaternion::from_sv(
            self.read_f32(position)?,
            self.read_vector3(position)?,
        ))
    }

    fn read_qs_transform(&self, position: &mut u32) -> Result<QsTransform<f32>> {
        Ok(QsTransform::from_cols(
            self.read_vector4(position)?,
            self.read_quaternion(position)?,
            self.read_vector4(position)?,
        ))
    }

    fn read_array_size(&self, position: &mut u32) -> Result<u32> {
        self.read_usize(position)?; // Consume pointer(u32|u64 == usize) assert 0
        let size = self.read_u32(position)?; // size(4bytes)
        let cap_flags = self.read_u32(position)?; // Capacity and flags(4bytes)

        let size_cap_flags = size | (0x80 << 24);
        if cap_flags != size_cap_flags {
            return Err(BytesDeError::MismatchCapacityAndSize {
                expected: size_cap_flags.to_string(),
                actual: cap_flags.to_string(),
            }
            .into());
        }
        Ok(size)
    }

    fn read_string_ptr_array<'a>(&'a self, position: &mut u32) -> Result<HkArrayStringPtr<'a>> {
        let current_start = *position;
        let mut strings = Vec::new();

        let size = self.read_array_size(position)?;
        if size > 0 {
            let mut local_dst = self.data_section.local_map[&current_start].dst;
            for _ in 0..size {
                strings.push(self.read_string_ptr(&mut local_dst)?);
            }
        }

        Ok(strings.into())
    }

    fn read_class_array<'a, T>(&'a self, position: &mut u32) -> Result<HkArrayClass<T>>
    where
        T: ByteDeSerialize<'a> + 'a,
    {
        let size = self.read_array_size(position)?;
        let mut res = Vec::new();

        if size > 0 {
            for _ in 0..size {
                res.push(T::from_bytes(self, position)?);
            }
        }

        Ok(res.into())
    }

    fn read_class_ptr<'a>(&'a self, position: &mut u32) -> Result<Cow<'a, str>> {
        let current_start = *position;

        if !self.data_section.global_map.contains_key(&current_start) {
            return Ok("".into());
        }

        let global_dst = self.data_section.global_map[&current_start].dst;
        let class_index = self.data_section.virtual_map.get_index_of(&global_dst);

        self.read_usize(position)?; // consume ptr usize.
        Ok(format!("#{:04}", 50 + class_index.unwrap()).into())
    }

    fn read_string_ptr<'a>(&'a self, position: &mut u32) -> Result<Cow<'a, str>> {
        let current_start = *position;

        if !self.data_section.local_map.contains_key(&current_start) {
            tracing::debug!("Not found local_map.dst. string ptr:current_start = {current_start}");
            return Ok("".into());
        }

        let local_dst = self.data_section.local_map[&current_start].dst;
        tracing::debug!("string ptr:current_start = {current_start}");
        tracing::debug!("string ptr:local_dst = {local_dst}");

        let c_str = CStr::from_bytes_until_nul(self.bytes(local_dst))?;
        let s = from_utf8(c_str.to_bytes())?;

        self.read_usize(position)?; // consume ptr usize.
        Ok(s.into())
    }
}

/// Information required for deserialization and re_serialization.
#[derive(Debug, Clone, PartialEq)]
pub struct HkxDeserializer<'de, B: ByteOrder> {
    pub header: &'de HkxHeader<B>,

    /// `__classnames__` section content bytes.
    pub class_section: SectionContents<'de>,
    /// `__data__` section content bytes. (`hkparam`s data of each class)
    pub data_section: SectionContents<'de>,
    /// `__type__` section content bytes.
    ///
    /// This information is not used during deserialization, but is needed during re_serialization.
    pub type_section: SectionContents<'de>,

    /// Signature & ClassName pairs from `__class_names__` section content bytes.
    pub class_names: ClassNames<'de>,
}

pub type DeserializedObjects<'de> = IndexMap<u32, ClassParams<'de>>;

impl<'de, B: ByteOrder> HkxDeserializer<'de, B> {
    /// Read hkx header, each section header, section fixup offsets maps, and class name offsets map.
    ///
    /// # Note
    /// This method does not yet read the binary data information of the fields of the C++ Havok Class.
    pub fn from_bytes(bytes: &'de [u8]) -> Result<Self> {
        let mut current_position = 0;

        // 1. Read 64bytes hkx file header.
        let header = HkxHeader::<B>::ref_from_bytes(bytes)?;
        current_position += HkxHeader::len();

        // 2. Skip padding
        current_position += header.padding_size();

        // 3. Read 48bytes * 3 section headers
        let class_header = SectionHeader::<B>::ref_from_bytes(&bytes[current_position..])?;
        current_position += size_of::<SectionHeader<B>>();

        let type_header = SectionHeader::<B>::ref_from_bytes(&bytes[current_position..])?;
        current_position += size_of::<SectionHeader<B>>();

        let data_header = SectionHeader::<B>::ref_from_bytes(&bytes[current_position..])?;

        tracing::debug!("class_header: {class_header}");
        tracing::debug!(" type_header: {type_header}");
        tracing::debug!(" data_header: {data_header}");

        // 4. Section fixup map by each section header information
        let class_section = SectionContents::from_bytes(bytes, class_header, 0)?;
        let type_section = SectionContents::from_bytes(bytes, type_header, 1)?;
        let data_section = SectionContents::from_bytes(bytes, data_header, 2)?;

        // 5. Read class section content
        let class_names = ClassNames::from_bytes::<B>(class_section.section_data)?;

        tracing::debug!("data_section.local_map: {:#?}", &data_section.local_map);
        tracing::debug!("data_section.global_map: {:#?}", &data_section.global_map);
        tracing::debug!("data_section.virtual_map: {:#?}", &data_section.virtual_map);
        tracing::debug!(" class_names: {:#?}", &class_names);

        Ok(Self {
            header,
            class_section,
            data_section,
            type_section,
            class_names,
        })
    }

    /// Create a new instance from hkx class fields.
    pub fn deserialize(&self) -> Result<Hkx> {
        let mut position;
        let mut deserialized_objects = Vec::new();

        for (_, virtual_fixup) in &self.data_section.virtual_map {
            position = virtual_fixup.src;

            let name_offset = virtual_fixup.name_offset as usize;
            let hk_class_name = &self.class_names.0[&name_offset].class_name.to_str()?;
            let class = ClassParams::from_bytes(hk_class_name, self, &mut position)?;

            tracing::debug!("deserialized class = {:#?}", class);
            deserialized_objects.push(class);
        }

        Ok(Hkx {
            class_version: self.header.file_version.get(),
            content_version: self.header.contents_version_string_as_str()?.into(),
            top_level_object: "#0050".into(),
            hk_section: deserialized_objects.into(),
        })
    }
}

/// Error type for binary data deserialization.
#[derive(Debug, thiserror::Error)]
pub enum BytesDeError {
    /// Expected pointer size 4 or 8 byte. But got {0}.
    #[error("Expected pointer size 4 or 8 byte. But got {0}.")]
    InvalidPtrSize(u8),

    /// Mismatch array capacity and size. actual: {actual}, expected: {expected}.
    #[error("Mismatch array capacity and size. actual: {actual}, expected: {expected}.")]
    MismatchCapacityAndSize { expected: String, actual: String },
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused)]
    use pretty_assertions::assert_eq;
    use zerocopy::{BigEndian, LittleEndian};

    #[test]
    #[quick_tracing::init(test = "deserialize_hkx_bytes", level = "Error")]
    fn should_deserialize() -> anyhow::Result<()> {
        let test_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("tests");
        // let bytes = include_bytes!("../../../../tests/1hm_behavior_x86_64.hkx");
        // let bytes = include_bytes!("../../../../tests/defaultmale.hkx");
        let bytes = include_bytes!("../../../../tests/wisp_skeleton.hkx");
        // let bytes = std::fs::read(test_dir.join("defaultmale.hkx")).unwrap();
        let bytes = bytes.as_slice();

        match HkxHeader::is_big_endian(bytes) {
            true => {
                let de = HkxDeserializer::<BigEndian>::from_bytes(bytes)?;
                let hkx = de.deserialize()?;
                std::fs::write(
                    test_dir.join("test.xml"),
                    crate::helpers::serde::to_string_pretty_xml(&hkx, ' ', 2)?,
                )?;
            }
            false => {
                let de = HkxDeserializer::<LittleEndian>::from_bytes(bytes)?;
                let hkx = de.deserialize()?;
                std::fs::write(
                    test_dir.join("test.xml"),
                    crate::helpers::serde::to_string_pretty_xml(&hkx, ' ', 2)?,
                )?;
            }
        };

        Ok(())
    }
}
