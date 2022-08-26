use std::{fmt, str};

use anyhow::bail;
use crc::{Crc, CRC_32_ISO_HDLC};

use crate::chunk_type::ChunkType;

const CRC: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

struct Chunk {
  length: u32,
  chunk_type: ChunkType,
  data: Vec<u8>,
  crc: u32,
}

impl Chunk {
  fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
    let mut check_data = chunk_type.bytes().to_vec();
    check_data.extend_from_slice(&data);
    Chunk {
      length: data.len() as u32,
      chunk_type,
      data,
      crc: CRC.checksum(&check_data),
    }
  }
  fn length(&self) -> u32 {
    self.length
  }
  fn chunk_type(&self) -> &ChunkType {
    &self.chunk_type
  }
  fn data(&self) -> &[u8] {
    &self.data
  }
  fn crc(&self) -> u32 {
    self.crc
  }
  fn data_as_string(&self) -> super::Result<String> {
    let s = str::from_utf8(&self.data)?;
    Ok(s.to_string())
  }
  fn as_bytes(&self) -> Vec<u8> {
    let mut res = vec![];

    res.extend_from_slice(&self.length.to_be_bytes());
    res.extend_from_slice(&self.chunk_type.bytes());
    res.extend_from_slice(&self.data);
    res.extend_from_slice(&self.crc.to_be_bytes());

    res
  }
}

impl fmt::Display for Chunk {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let res = self.data_as_string().unwrap_or("Data is not a string".to_string());
    write!(f, "{}", res)
  }
}

impl TryFrom<&[u8]> for Chunk {
  type Error = super::Error;

  fn try_from(value: &[u8]) -> super::Result<Self> {
    let (length, value) = value.split_at(4);
    let length = u32::from_be_bytes(length.try_into()?);

    let (raw_data, value) = value.split_at(length as usize + 4);

    if value.len() != 4 {
      bail!("Invalid chunk length");
    }
    let crc = u32::from_be_bytes(value.try_into()?);

    if CRC.checksum(raw_data) != crc {
      bail!("Invalid CRC");
    }

    let (chunk_type, data) = raw_data.split_at(4);
    let tmp: [u8; 4] = chunk_type.try_into()?;
    let chunk_type = ChunkType::try_from(tmp)?;

    Ok(Self {
      length,
      chunk_type,
      data: data.to_vec(),
      crc,
    })
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
    let data = "This is where your secret message will be!".as_bytes().to_vec();
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
