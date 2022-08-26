mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use args::{Args, Commands};
use clap::Parser;
use commands::*;

pub type Error = anyhow::Error;
pub type Result<T> = anyhow::Result<T>;

fn main() -> Result<()> {
  let cli = Args::parse();

  match cli.command {
    Commands::Encode {
      file_path,
      chunk_type,
      message,
      output_file,
    } => {
      if let Err(err) = encode(
        file_path,
        chunk_type,
        message,
        output_file
      ) {
        eprintln!("[Error] {}", err);
      }
    }
    Commands::Decode { file_path, chunk_type } => {
      if let Err(err) = decode(file_path, chunk_type) {
        eprintln!("[Error] {}", err);
      }
    }
    Commands::Remove { file_path, chunk_type } => {
      if let Err(err) = remove(file_path, chunk_type) {
        eprintln!("[Error] {}", err);
      }
    }
    Commands::Print { file_path } => {
      if let Err(err) = print(file_path) {
        eprintln!("[Error] {}", err);
      }
    }
  }

  Ok(())
}
