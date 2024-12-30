use clap::{Parser, Subcommand};
use dejavu::deduplicate;
use dejavu::errors::DeduplicationError;

// todo: these should be optional arguments
const DUPLICATE_THRESHOLD: u32 = 10;
const REPORT_FILE_NAME: &str = "dedup_report.json";

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
    fn run(self) -> Result<(), DeduplicationError> {
        match self.command {
            Commands::Deduplicate { directory } => {
                deduplicate::run(directory, DUPLICATE_THRESHOLD, REPORT_FILE_NAME)
            }
        }
    }
}

fn run() -> Result<(), DeduplicationError> {
    let cli = Cli::parse();
    cli.run()
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{}", error); // Print the error message
        std::process::exit(1);
    }
}
