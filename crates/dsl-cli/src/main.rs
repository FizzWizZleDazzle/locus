//! LocusDSL CLI — parse, generate, validate, batch problem generation

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "dsl-cli", about = "LocusDSL problem generation CLI")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Parse and validate a YAML problem file
    Parse {
        /// Path to .yaml problem file
        file: PathBuf,
    },

    /// Generate N problem instances from a YAML file
    Generate {
        /// Path to .yaml problem file
        file: PathBuf,
        /// Number of problems to generate
        #[arg(short = 'n', long, default_value = "1")]
        count: usize,
    },

    /// Run full 6-layer validation on a file or directory
    Validate {
        /// Path to .yaml file or directory of .yaml files
        path: PathBuf,
        /// Number of test generations per file
        #[arg(short, long, default_value = "10")]
        runs: usize,
    },

    /// Batch generate from a directory of YAML files
    Batch {
        /// Directory containing .yaml problem files
        dir: PathBuf,
        /// Output file (JSON or SQL)
        #[arg(short, long)]
        output: PathBuf,
        /// Problems per file
        #[arg(short = 'n', long, default_value = "10")]
        count: usize,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Parse { file } => {
            let yaml = std::fs::read_to_string(&file).unwrap_or_else(|e| {
                eprintln!("Failed to read {}: {e}", file.display());
                std::process::exit(1);
            });
            match locus_dsl::parse(&yaml) {
                Ok(spec) => {
                    println!("OK: {}/{}", spec.topic.main, spec.topic.sub);
                    println!("  Variables: {}", spec.variables.len());
                    println!("  Constraints: {}", spec.constraints.len());
                    if let Some(ref variants) = spec.variants {
                        println!("  Variants: {}", variants.len());
                    }
                }
                Err(e) => {
                    eprintln!("ERROR: {e}");
                    std::process::exit(1);
                }
            }
        }

        Command::Generate { file, count } => {
            let yaml = std::fs::read_to_string(&file).unwrap_or_else(|e| {
                eprintln!("Failed to read {}: {e}", file.display());
                std::process::exit(1);
            });
            let spec = match locus_dsl::parse(&yaml) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Parse error: {e}");
                    std::process::exit(1);
                }
            };

            let mut success = 0;
            let mut errors = 0;
            for _ in 0..count {
                match locus_dsl::generate(&spec) {
                    Ok(problem) => {
                        println!("{}", serde_json::to_string(&problem).unwrap());
                        success += 1;
                    }
                    Err(e) => {
                        eprintln!("Generation error: {e}");
                        errors += 1;
                    }
                }
            }
            eprintln!("Generated: {success}, Errors: {errors}");
        }

        Command::Validate { path, runs } => {
            eprintln!("TODO: validate {}", path.display());
            // Walk directory, parse each .yaml, generate `runs` times, report
        }

        Command::Batch { dir, output, count } => {
            eprintln!("TODO: batch {} -> {}", dir.display(), output.display());
        }
    }
}
