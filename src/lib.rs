
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "unitconv", version = "1.0", about = "Aplikasi Konversi Unit")]
pub struct Cli {
  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
  /// Konversi nilai unit
  Convert {
    #[arg(long)]
    from: Option<String>,

    #[arg(long)]
    to: Option<String>,

    #[arg(long)]
    value: Option<f64>
  }
}

pub fn run(cli: Cli) -> Result<bool, String>{
  Ok(true)
}