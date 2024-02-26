use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use std::io::{self, Read, Write};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct HkxVertexBufferVertexData {
    pub m_vector_data: Vec<[OrderedFloat<f32>; 4]>,
    pub m_float_data: Vec<OrderedFloat<f32>>,
    pub m_uint32_data: Vec<u32>,
    pub m_uint16_data: Vec<u16>,
    pub m_uint8_data: Vec<u8>,
    pub m_num_verts: u32,
    pub m_vector_stride: u32,
    pub m_float_stride: u32,
    pub m_uint32_stride: u32,
    pub m_uint16_stride: u32,
    pub m_uint8_stride: u32,
    pub signature: u32,
}

impl HkxVertexBufferVertexData {
    pub fn new() -> Self {
        HkxVertexBufferVertexData {
            m_vector_data: Vec::new(),
            m_float_data: Vec::new(),
            m_uint32_data: Vec::new(),
            m_uint16_data: Vec::new(),
            m_uint8_data: Vec::new(),
            m_num_verts: 0,
            m_vector_stride: 0,
            m_float_stride: 0,
            m_uint32_stride: 0,
            m_uint16_stride: 0,
            m_uint8_stride: 0,
            signature: 0xd72b6fd0,
        }
    }

    pub fn read<R: Read>(&mut self, reader: &mut R) -> io::Result<()> {
        for _ in 0..self.m_num_verts {
            let mut array = [OrderedFloat(0.0); 4];
            for i in array.iter_mut() {
                *i = reader.read_f32::<LittleEndian>()?.into();
            }
            self.m_vector_data.push(array);
        }

        for _ in 0..self.m_num_verts {
            self.m_float_data
                .push(reader.read_f32::<LittleEndian>()?.into());
        }

        for _ in 0..self.m_num_verts {
            self.m_uint32_data.push(reader.read_u32::<LittleEndian>()?);
        }

        for _ in 0..self.m_num_verts {
            self.m_uint16_data.push(reader.read_u16::<LittleEndian>()?);
        }

        for _ in 0..self.m_num_verts {
            self.m_uint8_data.push(reader.read_u8()?);
        }

        self.m_vector_stride = reader.read_u32::<LittleEndian>()?;
        self.m_float_stride = reader.read_u32::<LittleEndian>()?;
        self.m_uint32_stride = reader.read_u32::<LittleEndian>()?;
        self.m_uint16_stride = reader.read_u32::<LittleEndian>()?;
        self.m_uint8_stride = reader.read_u32::<LittleEndian>()?;
        Ok(())
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        for v in &self.m_vector_data {
            for i in v.iter().take(4) {
                writer.write_f32::<LittleEndian>((*i).into())?;
            }
        }

        for &f in &self.m_float_data {
            writer.write_f32::<LittleEndian>(*f)?;
        }

        for &u in &self.m_uint32_data {
            writer.write_u32::<LittleEndian>(u)?;
        }

        for &u in &self.m_uint16_data {
            writer.write_u16::<LittleEndian>(u)?;
        }

        for &u in &self.m_uint8_data {
            writer.write_u8(u)?;
        }

        writer.write_u32::<LittleEndian>(self.m_vector_stride)?;
        writer.write_u32::<LittleEndian>(self.m_float_stride)?;
        writer.write_u32::<LittleEndian>(self.m_uint32_stride)?;
        writer.write_u32::<LittleEndian>(self.m_uint16_stride)?;
        writer.write_u32::<LittleEndian>(self.m_uint8_stride)?;
        Ok(())
    }
}

impl Default for HkxVertexBufferVertexData {
    fn default() -> Self {
        HkxVertexBufferVertexData::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn serialize_and_deserialize() {
        // Create an instance of HkxVertexBufferVertexData for testing
        let original_data = HkxVertexBufferVertexData {
            m_vector_data: vec![
                [
                    OrderedFloat(1.0),
                    OrderedFloat(2.0),
                    OrderedFloat(3.0),
                    OrderedFloat(4.0),
                ],
                ([
                    OrderedFloat(5.0),
                    OrderedFloat(6.0),
                    OrderedFloat(7.0),
                    OrderedFloat(8.0),
                ]),
            ],
            m_float_data: vec![OrderedFloat(1.1), OrderedFloat(2.2), OrderedFloat(3.3)],
            m_uint32_data: vec![10, 20, 30],
            m_uint16_data: vec![100, 200, 300],
            m_uint8_data: vec![5, 10, 15],
            m_num_verts: 2,
            m_vector_stride: 16,
            m_float_stride: 12,
            m_uint32_stride: 4,
            m_uint16_stride: 2,
            m_uint8_stride: 1,
            signature: 0xd72b6fd0,
        };

        let mut buffer = String::new();
        let mut ser = quick_xml::se::Serializer::new(&mut buffer);
        ser.expand_empty_elements(true);

        original_data.serialize(ser).unwrap();
        assert_eq!(buffer, "");
    }
}
