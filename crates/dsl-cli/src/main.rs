//! LocusDSL CLI — parse, generate, validate, batch, AI generation

mod ai;

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

    /// Run full validation on a file or directory
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

    /// Use AI to generate new problem YAML files (concurrent)
    Ai {
        /// Topics, comma-separated (e.g. "calculus/derivative_rules,algebra1/quadratic_formula")
        topic: String,
        /// Difficulty level
        #[arg(short, long, default_value = "medium")]
        difficulty: String,
        /// Output directory (default: stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// LLM model to use
        #[arg(short, long, default_value = "claude-sonnet-4-20250514")]
        model: String,
        /// Number of different problems to generate
        #[arg(short = 'n', long, default_value = "1")]
        count: usize,
        /// Max concurrent API requests
        #[arg(short = 'j', long, default_value = "5")]
        concurrency: usize,
    },
}

fn main() {
    dotenvy::dotenv().ok();
    let cli = Cli::parse();

    match cli.command {
        Command::Parse { file } => cmd_parse(&file),
        Command::Generate { file, count } => cmd_generate(&file, count),
        Command::Validate { path, runs } => cmd_validate(&path, runs),
        Command::Batch {
            dir,
            output,
            count,
        } => cmd_batch(&dir, &output, count),
        Command::Ai {
            topic,
            difficulty,
            output,
            model,
            count,
            concurrency,
        } => cmd_ai(&topic, &difficulty, output.as_deref(), &model, count, concurrency),
    }
}

fn cmd_parse(file: &PathBuf) {
    let yaml = read_file(file);
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

fn cmd_generate(file: &PathBuf, count: usize) {
    let yaml = read_file(file);
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

fn cmd_validate(path: &PathBuf, runs: usize) {
    let files = collect_yaml_files(path);
    if files.is_empty() {
        eprintln!("No .yaml files found in {}", path.display());
        std::process::exit(1);
    }

    let mut total_ok = 0;
    let mut total_err = 0;

    for file in &files {
        let yaml = read_file(file);
        let spec = match locus_dsl::parse(&yaml) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("FAIL {}: parse error: {e}", file.display());
                total_err += 1;
                continue;
            }
        };

        let mut file_ok = 0;
        let mut file_err = 0;
        for _ in 0..runs {
            match locus_dsl::generate(&spec) {
                Ok(_) => file_ok += 1,
                Err(e) => {
                    eprintln!("FAIL {}: {e}", file.display());
                    file_err += 1;
                }
            }
        }

        if file_err == 0 {
            println!("OK   {} ({}/{})", file.display(), file_ok, runs);
            total_ok += 1;
        } else {
            println!(
                "FAIL {} ({}/{} passed)",
                file.display(),
                file_ok,
                runs
            );
            total_err += 1;
        }
    }

    println!("\n{} files OK, {} files FAILED", total_ok, total_err);
    if total_err > 0 {
        std::process::exit(1);
    }
}

fn cmd_batch(dir: &PathBuf, output: &PathBuf, count: usize) {
    let files = collect_yaml_files(dir);
    if files.is_empty() {
        eprintln!("No .yaml files found in {}", dir.display());
        std::process::exit(1);
    }

    let mut all_problems = Vec::new();
    for file in &files {
        let yaml = read_file(file);
        let spec = match locus_dsl::parse(&yaml) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Skip {}: {e}", file.display());
                continue;
            }
        };

        for _ in 0..count {
            match locus_dsl::generate(&spec) {
                Ok(p) => all_problems.push(p),
                Err(e) => eprintln!("Error {}: {e}", file.display()),
            }
        }
    }

    let json = serde_json::to_string_pretty(&all_problems).unwrap();
    std::fs::write(output, &json).unwrap_or_else(|e| {
        eprintln!("Failed to write {}: {e}", output.display());
        std::process::exit(1);
    });
    eprintln!(
        "Wrote {} problems to {}",
        all_problems.len(),
        output.display()
    );
}

fn cmd_ai(
    topics_str: &str,
    difficulty: &str,
    output: Option<&std::path::Path>,
    model: &str,
    count: usize,
    concurrency: usize,
) {
    let api_key = std::env::var("FACTORY_AI_API_KEY")
        .or_else(|_| std::env::var("ANTHROPIC_API_KEY"))
        .unwrap_or_else(|_| {
            eprintln!("FACTORY_AI_API_KEY not set. Set it in .env or environment.");
            std::process::exit(1);
        });

    let topics: Vec<&str> = topics_str.split(',').map(|s| s.trim()).collect();
    let total = topics.len() * count;

    eprintln!(
        "Generating {count} YAML(s) x {} topic(s) = {total} total, concurrency {concurrency}",
        topics.len()
    );

    let rt = tokio::runtime::Runtime::new().unwrap();

    // Build all tasks: (topic, index) pairs
    let mut all_tasks: Vec<(String, usize)> = Vec::new();
    for topic in &topics {
        for i in 0..count {
            all_tasks.push((topic.to_string(), i));
        }
    }

    // Run all concurrently
    let results = rt.block_on(ai::generate_batch_multi(
        &all_tasks,
        difficulty,
        &api_key,
        model,
        concurrency,
    ));

    let mut success = 0;
    let mut failed = 0;

    for ((topic, idx), result) in all_tasks.iter().zip(results.iter()) {
        match result {
            Ok(yaml) => {
                if let Some(dir) = output {
                    let topic_dir = dir.join(topic.replace('/', "_"));
                    std::fs::create_dir_all(&topic_dir).ok();
                    let filename = format!("{}.yaml", idx + 1);
                    let file_path = topic_dir.join(&filename);
                    std::fs::write(&file_path, yaml).unwrap_or_else(|e| {
                        eprintln!("Failed to write {}: {e}", file_path.display());
                    });
                    eprintln!("Wrote {}", file_path.display());
                } else {
                    println!("--- {topic} #{} ---", idx + 1);
                    println!("{yaml}");
                }
                success += 1;
            }
            Err(e) => {
                eprintln!("{topic} #{} failed: {e}", idx + 1);
                failed += 1;
            }
        }
    }

    eprintln!("\nDone: {success}/{total} succeeded, {failed} failed");
    if failed > 0 && success == 0 {
        std::process::exit(1);
    }
}

fn read_file(path: &PathBuf) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("Failed to read {}: {e}", path.display());
        std::process::exit(1);
    })
}

fn collect_yaml_files(path: &PathBuf) -> Vec<PathBuf> {
    if path.is_file() {
        return vec![path.clone()];
    }
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                files.extend(collect_yaml_files(&p));
            } else if p.extension().map_or(false, |e| e == "yaml" || e == "yml") {
                files.push(p);
            }
        }
    }
    files.sort();
    files
}
