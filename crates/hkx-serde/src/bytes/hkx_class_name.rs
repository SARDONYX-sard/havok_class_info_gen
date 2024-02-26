use byteorder::{ByteOrder, ReadBytesExt as _, WriteBytesExt as _};
use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom, Write};

const SIGNATURE_SEPARATOR: u8 = 0x09;

#[derive(Debug, Clone, PartialEq)]
struct HKXClassName {
    pub class_name: String,
    pub signature: u32,
}

impl HKXClassName {
    /// Read one class from bytes reader.
    fn read<B: ByteOrder>(mut br: impl Read) -> std::io::Result<Self> {
        let signature = br.read_u32::<B>()?;
        let _separator = br.read_u8()?; // Unused separator(0x09)

        // Read ASCII characters until non-ASCII characters.
        let mut class_name_bytes = Vec::new();
        loop {
            let byte = br.read_u8()?;
            if byte.is_ascii() {
                class_name_bytes.push(byte);
            } else {
                break;
            }
        }

        let class_name = String::from_utf8_lossy(&class_name_bytes).into();
        Ok(HKXClassName {
            class_name,
            signature,
        })
    }

    /// Write one class to bytes writer.
    fn write<B: ByteOrder>(&self, mut bw: impl Write) -> std::io::Result<()> {
        bw.write_u32::<B>(self.signature)?;
        bw.write_u8(SIGNATURE_SEPARATOR)?; // Unused separator
        bw.write_all(self.class_name.as_bytes())?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HKXClassNames {
    pub class_names: Vec<HKXClassName>,
    pub offset_class_names_map: HashMap<u32, HKXClassName>,
}

impl HKXClassNames {
    /// Read class from bytes reader.
    pub fn read<B: ByteOrder>(mut br: impl Read + Seek) -> std::io::Result<Self> {
        let mut class_names = Vec::new();
        let mut offset_class_names_map = HashMap::new();

        loop {
            let mut signature_buf = [0; 4];
            // The 0x09 serves as a separator,
            // and binaries such as `<class_of_signature>0x09` are assumed.
            if br.read_exact(&mut signature_buf).is_err() || br.read_u8()? != SIGNATURE_SEPARATOR {
                break;
            }
            let string_start = br.stream_position()? as u32 - 5;
            br.seek(SeekFrom::Current(-5))?;
            let class_name = HKXClassName::read::<B>(&mut br)?;
            class_names.push(class_name.clone());
            offset_class_names_map.insert(string_start, class_name);
        }
        Ok(HKXClassNames {
            class_names,
            offset_class_names_map,
        })
    }
}
