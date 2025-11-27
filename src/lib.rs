
use clap::{Parser, Subcommand};

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
  }
}

pub fn validate_arguments(from: Option<String>, to: Option<String>, value: Option<String>) -> Result<(String, String, f64), String> {
  if let None = from {
    return Err("Nilai `from` wajib diisi.".to_string());
  }

  if let None = to {
    return Err("Nilai `to` wajib diisi.".to_string());
  }

  if let None = value {
    return Err("Nilai `value` wajib diisi.".to_string());
  }

  let parsed_value = match value.unwrap().to_string().parse::<f64>() {
      Ok(v) => v,
      Err(_) => return Err("Pastikan value adalah number".to_string())
  };

  let from_unwrap = from.unwrap();
  let to_unwrap = to.unwrap();

  return Ok((from_unwrap, to_unwrap, parsed_value));
}

pub fn run(cli: Cli) -> Result<bool, String> {

  match cli.command {
    Some(Commands::Convert { from, to, value }) => {
      let (from_unwrap, to_unwrap, parsed_value) = match validate_arguments(from, to, value) {
          Ok(validated) => validated,
          Err(e) => return Err(e.to_string())
      };

      print!("from: {}, to: {}, value: {}", from_unwrap, to_unwrap, parsed_value);
    }
    None => {
      print!("Perintah tidak valid. Gunakan `--help` untuk melihat daftar perintah");
    }
  }

  Ok(true)
}