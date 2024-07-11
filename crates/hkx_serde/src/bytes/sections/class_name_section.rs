//! HKX `__classnames__` section binary Reader / Writer
use indexmap::IndexMap;
use std::ffi::{CStr, FromBytesUntilNulError};
use zerocopy::ByteOrder;

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClassPair<'a> {
    /// e.g. `0x2772c11e`
    pub signature: u32,
    /// e.g. `hkRootLevelContainer\0`
    pub class_name: &'a CStr,
}

impl<'a> ClassPair<'a> {
    /// Signature separator byte
    pub const SIGNATURE_SEPARATOR: u8 = 0x09;

    /// Read one class info(signature & class name) from bytes reader.
    ///
    /// # Expected bytes format
    /// signature(4bytes) + separator(0x09) + ClassName(Null terminated ASCII string)
    ///
    /// # Returns
    /// (Class name & signature, read bytes size)
    pub fn from_bytes<B: ByteOrder>(bytes: &'a [u8]) -> Result<(Self, usize)> {
        if bytes.len() < 5 {
            return Err(ClassNamesSectionError::InsufficientReadBufferSize(
                bytes.len(),
            ));
        };

        let mut offset = 0;

        // LE Example: F6 5E 58 75
        let signature = B::read_u32(&bytes[offset..offset + 4]);
        offset += 4;

        let separator = bytes[offset];
        offset += 1;

        if separator != Self::SIGNATURE_SEPARATOR {
            return Err(ClassNamesSectionError::InvalidSignatureSeparator {
                signature,
                wrong_separator: separator,
            });
        }

        let class_name = CStr::from_bytes_until_nul(&bytes[offset..])?;

        let class_name_len = class_name.to_str()?.len() + 1; // Include null terminator
        offset += class_name_len;

        let hkx_class_name = ClassPair {
            class_name,
            signature,
        };

        Ok((hkx_class_name, offset))
    }

    /// Write one class info(signature & class name) to bytes slice.
    ///
    /// # Return
    /// Written end offset.
    pub fn write_bytes<B: ByteOrder>(&self, bytes: &mut [u8]) -> Result<usize> {
        // Calculate need buffer size
        let class_name_bytes = self.class_name.to_bytes_with_nul();
        let class_name_len = class_name_bytes.len();
        // signature(4 bytes) + separator(1 bytes) + class name
        let class_info_bytes_len = 4 + 1 + class_name_len;

        if bytes.len() < class_info_bytes_len {
            return Err(ClassNamesSectionError::InsufficientWriteBufferSize {
                expected: bytes.len(),
                actual: class_info_bytes_len,
            });
        }

        B::write_u32(&mut bytes[0..4], self.signature);
        bytes[4] = Self::SIGNATURE_SEPARATOR;
        bytes[5..5 + class_name_len].copy_from_slice(class_name_bytes);

        Ok(class_info_bytes_len) // Return the written end offset
    }
}

/// IndexMap for each class name start position & (signature, class name)
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ClassNames<'bytes>(pub IndexMap<usize, ClassPair<'bytes>>);

impl<'a> ClassNames<'a> {
    /// For 16 bytes alignment
    const CLASSNAMES_END_PADDING: u8 = 0xFF;

    /// Read classes information(signature & className pair) from bytes reader.
    ///
    /// # Assumption
    /// The position of `Read` must be `absolute data start`.
    ///
    /// - `absolute_data_start`: The offset is signature & className pair bytes array in `__class_names__` section contents
    pub fn from_bytes<B: ByteOrder>(bytes: &'a [u8]) -> Result<Self> {
        let mut offset_class_names_map = IndexMap::new();
        let mut offset = 0;

        while let Ok((class_pair, read_size)) = ClassPair::from_bytes::<B>(&bytes[offset..]) {
            let class_name_start = offset + 5; // after 4bytes signature + separator 1 byte
            offset_class_names_map.insert(class_name_start, class_pair);
            offset += read_size;

            if offset + 5 > bytes.len() {
                break;
            }
        }

        Ok(Self(offset_class_names_map))
    }

