use clap::Parser;
use unitconv::{run, Cli};

fn main() {
    let cli= Cli::parse();
    run(cli);
}
