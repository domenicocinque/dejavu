mod commands;
mod utils;

use clap::{Parser, Subcommand};
use commands::deduplicate;

#[derive(Subcommand)]
enum Commands {
    /// Deduplicate images within a directory
    Deduplicate { directory: Option<String> },
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    fn run(self) {
        match self.command {
            Commands::Deduplicate { directory } => match directory {
                Some(dir) => {
                    if let Err(err) = deduplicate::run(&dir) {
                        eprintln!("Error: {:?}", err);
                        std::process::exit(1);
                    }
                }
                None => {
                    println!("Directory not provided");
                }
            },
        }
    }
}

fn main() {
    let cli = Cli::parse();

    cli.run();
}