    pub fn write_bytes<B: ByteOrder>(&self, bytes: &mut [u8]) -> Result<()> {
        let mut offset = 0;

        for (_, class) in &self.0 {
            let next = class.write_bytes::<B>(&mut bytes[offset..])?;
            offset += next;
        }

        while (offset % 16) != 0 {
            bytes[offset] = Self::CLASSNAMES_END_PADDING;
            offset += 1;
        }

        Ok(())
    }

    /// Converts the offset-based HashMap into a Vec.
    pub fn as_vec_ref<'this: 'a>(&'this self) -> Vec<&'a ClassPair<'a>> {
        self.0.values().collect()
    }
}

/// Hkx ClassNames section header Error Result
type Result<T, E = ClassNamesSectionError> = core::result::Result<T, E>;

/// Hkx ClassNames section Error
#[derive(Debug, thiserror::Error)]
pub enum ClassNamesSectionError {
    /// Insufficient buffer size to write class info
    #[error(
        "Insufficient buffer size to write class info. expected: {expected}, actual: {actual}"
    )]
    InsufficientWriteBufferSize { expected: usize, actual: usize },

    /// 4bytes(signature) + 1bytes(separator) + any bytes(className) = At least 5bytes required, but got {0}.
    #[error("4bytes(signature) + 1bytes(separator) + any bytes(className) = At least 5bytes required, but got {0}.")]
    InsufficientReadBufferSize(usize),

    /// Invalid separator byte: ClassName expected `0x09` after u32 signature (`{signature:#x}`), but got `{wrong_separator:#2x}`.
    #[error("Invalid separator byte: ClassName expected `0x09` after u32 signature (`{signature:#x}`), but got `{wrong_separator:#2x}`.")]
    InvalidSignatureSeparator { signature: u32, wrong_separator: u8 },

    /// Invalid str.
    #[error(transparent)]
    Utf8Error(#[from] core::str::Utf8Error),

    /// External io error.
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    /// Cstr non terminated string.
    #[error(transparent)]
    FromBytesUntilNulError(#[from] FromBytesUntilNulError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_class_pair_from_bytes_and_write_bytes() {
        // Create some sample bytes to test with
        let bytes = b"\x11\x22\x33\x44\x09test_class\0";

        // Test reading from bytes
        let (class_pair, read_size) =
            ClassPair::from_bytes::<zerocopy::LittleEndian>(bytes).unwrap();
        assert_eq!(class_pair.signature, 0x44332211);
        assert_eq!(class_pair.class_name.to_str().unwrap(), "test_class");
        assert_eq!(read_size, 16); // 4 bytes signature + 1 byte separator + 10 bytes class name + 1 byte null terminator

        // Test writing to bytes
        let mut output_bytes = [0u8; 16];
        let written_size = class_pair
            .write_bytes::<zerocopy::LittleEndian>(&mut output_bytes)
            .unwrap();
        assert_eq!(written_size, 16);
        assert_eq!(output_bytes.as_slice(), bytes.as_slice());
    }

    #[test]
    fn test_class_names_from_bytes_and_write_bytes() {
        let bytes = b"\x11\x22\x33\x44\x09test_class1\0\x44\x55\x66\x77\x09test_class2\0\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff";

        let class_names = ClassNames::from_bytes::<zerocopy::LittleEndian>(bytes).unwrap();
        assert_eq!(class_names.0.len(), 2);

        let test_class1_name_start = 5;
        let ClassPair {
            signature,
            class_name,
        } = class_names.0[&test_class1_name_start];
        assert_eq!(signature, 0x44332211);
        assert_eq!(class_name, c"test_class1");

        let test_class2_name_start = 22;
        let ClassPair {
            signature,
            class_name,
        } = class_names.0[&test_class2_name_start];
        assert_eq!(signature, 0x77665544);
        assert_eq!(class_name, c"test_class2");

        // Test writing to bytes
        let mut output_bytes = [0u8; 64];
        class_names
            .write_bytes::<zerocopy::LittleEndian>(&mut output_bytes)
            .unwrap();
        assert_eq!(&output_bytes[0..38], &bytes[0..38]);
        assert_eq!(&output_bytes[38..48], &[0xff; 10]); // Alignment is done by filling in with 0xFF up to multiples of 16bytes.
        assert_eq!(&output_bytes[48..], &[0x00; 16]); // Check after padding bytes
    }
}
