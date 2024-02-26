//! # HKX Header Format Specification
//!
//! The HKX header format is used for storing metadata information in HKX files.
//! HKX files are binary files commonly used in video game development for storing animation and physics data.
//! The header contains essential information about the structure and properties of the HKX file.
//!
//! | Field Name                     | Description                                                    | Size (bytes) | Offset (bytes) |
//! | ------------------------------ | -------------------------------------------------------------- | ------------ | -------------- |
//! | Magic0                         | First magic number (`0x57E0E057`)                              | 4            | 0              |
//! | Magic1                         | Second magic number (`0x10C0C010`)                             | 4            | 4              |
//! | UserTag                        | User-defined tag                                               | 4            | 8              |
//! | FileVersion                    | Version of the file (LittleEndian e.g. 0x08 0x00 0x00 0x00)    | 4            | 12             |
//! | PointerSize                    | Size of pointers in bytes (4 or 8)                             | 1            | 16             |
//! | Endian                         | Endianness of the file (0 for big-endian, 1 for little-endian) | 1            | 17             |
//! | PaddingOption                  | Padding option used in the file                                | 1            | 18             |
//! | BaseClass                      | Base class                                                     | 1            | 19             |
//! | SectionCount                   | Number of sections in the HKX file                             | 4            | 20             |
//! | ContentsSectionIndex           | Index of the contents section within the file                  | 4            | 24             |
//! | ContentsSectionOffset          | Offset of the contents section within the file                 | 4            | 28             |
//! | ContentsClassNameSectionIndex  | Index of the contents class name section within the file       | 4            | 32             |
//! | ContentsClassNameSectionOffset | Offset of the contents class name section within the file      | 4            | 36             |
//! | ContentsVersionString          | Version string of the contents (fixed-size string, 16 bytes)   | 16           | 40             |
//! | Flags                          | Various flags used in the file                                 | 4            | 56             |
//! | MaxPredicate                   | Maximum predicate value                                        | 2            | 60             |
//! | SectionOffset                  | Section offset within the file                                 | 2            | 62             |
//!
//! ## Unknowns
//! | Field Name                     | Description                                                    | Size (bytes) | Offset (bytes) |
//! | ------------------------------ | -------------------------------------------------------------- | ------------ | -------------- |
//! | Unk40                          | Unknown field                                                  | 2            | 64             |
//! | Unk42                          | Unknown field                                                  | 2            | 66             |
//! | Unk44                          | Unknown field                                                  | 4            | 68             |
//! | Unk48                          | Unknown field                                                  | 4            | 72             |
//! | Unk4C                          | Unknown field                                                  | 4            | 76             |

use super::fix_str_io::{read_fix_str, write_fix_str};
use byteorder::{ByteOrder, ReadBytesExt as _, WriteBytesExt as _};
use std::io::{Read, Write};

/// Structure representing a HkxHeader of size 64bytes
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
/// The HKX header contains metadata information about the HKX file.
///
/// # Information
/// The size of the binary header itself is 64bytes.
/// (i.e., different from the size of this structure).
pub struct HkxHeader {
    /// It will always be this 4 bytes.
    ///
    /// `57E0 E057`
    pub magic0: u32,
    /// It will always be this 4 bytes.
    ///
    /// `10C0 C010`
    pub magic1: u32,
    /// User-defined tag.
    pub user_tag: i32,
    /// Version of the file.
    pub file_version: i32,
    /// Size of pointers in bytes.
    pub pointer_size: u8,
    /// Endianness of the file (0 for little-endian, 1 for big-endian).
    pub endian: u8,
    /// Padding option used in the file.
    pub padding_option: u8,
    /// Base class.
    pub base_class: u8,
    /// Number of sections in the HKX file.
    pub section_count: i32,
    /// Index of the contents section.
    pub contents_section_index: i32,
    /// Offset of the contents section.
    pub contents_section_offset: i32,
    /// Index of the contents class name section.
    pub contents_class_name_section_index: i32,
    /// Offset of the contents class name section.
    pub contents_class_name_section_offset: i32,
    /// Version string of the contents.
    /// # NOTE
    /// Originally `[u8;16]`, although it is string for API convenience.
    pub contents_version_string: String,
    /// Various flags.
    pub flags: i32,
    /// Maximum predicate.
    pub max_predicate: i16,
    /// Section offset.
    pub section_offset: i16,
    /// Unknown field 1.(binary header offset: 0x40 bytes)
    pub unk40: i16,
    /// Unknown field 2.(binary header offset: 0x42 bytes)
    pub unk42: i16,
    /// Unknown field 3.(binary header offset: 0x44 bytes)
    pub unk44: u32,
    /// Unknown field 4.(binary header offset: 0x48 bytes)
    pub unk48: u32,
    /// Unknown field 5.(binary header offset: 0x4c bytes)
    pub unk4c: u32,
}

