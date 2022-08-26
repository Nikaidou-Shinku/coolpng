use std::{fs, io::{Read, Write}, str::FromStr};

use anyhow::bail;

use crate::{chunk::Chunk, png::Png, chunk_type::ChunkType};

pub fn encode(
  file_path: String,
  chunk_type: String,
  message: String,
  output_file: Option<String>,
) -> super::Result<()> {
  let mut file = fs::File::open(file_path)?;
  let mut data = vec![];

  file.read_to_end(&mut data)?;
  let data: &[u8] = &data;

  let mut png = Png::try_from(data)?;

  let chunk_type = ChunkType::from_str(&chunk_type)?;
  png.append_chunk(Chunk::new(chunk_type, message.as_bytes().to_vec()));

  let output_file = output_file.unwrap_or("output.png".to_string());
  let mut output_file = fs::File::create(output_file)?;
  output_file.write_all(&png.as_bytes())?;
  Ok(())
}

pub fn decode(file_path: String, chunk_type: String) -> super::Result<()> {
  let mut file = fs::File::open(file_path)?;
  let mut data = vec![];

  file.read_to_end(&mut data)?;
  let data: &[u8] = &data;

  let png = Png::try_from(data)?;

  match png.chunk_by_type(&chunk_type) {
    Some(chunk) => println!("{}", chunk.data_as_string()?),
    None => bail!("Chunk not found"),
  }
  Ok(())
}

pub fn remove(file_path: String, chunk_type: String) -> super::Result<()> {
  let mut file = fs::File::open(&file_path)?;
  let mut data = vec![];

  file.read_to_end(&mut data)?;
  let data: &[u8] = &data;

  let mut png = Png::try_from(data)?;

  png.remove_chunk(&chunk_type)?;

  let mut file = fs::File::create(file_path)?;
  file.write_all(&png.as_bytes())?;
  Ok(())
}

pub fn print(file_path: String) -> super::Result<()> {
  let mut file = fs::File::open(file_path)?;
  let mut data = vec![];

  file.read_to_end(&mut data)?;
  let data: &[u8] = &data;

  let png = Png::try_from(data)?;

  for chunk in png.chunks() {
    println!("{}", chunk);
  }
  Ok(())
}
