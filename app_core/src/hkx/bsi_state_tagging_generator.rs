use byteorder::{LittleEndian, ReadBytesExt as _, WriteBytesExt as _};
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use std::io::{Read, Write};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct BSiStateTaggingGenerator {
    pub m_p_default_generator: Option<hkbGenerator>,
    pub m_i_state_to_set_as: i32,
    pub m_i_priority: i32,
    pub signature: u32,
}

impl BSiStateTaggingGenerator {
    pub fn new() -> Self {
        BSiStateTaggingGenerator {
            m_i_state_to_set_as: 0,
            m_i_priority: 0,
            signature: 0xf0826fc1,
        }
    }

    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        reader.read_exact(&mut [0; 8])?; // Skip 8 bytes
        self.m_p_default_generator = Some(hkbGenerator::read_class_pointer(reader)?);
        self.m_i_state_to_set_as = reader.read_i32::<LittleEndian>()?;
        self.m_i_priority = reader.read_i32::<LittleEndian>()?;
        Ok(())
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {

        writer.write_all(&[0; 8])?; // Write 8 bytes of zeros
        if let Some(ref p_default_generator) = self.m_p_default_generator {
            p_default_generator.write_class_pointer(writer)?;
        } else {
            // Handle the case where m_p_default_generator is None
            // ...
        }
        writer.write_i32::<LittleEndian>(self.m_i_state_to_set_as)?;
        writer.write_i32::<LittleEndian>(self.m_i_priority)?;
        Ok(())
    }
}

impl Default for BSiStateTaggingGenerator {
    fn default() -> Self {
        BSiStateTaggingGenerator::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn serialize_and_deserialize() {
        let original_data = BSiStateTaggingGenerator {
            m_p_default_generator: Some(hkbGenerator::new()),
            m_i_state_to_set_as: 42,
            m_i_priority: 123,
            signature: 0xf0826fc1,
        };

        // Serialize to XML
        let serialized_xml = original_data.to_xml_string().unwrap();

        let deserialized_data: BSiStateTaggingGenerator =
            xml_from_str(&serialized_xml).expect("Failed to deserialize from XML");

        assert_eq!(original_data, deserialized_data);
    }
}