impl HkxHeader {
    /// Constructs a new [`HkxHeader`] with default values.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn skyrim_se() -> Self {
        HkxHeader {
            magic0: 0x57E0E057,
            magic1: 0x10C0C010,
            file_version: 0x08,
            pointer_size: 8,
            endian: 1,
            base_class: 1,
            section_count: 3,
            contents_section_index: 2,
            contents_class_name_section_offset: 0x4B,
            // The only way to take READ into account is to make it owned.
            contents_version_string: "hk_2010.2.0-r1".to_owned(),
            max_predicate: -1,
            section_offset: -1,
            ..Default::default()
        }
    }

    /// Reads an [`HkxHeader`] from a [`Read`] trait object.
    ///
    /// # Errors
    ///
    /// This method will return an error if reading fails for any reason.
    pub fn read<B: ByteOrder>(mut br: impl Read) -> std::io::Result<Self> {
        let magic0 = br.read_u32::<B>()?;
        let magic1 = br.read_u32::<B>()?;
        let user_tag = br.read_i32::<B>()?;
        let file_version = br.read_i32::<B>()?;
        let pointer_size = br.read_u8()?;
        let endian = br.read_u8()?;
        let padding_option = br.read_u8()?;
        let base_class = br.read_u8()?;
        let section_count = br.read_i32::<B>()?;
        let contents_section_index = br.read_i32::<B>()?;
        let contents_section_offset = br.read_i32::<B>()?;
        let contents_class_name_section_index = br.read_i32::<B>()?;
        let contents_class_name_section_offset = br.read_i32::<B>()?;
        let contents_version_string = read_fix_str(&mut br, 16)?;
        let flags = br.read_i32::<B>()?;
        let max_predicate = br.read_i16::<B>()?;
        let section_offset = br.read_i16::<B>()?;

        let (unk40, unk42, unk44, unk48, unk4c) = if section_offset == 16 {
            let unk40 = br.read_i16::<B>()?;
            let unk42 = br.read_i16::<B>()?;
            let unk44 = br.read_u32::<B>()?;
            let unk48 = br.read_u32::<B>()?;
            let unk4c = br.read_u32::<B>()?;
            (unk40, unk42, unk44, unk48, unk4c)
        } else {
            (0, 0, 0, 0, 0)
        };

        Ok(HkxHeader {
            magic0,
            magic1,
            user_tag,
            file_version,
            pointer_size,
            endian,
            padding_option,
            base_class,
            section_count,
            contents_section_index,
            contents_section_offset,
            contents_class_name_section_index,
            contents_class_name_section_offset,
            contents_version_string,
            flags,
            max_predicate,
            section_offset,
            unk40,
            unk42,
            unk44,
            unk48,
            unk4c,
        })
    }

    /// Writes the [`HkxHeader`] to a [`Write`] trait object.
    ///
    /// # Errors
    ///
    /// This method will return an error if writing fails for any reason.
    pub fn write<B: ByteOrder>(&self, mut bw: impl Write) -> std::io::Result<()> {
        bw.write_u32::<B>(self.magic0)?;
        bw.write_u32::<B>(self.magic1)?;
        bw.write_i32::<B>(self.user_tag)?;
        bw.write_i32::<B>(self.file_version)?;
        bw.write_u8(self.pointer_size)?;
        bw.write_u8(self.endian)?;
        bw.write_u8(self.padding_option)?;
        bw.write_u8(self.base_class)?;
        bw.write_i32::<B>(self.section_count)?;
        bw.write_i32::<B>(self.contents_section_index)?;
        bw.write_i32::<B>(self.contents_section_offset)?;
        bw.write_i32::<B>(self.contents_class_name_section_index)?;
        bw.write_i32::<B>(self.contents_class_name_section_offset)?;
        write_fix_str(&mut bw, &self.contents_version_string, 16)?;
        bw.write_i32::<B>(self.flags)?;
        bw.write_i16::<B>(self.max_predicate)?;
        bw.write_i16::<B>(self.section_offset)?;

        if self.section_offset == 16 {
            bw.write_i16::<B>(self.unk40)?;
            bw.write_i16::<B>(self.unk42)?;
            bw.write_u32::<B>(self.unk44)?;
            bw.write_u32::<B>(self.unk48)?;
            bw.write_u32::<B>(self.unk4c)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use byteorder::NativeEndian;
    use pretty_assertions::assert_eq;
    use std::io::Cursor;

    const SKYRIM_SE_ROW_HEADER: [u8; 64] = [
        0x57, 0xE0, 0xE0, 0x57, // magic0
        0x10, 0xC0, 0xC0, 0x10, // magic1
        0x00, 0x00, 0x00, 0x00, // user tag
        0x08, 0x00, 0x00, 0x00, // file version
        0x08, // pointer size
        0x01, // endian
        0x00, // padding option
        0x01, // base class
        0x03, 0x00, 0x00, 0x00, // section count
        0x02, 0x00, 0x00, 0x00, // contents section index
        0x00, 0x00, 0x00, 0x00, // content section offset
        0x00, 0x00, 0x00, 0x00, // contents class name section index
        0x4B, 0x00, 0x00, 0x00, // contents class name section offset
        // 16bytes contents version string b"hk_2010.2.0-r1\0\0"
        0x68, 0x6B, 0x5F, 0x32, 0x30, 0x31, 0x30, 0x2E, 0x32, 0x2E, 0x30, 0x2D, 0x72, 0x31, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, // flags
        0xFF, 0xFF, // max predicate: -1 as i16
        0xFF, 0xFF, // section offset: -1 as i16
    ];

    #[test]
    fn should_read_hkx_file() {
        let actual = HkxHeader::read::<NativeEndian>(Cursor::new(SKYRIM_SE_ROW_HEADER))
            .expect("Failed to read HKXHeader");

        assert_eq!(actual, HkxHeader::skyrim_se());
    }

    #[test]
    fn should_write_hkx_file() {
        let mut hkx_file = Vec::new();
        HkxHeader::skyrim_se()
            .write::<NativeEndian>(Cursor::new(&mut hkx_file))
            .expect("Failed to read HKXHeader");

        assert_eq!(hkx_file.as_slice(), SKYRIM_SE_ROW_HEADER.as_slice());
    }
}
