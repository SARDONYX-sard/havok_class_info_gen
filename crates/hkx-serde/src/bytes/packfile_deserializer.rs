use super::hkx_class_name::HKXClassNames;
use super::hkx_header::HkxHeader;
use super::hkx_section::HKXSection;
use byteorder::{ByteOrder, ReadBytesExt};
use std::collections::HashMap;
use std::io::{Cursor, Read, Seek, SeekFrom};

pub struct PackFileDeserializer {
    pub class_names: HKXClassNames,
    pub class_section: HKXSection,
    pub data_section: HKXSection,
    pub deserialized_objects: HashMap<u32, IHavokObject>,
    pub header: HkxHeader,
    pub type_section: HKXSection,
    pub ignore_non_fatal_error: bool,
}

impl PackFileDeserializer {
    fn construct_virtual_class(&mut self, br: impl Read + Seek, offset: u32) -> IHavokObject {
        if let Some(obj) = self.deserialized_objects.get(&offset) {
            return obj.clone();
        }

        let fixup = self.data_section.virtual_map[&offset];
        let hk_class_name = self.class_names.offset_class_names_map[&fixup.dst]
            .class_name
            .clone();

        let hk_class = match System::get_type(format!("HKX2.{}", hk_class_name)) {
            Some(class) => class,
            None => panic!("Havok class type '{}' not found!", hk_class_name),
        };

        let ret = match Activator::create_instance(&hk_class) {
            Some(instance) => instance,
            None => panic!("Failed to Activator.CreateInstance({})", hk_class),
        };

        br.seek(SeekFrom::Start(offset.into()));
        ret.read(self, br);

        self.deserialized_objects.insert(offset, ret.clone());
        ret
    }

    fn deserialize_partially<B: ByteOrder>(
        &mut self,
        mut br: impl Read + Seek,
    ) -> std::io::Result<()> {
        br.seek(SeekFrom::Start(0x11))?;
        let _endian = br.read_u8()? == 0x0;

        br.seek(SeekFrom::Start(0))?;
        self.header = HkxHeader::read::<B>(br)?;

        if self.header.section_offset == -1 {
            br.seek(SeekFrom::Start(0x40))?;
        } else {
            br.seek(SeekFrom::Start((self.header.section_offset as u64) + 0x40))?;
        }

        self.class_section = HKXSection::new::<B>(br, &self.header.contents_version_string)?;
        self.type_section = HKXSection::new::<B>(br, &self.header.contents_version_string)?;
        self.data_section = HKXSection::new::<B>(br, &self.header.contents_version_string)?;

        self.class_names = HKXClassNames::read::<B>(br)?;
        Ok(())
    }

    fn deserialize(&mut self, mut br: impl Read + Seek, ignore_non_fatal_error: bool) -> T {
        self.ignore_non_fatal_error = ignore_non_fatal_error;

        self.deserialize_partially(br);

        self.deserialized_objects = HashMap::new();
        let mut br2 = BinaryReaderEx::new(
            self.header.endian == 0,
            self.header.pointer_size == 8,
            Cursor::new(self.data_section.section_data.clone()),
        );
        self.construct_virtual_class(&mut br2, 0)
    }
}
