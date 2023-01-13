use std::{str::{self,FromStr}, fmt::Display};
use crate::error::PngError;




#[derive(PartialEq,Debug)]
pub struct ChunkType{
  inner: [u8;4]
}

impl ChunkType {
  pub fn bytes(&self) -> [u8;4] {
    self.inner
  }
  pub fn is_critical(&self) -> bool {
    let first = self.inner[0];
    first.is_ascii_uppercase()
  }

  pub fn is_public(&self) -> bool {
    let second = self.inner[1];
    second.is_ascii_uppercase()
  }

  pub fn is_reserved_bit_valid(&self) -> bool {
    let third = self.inner[2];
    third.is_ascii_uppercase()
  }

  pub fn is_safe_to_copy(&self) -> bool {
    let last = self.inner[3];
    last.is_ascii_lowercase()
  }

  pub fn is_valid(&self) -> bool {
    self.is_reserved_bit_valid()
    
  }

}



impl TryFrom<[u8;4]> for ChunkType {
  type Error = PngError;
  fn try_from(value: [u8;4]) -> Result<Self, Self::Error> {
    let flag = value.iter().all(|&x| x.is_ascii_alphabetic());
    if !flag {
      return Err(PngError::ChunkTypeError)
    }
    Ok(ChunkType{inner: value})
  }
}

impl FromStr  for ChunkType {
  type Err = PngError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if s.len() != 4 {
      return Err(PngError::ChunkTypeError)
    }
    let arr :[u8;4]= s.as_bytes().try_into().unwrap();
    ChunkType::try_from(arr)
  }
}


impl Display for ChunkType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let s = String::from_utf8(self.bytes().to_vec()).unwrap();
      write!(f,"{}",s)
  }
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes()); 
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
