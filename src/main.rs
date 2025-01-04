use clap::{Parser, Subcommand};
use idar::deduplicate;
use idar::errors::AppError;
use idar::removal;

#[derive(Subcommand)]
enum Commands {
    /// Deduplicate images within a directory.
    Deduplicate {
        /// Directory to scan for duplicates
        directory: String,

        /// Similarity threshold for detecting duplicates
        #[arg(short, long, default_value_t = 10)]
        duplicate_threshold: u32,

        /// Size of the hash to use for image comparison
        #[arg(short, long, default_value_t = 16)]
        hash_size: u32,

        /// Name of the file to save the deduplication report
        #[arg(short, long, default_value = "dedup_report.json")]
        report_file_name: String,
    },

    /// Remove duplicates from a directory based on a report file
    /// generated by the deduplicate command.
    Remove {
        report_file: String,
        output_dir: String,
    },
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    fn run(self) -> Result<(), AppError> {
        match self.command {
            Commands::Deduplicate {
                directory,
                duplicate_threshold,
                hash_size, 
                report_file_name,
            } => deduplicate::run(directory, duplicate_threshold, hash_size, &report_file_name),
            Commands::Remove {
                report_file,
                output_dir,
            } => removal::run(&report_file, &output_dir),
        }
    }
}

fn run() -> Result<(), AppError> {
    let cli = Cli::parse();
    cli.run()
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{}", error); 
        std::process::exit(1);
    }
}

