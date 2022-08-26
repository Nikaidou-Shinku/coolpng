use std::{fmt, str};

use anyhow::bail;

#[derive(Debug)]
struct ChunkType {
  name: [u8; 4],
}

impl ChunkType {
  fn bytes(&self) -> [u8; 4] {
    self.name
  }
  fn is_valid(&self) -> bool {
    self.is_reserved_bit_valid()
  }
  fn is_critical(&self) -> bool {
    let test = self.name[0] & 32;
    test == 0
  }
  fn is_public(&self) -> bool {
    let test = self.name[1] & 32;
    test == 0
  }
  fn is_reserved_bit_valid(&self) -> bool {
    let test = self.name[2] & 32;
    test == 0
  }
  fn is_safe_to_copy(&self) -> bool {
    let test = self.name[3] & 32;
    test != 0
  }
}

impl PartialEq for ChunkType {
  fn eq(&self, other: &ChunkType) -> bool {
    self.name == other.name
  }
}

impl Eq for ChunkType { }

impl fmt::Display for ChunkType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = str::from_utf8(&self.name).unwrap();
    write!(f, "{}", s)
  }
}

impl TryFrom<[u8; 4]> for ChunkType {
  type Error = super::Error;

  fn try_from(value: [u8; 4]) -> super::Result<Self> {
    for item in value {
      if !item.is_ascii_alphabetic() {
        bail!("Invalid chunk type name");
      }
    }
    Ok(Self { name: value })
  }
}

impl str::FromStr for ChunkType {
  type Err = super::Error;

  fn from_str(s: &str) -> super::Result<Self> {
    let bytes = s.as_bytes();
    if bytes.len() != 4 {
      bail!("Invalid length");
    }
    for item in bytes {
      if !item.is_ascii_alphabetic() {
        bail!("Invalid chunk type name");
      }
    }
    let mut name = [0; 4];
    name.copy_from_slice(bytes);
    Ok(Self { name })
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
