use clap::Parser;
use unitconv::{run, Cli};

fn main() {
    let cli= Cli::parse();
    if let Err(e) = run(cli) {
        eprint!("Error: {}", e);
        std::process::exit(1);
    }
}
