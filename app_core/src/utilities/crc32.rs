use crc::{Algorithm, Crc};

/// Applied CRC32 hash function settings used in Skyrim.
///
/// See below
/// - https://www.nexusmods.com/skyrim/articles/50508/
pub fn crc32(bytes: &[u8]) -> u32 {
    const CUSTOM_ALG: Algorithm<u32> = Algorithm {
        width: 32,
        poly: 0x04C11DB7,
        init: 0,
        refin: true,
        refout: true,
        xorout: 0,
        check: 0,
        residue: 0,
    };

    Crc::<u32>::new(&CUSTOM_ALG).checksum(bytes)
}

#[cfg(test)]
mod tests {
    use crate::utilities::crc32::crc32;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_crc() {
        assert_eq!(crc32(b"hkx"), 0x9E13D1FA);
    }
}
