mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use args::{Args, Commands};
use clap::Parser;

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
      
    }
    Commands::Decode { file_path, chunk_type } => {
      
    }
    Commands::Remove { file_path, chunk_type } => {
      
    }
    Commands::Print { file_path } => {
      
    }
  }

  Ok(())
}
