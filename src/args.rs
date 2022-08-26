use clap::{Subcommand, Parser};

#[derive(Parser)]
pub struct Args {
  #[clap(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
  Encode {
    file_path: String,
    chunk_type: String,
    message: String,
    output_file: Option<String>,
  },
  Decode {
    file_path: String,
    chunk_type: String,
  },
  Remove {
    file_path: String,
    chunk_type: String,
  },
  Print {
    file_path: String,
  },
}
