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
        /// Skip self-grade and KaTeX validation (for batch generation of pre-validated YAMLs)
        #[arg(long)]
        fast: bool,
    },

    /// Run full validation on a file or directory
    Validate {
        /// Path to .yaml file or directory of .yaml files
        path: PathBuf,
        /// Number of test generations per file
        #[arg(short, long, default_value = "10")]
        runs: usize,
    },

    /// Batch generate from a directory of YAML files (parallel, NDJSON output)
    Batch {
        /// Directory containing .yaml problem files
        dir: PathBuf,
        /// Output file (NDJSON: one JSON object per line)
        #[arg(short, long)]
        output: PathBuf,
        /// Problems per file
        #[arg(short = 'n', long, default_value = "10")]
        count: usize,
        /// Skip self-grade and KaTeX validation (for pre-validated YAMLs)
        #[arg(long)]
        fast: bool,
        /// Number of rayon threads (default: all cores)
        #[arg(short = 'j', long)]
        threads: Option<usize>,
    },

    /// Use AI to generate new problem YAML files (concurrent)
    Ai {
        /// Topics, comma-separated (e.g. "calculus/derivative_rules,algebra1/quadratic_formula")
        topic: String,
        /// Difficulty levels, comma-separated (generates one per topic×difficulty combo)
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
        Command::Generate { file, count, fast } => cmd_generate(&file, count, fast),
        Command::Validate { path, runs } => cmd_validate(&path, runs),
        Command::Batch {
            dir,
            output,
            count,
            fast,
            threads,
        } => cmd_batch(&dir, &output, count, fast, threads),
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

fn cmd_generate(file: &PathBuf, count: usize, fast: bool) {
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
    let max_consecutive_errors = 5; // bail early if file is broken
    let mut consecutive_errors = 0;
    let gen_fn = if fast { locus_dsl::generate_fast } else { locus_dsl::generate };
    for _ in 0..count {
        match gen_fn(&spec) {
            Ok(problem) => {
                println!("{}", serde_json::to_string(&problem).unwrap());
                success += 1;
                consecutive_errors = 0;
            }
            Err(e) => {
                eprintln!("Error: {e}");
                consecutive_errors += 1;
                if consecutive_errors >= max_consecutive_errors {
                    eprintln!("Bailing after {max_consecutive_errors} consecutive errors");
                    break;
                }
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

fn cmd_batch(dir: &PathBuf, output: &PathBuf, count: usize, fast: bool, threads: Option<usize>) {
    use rayon::prelude::*;
    use std::io::{BufWriter, Write};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};

    let n_threads = threads.unwrap_or_else(|| {
        std::thread::available_parallelism().map(|n| n.get()).unwrap_or(8)
    });
    rayon::ThreadPoolBuilder::new()
        .num_threads(n_threads)
        .build_global()
        .ok();

    let files = collect_yaml_files(dir);
    if files.is_empty() {
        eprintln!("No .yaml files found in {}", dir.display());
        std::process::exit(1);
    }

    // Parse all files upfront
    let specs: Vec<(String, locus_dsl::spec::ProblemSpec)> = files
        .iter()
        .filter_map(|f| {
            let yaml = read_file(f);
            match locus_dsl::parse(&yaml) {
                Ok(s) => Some((f.display().to_string(), s)),
                Err(e) => {
                    eprintln!("Skip {}: {e}", f.display());
                    None
                }
            }
        })
        .collect();

    let n_specs = specs.len();
    let total_target = n_specs * count;
    eprintln!(
        "{n_specs} specs × {count} = {total_target} problems, {n_threads} threads{}",
        if fast { " [fast]" } else { "" }
    );

    let file_out = std::fs::File::create(output).unwrap_or_else(|e| {
        eprintln!("Failed to create {}: {e}", output.display());
        std::process::exit(1);
    });
    let writer = Arc::new(Mutex::new(BufWriter::with_capacity(1 << 20, file_out)));

    let gen_fn: fn(&locus_dsl::spec::ProblemSpec) -> Result<locus_dsl::ProblemOutput, _> =
        if fast { locus_dsl::generate_fast } else { locus_dsl::generate };

    let total_written = Arc::new(AtomicUsize::new(0));
    let total_errors = Arc::new(AtomicUsize::new(0));
    let files_done = Arc::new(AtomicUsize::new(0));

    specs.par_iter().for_each(|(name, spec)| {
        let mut buf = Vec::with_capacity(count * 1024);
        let mut ok = 0usize;
        let mut consecutive_errors = 0u32;

        for _ in 0..count {
            match gen_fn(spec) {
                Ok(p) => {
                    serde_json::to_writer(&mut buf, &p).unwrap();
                    buf.push(b'\n');
                    ok += 1;
                    consecutive_errors = 0;
                }
                Err(e) => {
                    consecutive_errors += 1;
                    total_errors.fetch_add(1, Ordering::Relaxed);
                    if consecutive_errors >= 5 {
                        eprintln!("Bail {name} after 5 consecutive errors: {e}");
                        break;
                    }
                }
            }
        }

        if !buf.is_empty() {
            let mut w = writer.lock().unwrap();
            w.write_all(&buf).unwrap();
        }

        total_written.fetch_add(ok, Ordering::Relaxed);
        let done = files_done.fetch_add(1, Ordering::Relaxed) + 1;
        if done % 25 == 0 || done == n_specs {
            eprintln!(
                "[{done}/{n_specs}] {} written, {} errors",
                total_written.load(Ordering::Relaxed),
                total_errors.load(Ordering::Relaxed)
            );
        }
    });

    writer.lock().unwrap().flush().unwrap();
    eprintln!(
        "Done: {} problems → {}, {} errors",
        total_written.load(Ordering::Relaxed),
        output.display(),
        total_errors.load(Ordering::Relaxed)
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
    let difficulties: Vec<&str> = difficulty.split(',').map(|s| s.trim()).collect();

    // Build cross-product: topic × difficulty × count, skip existing files
    let mut all_tasks: Vec<(String, String, usize)> = Vec::new();
    let mut skipped = 0;
    for topic in &topics {
        for diff in &difficulties {
            for i in 0..count {
                // Check if output file already exists
                if let Some(dir) = output {
                    let topic_dir = if topic.contains('/') {
                        let parts: Vec<&str> = topic.splitn(2, '/').collect();
                        dir.join(parts[0]).join(parts[1])
                    } else {
                        dir.join(topic.replace('/', "_"))
                    };
                    let filename = if i == 0 {
                        format!("{}.yaml", diff)
                    } else {
                        format!("{}_{}.yaml", diff, i + 1)
                    };
                    if topic_dir.join(&filename).exists() {
                        skipped += 1;
                        continue;
                    }
                }
                all_tasks.push((topic.to_string(), diff.to_string(), i));
            }
        }
    }

    let total = all_tasks.len();
    if skipped > 0 {
        eprintln!("Skipped {skipped} existing files");
    }
    eprintln!(
        "{total} to generate, concurrency {concurrency}",
    );
    if total == 0 {
        eprintln!("Nothing to generate — all files exist");
        return;
    }

    let rt = tokio::runtime::Runtime::new().unwrap();

    // Convert to (topic, index) pairs + separate difficulty vec for the API
    let task_pairs: Vec<(String, usize)> = all_tasks.iter().map(|(t, _, i)| (t.clone(), *i)).collect();
    let task_diffs: Vec<String> = all_tasks.iter().map(|(_, d, _)| d.clone()).collect();

    let results = rt.block_on(ai::generate_batch_multi_diff(
        &task_pairs,
        &task_diffs,
        &api_key,
        model,
        concurrency,
    ));

    let mut success = 0;
    let mut failed = 0;

    for ((topic, diff, idx), result) in all_tasks.iter().zip(results.iter()) {
        match result {
            Ok(yaml) => {
                if let Some(dir) = output {
                    // problems/main_topic/subtopic/difficulty.yaml
                    let topic_dir = if topic.contains('/') {
                        let parts: Vec<&str> = topic.splitn(2, '/').collect();
                        dir.join(parts[0]).join(parts[1])
                    } else {
                        dir.join(topic)
                    };
                    std::fs::create_dir_all(&topic_dir).ok();
                    let filename = if *idx == 0 {
                        format!("{}.yaml", diff)
                    } else {
                        format!("{}_{}.yaml", diff, idx + 1)
                    };
                    let file_path = topic_dir.join(&filename);
                    std::fs::write(&file_path, yaml).unwrap_or_else(|e| {
                        eprintln!("Failed to write {}: {e}", file_path.display());
                    });
                    eprintln!("Wrote {}", file_path.display());
                } else {
                    println!("--- {topic} ({diff}) #{} ---", idx + 1);
                    println!("{yaml}");
                }
                success += 1;
            }
            Err(e) => {
                eprintln!("{topic} ({diff}) #{} failed: {e}", idx + 1);
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
