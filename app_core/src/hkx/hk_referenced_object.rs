use byteorder::{ByteOrder, ReadBytesExt as _, WriteBytesExt as _};
use serde::{Deserialize, Serialize};
use std::io::{self, Read, Seek, SeekFrom, Write};

/// # Hkxcmd Info
/// ```txt
/// Signature: 3b1c1113
///  VTable: TRUE
///  Name: hkReferencedObject
///  Parent: hkBaseObject (e0708a00)
///  Size: 8
///  #IFace: 0
///  #Enums: 0
///  #Members: 2
///    000 memSizeAndFlags,HK_NULL,HK_NULL,hkUint16,00000006,00000000,0,00000400,4,HK_NULL,HK_NULL
///    001 referenceCount,HK_NULL,HK_NULL,hkInt16,00000005,00000000,0,00000400,6,HK_NULL,HK_NULL
///  Version: 0
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HkReferencedObject {
    #[serde(borrow)]
    #[serde(default = "HkReferencedObject::signature")]
    #[serde(rename = "@signature")]
    /// Fixed value unique to each class: `0xc713064e`
    pub signature: &'static str,
    #[serde(skip_serializing)]
    mem_size_and_flags: u16,
    #[serde(skip_serializing)]
    reference_count: i16,
}

impl HkReferencedObject {
    pub fn new(mem_size_and_flags: u16, reference_count: i16) -> Self {
        Self {
            mem_size_and_flags,
            reference_count,
            signature: Self::signature(),
        }
    }

    pub const fn signature() -> &'static str {
        "0x3b1c1113"
    }

    pub fn read<B: ByteOrder>(mut reader: impl Read + Seek) -> io::Result<Self> {
        let mem_size_and_flags = reader.read_u16::<B>()?;
        let reference_count = reader.read_i16::<B>()?;

        // Seek to the next 8-byte boundary if necessary
        let remain = reader.stream_position()? % 8;
        if remain != 0 {
            let padding_size = 8 - remain;
            reader.seek(SeekFrom::Current(padding_size as i64))?;
        }

        Ok(HkReferencedObject {
            signature: Self::signature(),
            mem_size_and_flags,
            reference_count,
        })
    }

    pub fn write<B: ByteOrder>(&self, mut writer: impl Write + Seek) -> io::Result<()> {
        writer.write_u16::<B>(self.mem_size_and_flags)?;
        writer.write_i16::<B>(self.reference_count)?;

        // Seek to the next 8-byte boundary if necessary
        let remain = writer.stream_position()? % 8;
        if remain != 0 {
            let padding_size = 8 - (remain);
            for _ in 0..padding_size {
                writer.write_u8(0)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use byteorder::NativeEndian;
    use pretty_assertions::assert_eq;
    use quick_xml::de::from_str;
    use std::io::Cursor;

    #[test]
    fn should_deserialize() {
        let data = "<HkReferencedObject signature=\"0x3b1c1113\">\
                <mem_size_and_flags>6</mem_size_and_flags>\
                <reference_count>5</reference_count>\
            </HkReferencedObject>";

        let actual: HkReferencedObject = from_str(data).unwrap();
        assert_eq!(actual, HkReferencedObject::new(6, 5));
    }

    #[test]
    fn should_read_bytes() {
        #[rustfmt::skip]
        let bytes = [
            0x13, 0x11, // memSizeAndFlags
            0x1c, 0x3b, // referenceCount
            0x00, 0x00, 0x00, 0x00 // Padding
        ];

        let actual =
            HkReferencedObject::read::<NativeEndian>(&mut Cursor::new(bytes.as_slice())).unwrap();
        assert_eq!(actual, HkReferencedObject::new(0x1113, 0x3b1c,));
    }

    #[test]
    fn should_write_bytes() {
        let hkx_struct = HkReferencedObject::new(0x1113, 0x3b1c);

        // NOTE: To use [`Cursor`], you must initialize it before you can write to it.
        let mut actual = vec![0u8; 8];
        hkx_struct
            .write::<NativeEndian>(&mut Cursor::new(actual.as_mut_slice()))
            .unwrap();

        #[rustfmt::skip]
        assert_eq!(actual, [
            0x13, 0x11, // memSizeAndFlags
            0x1c, 0x3b, // referenceCount
            0, 0, 0, 0, // Padding align 8
        ]);
    }
}
