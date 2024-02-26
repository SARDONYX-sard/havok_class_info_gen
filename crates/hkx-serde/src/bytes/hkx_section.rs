use byteorder::{ByteOrder, ReadBytesExt as _, WriteBytesExt as _};
use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom, Write};

use super::fix_str_io::{read_fix_str, write_fix_str};

trait Fixup {
    /// Read bytes from byte reader.
    fn read<B: ByteOrder>(reader: impl Read) -> std::io::Result<Self>
    where
        Self: Sized;

    /// Write bytes to writer.
    fn write<B: ByteOrder>(&self, writer: impl Write) -> std::io::Result<()>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocalFixup {
    pub src: u32,
    pub dst: u32,
}

impl Fixup for LocalFixup {
    fn read<B: ByteOrder>(mut br: impl Read) -> std::io::Result<Self> {
        let src = br.read_u32::<B>()?;
        let dst = br.read_u32::<B>()?;
        Ok(LocalFixup { src, dst })
    }

    fn write<B: ByteOrder>(&self, mut bw: impl Write) -> std::io::Result<()> {
        bw.write_u32::<B>(self.src)?;
        bw.write_u32::<B>(self.dst)?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GlobalFixup {
    pub src: u32,
    pub dst_section_index: u32,
    pub dst: u32,
}

impl Fixup for GlobalFixup {
    fn read<B: ByteOrder>(mut br: impl Read) -> std::io::Result<Self> {
        let src = br.read_u32::<B>()?;
        let dst_section_index = br.read_u32::<B>()?;
        let dst = br.read_u32::<B>()?;
        Ok(GlobalFixup {
            src,
            dst_section_index,
            dst,
        })
    }

    fn write<B: ByteOrder>(&self, mut bw: impl Write) -> std::io::Result<()> {
        bw.write_u32::<B>(self.src)?;
        bw.write_u32::<B>(self.dst_section_index)?;
        bw.write_u32::<B>(self.dst)?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VirtualFixup {
    pub src: u32,
    pub dst_section_index: u32,
    pub dst: u32,
}

impl Fixup for VirtualFixup {
    fn read<B: ByteOrder>(mut br: impl Read) -> std::io::Result<Self> {
        let src = br.read_u32::<B>()?;
        let dst_section_index = br.read_u32::<B>()?;
        let dst = br.read_u32::<B>()?;
        Ok(VirtualFixup {
            src,
            dst_section_index,
            dst,
        })
    }

    fn write<B: ByteOrder>(&self, mut bw: impl Write) -> std::io::Result<()> {
        bw.write_u32::<B>(self.src)?;
        bw.write_u32::<B>(self.dst_section_index)?;
        bw.write_u32::<B>(self.dst)?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HKXSection {
    pub global_map: HashMap<u32, GlobalFixup>,
    pub local_map: HashMap<u32, LocalFixup>,
    pub virtual_map: HashMap<u32, VirtualFixup>,
    pub global_fixups: Vec<GlobalFixup>,
    pub local_fixups: Vec<LocalFixup>,
    pub virtual_fixups: Vec<VirtualFixup>,
    pub section_data: Vec<u8>,
    pub section_id: i32,
    pub section_tag: String,
    pub contents_version_string: String,
}

impl HKXSection {
    const OFFSET_MAP_IS_NONE: u32 = 0xFFFFFFFF;

    /// Same [`Self::read`]
    pub fn new<B: ByteOrder>(
        br: impl Read + Seek,
        contents_version_string: &str,
    ) -> std::io::Result<Self> {
        Self::read::<B>(br, contents_version_string)
    }

    pub fn read<B: ByteOrder>(
        mut br: impl Read + Seek,
        contents_version_string: &str,
    ) -> std::io::Result<Self> {
        let section_tag = read_fix_str(&mut br, 19)?;
        let _separator = br.read_u8()?; // Unused separator
        let absolute_data_start = br.read_u32::<B>()?;
        let local_fixups_offset = br.read_u32::<B>()?;
        let global_fixups_offset = br.read_u32::<B>()?;
        let virtual_fixups_offset = br.read_u32::<B>()?;
        let exports_offset = br.read_u32::<B>()?;
        let _imports_offset = br.read_u32::<B>()?;
        let _end_offset = br.read_u32::<B>()?;

        br.seek(SeekFrom::Start(absolute_data_start.into()))?;
        let mut section_data = vec![0; (local_fixups_offset) as usize];
        br.read_exact(&mut section_data)?;

        let mut local_fixups = Vec::new();
        br.seek(SeekFrom::Start(
            (absolute_data_start + local_fixups_offset).into(),
        ))?;
        while br.stream_position()? < (absolute_data_start + global_fixups_offset).into() {
            if br.read_u32::<B>()? != Self::OFFSET_MAP_IS_NONE {
                local_fixups.push(LocalFixup::read::<B>(&mut br)?);
            }
        }

        let mut global_fixups = Vec::new();
        br.seek(SeekFrom::Start(
            (absolute_data_start + global_fixups_offset).into(),
        ))?;
        while br.stream_position()? < (absolute_data_start + virtual_fixups_offset).into() {
            if br.read_u32::<B>()? != Self::OFFSET_MAP_IS_NONE {
                global_fixups.push(GlobalFixup::read::<B>(&mut br)?);
            }
        }

        let mut virtual_fixups = Vec::new();
        br.seek(std::io::SeekFrom::Start(
            (absolute_data_start + virtual_fixups_offset).into(),
        ))?;
        while br.stream_position()? < (absolute_data_start + exports_offset).into() {
            if br.read_u32::<B>()? != Self::OFFSET_MAP_IS_NONE {
                virtual_fixups.push(VirtualFixup::read::<B>(&mut br)?);
            }
        }

        if contents_version_string != "hk_2010.2.0-r1" {
            br.read_u32::<B>()?; // Padding
            br.read_u32::<B>()?; // Padding
            br.read_u32::<B>()?; // Padding
            br.read_u32::<B>()?; // Padding
        }

        Ok(HKXSection {
            global_map: global_fixups
                .iter()
                .map(|fixup| (fixup.src, fixup.clone()))
                .collect(),
            local_map: local_fixups
                .iter()
                .map(|fixup| (fixup.src, fixup.clone()))
                .collect(),
            virtual_map: virtual_fixups
                .iter()
                .map(|fixup| (fixup.src, fixup.clone()))
                .collect(),
            global_fixups,
            local_fixups,
            virtual_fixups,
            section_data,
            section_id: 0, // Set a default value
            section_tag,
            contents_version_string: contents_version_string.to_owned(),
        })
    }

    fn write_header<B: ByteOrder>(&self, mut bw: impl Write) -> std::io::Result<()> {
        write_fix_str(&mut bw, &self.section_tag, 19)?;
        bw.write_u8(0xFF)?; // Unused separator
        bw.write_u32::<B>(0)?; // Placeholder for absolute offset
        bw.write_u32::<B>(0)?; // Placeholder for local offset
        bw.write_u32::<B>(0)?; // Placeholder for global offset
        bw.write_u32::<B>(0)?; // Placeholder for virtual offset
        bw.write_u32::<B>(0)?; // Placeholder for exports offset
        bw.write_u32::<B>(0)?; // Placeholder for imports offset
        bw.write_u32::<B>(0)?; // Placeholder for end offset

        if self.contents_version_string != "hk_2010.2.0-r1" {
            bw.write_u32::<B>(0xFFFFFFFF)?;
            bw.write_u32::<B>(0xFFFFFFFF)?;
            bw.write_u32::<B>(0xFFFFFFFF)?;
            bw.write_u32::<B>(0xFFFFFFFF)?;
        }

        Ok(())
    }

    fn write_data<B: ByteOrder>(&self, mut bw: impl Write + Seek) -> std::io::Result<()> {
        let absolute_offset = bw.stream_position()? as u32;
        bw.write_all(&self.section_data)?;

        while bw.stream_position()? % 16 != 0 {
            bw.write_u8(0xFF)?;
        } // 16-byte alignment

        let local_offset = bw.stream_position()? as u32;
        for local_fixup in &self.local_fixups {
            local_fixup.write::<B>(&mut bw)?;
        }

        while bw.stream_position()? % 16 != 0 {
            bw.write_u8(0xFF)?;
        } // 16-byte alignment

        let global_offset = bw.stream_position()? as u32;
        for global_fixup in &self.global_fixups {
            global_fixup.write::<B>(&mut bw)?;
        }

        while bw.stream_position()? % 16 != 0 {
            bw.write_u8(0xFF)?;
        } // 16-byte alignment

        let virtual_offset = bw.stream_position()? as u32;
        for virtual_fixup in &self.virtual_fixups {
            virtual_fixup.write::<B>(&mut bw)?;
        }

        while bw.stream_position()? % 16 != 0 {
            bw.write_u8(0xFF)?;
        } // 16-byte alignment

        let exports_offset = bw.stream_position()? as u32;
        let imports_offset = exports_offset;
        let end_offset = bw.stream_position()? as u32;

        bw.seek(std::io::SeekFrom::Start(4))?;
        bw.write_u32::<B>(absolute_offset)?;
        bw.write_u32::<B>(local_offset)?;
        bw.write_u32::<B>(global_offset)?;
        bw.write_u32::<B>(virtual_offset)?;
        bw.write_u32::<B>(exports_offset)?;
        bw.write_u32::<B>(imports_offset)?;
        bw.write_u32::<B>(end_offset)?;

        Ok(())
    }
}
