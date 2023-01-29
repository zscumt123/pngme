use crate::chunk_type::ChunkType;
use crate::error::PngError;
use crate::utils::read_be_u32;
use crc::{Crc, CRC_32_ISO_HDLC};
use std::fmt::Display;

#[derive(Debug)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    chunk_data: Vec<u8>,
    crc: u32,
}
impl Chunk {
    const CHUNK_CRC: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let t = chunk_type
            .bytes()
            .iter()
            .chain(data.iter())
            .copied()
            .collect::<Vec<_>>();
        Self {
            length: data.len() as u32,
            chunk_type,
            chunk_data: data,
            crc: Self::CHUNK_CRC.checksum(&t),
        }
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        self.length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect::<Vec<_>>()
    }
    pub fn data(&self) -> &[u8] {
        &self.chunk_data
    }
    pub fn length(&self) -> u32 {
        self.length
    }
    pub fn crc(&self) -> u32 {
        self.crc
    }
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }
    pub fn data_as_string(&self) -> Result<String, anyhow::Error> {
        let s = String::from_utf8(self.chunk_data.to_vec())?;
        Ok(s)
    }
}
//
impl TryFrom<&[u8]> for Chunk {
    type Error = PngError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let source_len = value.len();
        let source_chunk_len = read_be_u32(&value[..4]);
        let chunk_data = &value[8..source_len - 4];
        let source_crc_val = read_be_u32(&value[source_len - 4..source_len]);
        //check length and crc
        if source_len < 12
            || (source_chunk_len as usize) != chunk_data.len()
            || source_crc_val != Self::CHUNK_CRC.checksum(&value[4..source_len - 4])
        {
            return Err(PngError::ChunkError);
        }

        let arr: [u8; 4] = (&value[4..8]).try_into().unwrap();
        Ok(Self {
            length: source_chunk_len,
            chunk_type: ChunkType::try_from(arr)?,
            chunk_data: chunk_data.try_into().unwrap(),
            crc: source_crc_val,
        })
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chunk_type_string = self.chunk_type.to_string();
        let chunk_data_string = String::from_utf8_lossy(self.data()).to_string();
        write!(
            f,
            "data_length:{},chunk_type:{},chunk_data:{}, crc:{}",
            self.length, chunk_type_string, chunk_data_string, self.crc
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
