use clap::Parser;
use unitconv::{run, Cli};

fn main() {
    let cli= Cli::parse();
    if let Err(e) = run(cli) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
