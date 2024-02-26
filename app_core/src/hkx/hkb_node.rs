use byteorder::{LittleEndian, ReadBytesExt as _, WriteBytesExt as _};
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use std::io::{Read, Write};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct hkbNode {
    pub m_user_data: u64,
    pub m_name: String,
    m_id: i16,
    m_clone_state: i8,
    #[serde(default)]
    pub m_pad_node: Vec<bool>,
    pub signature: u32,
}

impl hkbNode {
    pub fn new() -> Self {
        hkbNode {
            m_user_data: 0,
            m_name: String::new(),
            m_id: 0,
            m_clone_state: 0,
            m_pad_node: vec![false; 1],
            signature: 0x6d26f61d,
        }
    }

    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.m_user_data = reader.read_u64::<LittleEndian>()?;
        self.m_name = read_string_pointer(reader)?;
        self.m_id = reader.read_i16::<LittleEndian>()?;
        self.m_clone_state = reader.read_i8()?;
        self.m_pad_node = read_boolean_c_style_array(reader, 1)?;
        reader.read_exact(&mut [0; 4])?; // Skip 4 bytes
        Ok(())
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_u64::<LittleEndian>(self.m_user_data)?;
        writer.write(&self.m_name.as_bytes())?;
        writer.write_i16::<LittleEndian>(self.m_id)?;
        writer.write_i8(self.m_clone_state)?;
        writer.write(&self.m_pad_node())?;
        writer.write_all(&[0; 4])?; // Write 4 bytes of zeros
        Ok(())
    }
}

impl Default for hkbNode {
    fn default() -> Self {
        hkbNode::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn serialize_and_deserialize() {
        let original_data = hkbNode {
            m_user_data: 123,
            m_name: String::from("TestNode"),
            m_id: 42,
            m_clone_state: 1,
            m_pad_node: vec![false; 1],
            signature: 0x6d26f61d,
        };

        assert_eq!(original_data, deserialized_data);
    }
}
