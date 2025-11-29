
mod conv;

use clap::{Parser, Subcommand};
use conv::{validate_arguments};

use crate::conv::{convert_temperature, list_unit};

#[derive(Parser)]
#[command(name = "unitconv", version = "1.0", about = "Aplikasi Konversi Unit")]
pub struct Cli {
  #[command(subcommand)]
  command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
  /// Konversi nilai unit
  Convert {
    /// Konversi unit awal
    #[arg(long)]
    from: Option<String>,

    /// Konversi unit tujuan
    #[arg(long)]
    to: Option<String>,

    /// Nilai yang akan dikonversi
    #[arg(long)]
    value: Option<String>
  },
  /// List unit konversi
  List
}

pub fn run(cli: Cli) -> Result<bool, String> {

  match cli.command {
    Some(Commands::Convert { from, to, value }) => {
      let (from_unit, to_unit, parsed_value) = validate_arguments(from, to, value)?;
      convert_temperature(&from_unit, &to_unit, &parsed_value);
    }
    Some(Commands::List) => {
      list_unit();
    }
    None => {
      println!("Perintah tidak valid. Gunakan `--help` untuk melihat daftar perintah");
    }
  }

  Ok(true)
}