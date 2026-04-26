//! LocusDSL CLI — parse, generate, validate, batch, AI generation

mod ai;
mod upload;

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
        /// Executor: `auto` (try enumerate, fall back), `cpu`, `gpu`, or `legacy`
        #[arg(long, default_value = "auto")]
        executor: String,
    },

    /// Audit YAMLs: render N samples each, scan for banlist patterns and orphan
    /// variable names. Reports which files would fail today's validators so you
    /// can hand-fix or regenerate before re-running batch generation.
    Audit {
        /// Path to .yaml file or directory of .yaml files
        path: PathBuf,
        /// Number of seeds to render per YAML
        #[arg(short = 'n', long, default_value = "8")]
        runs: usize,
        /// Print full output for each failing file
        #[arg(short, long)]
        verbose: bool,
    },

    /// Verify enumerator output against legacy rejection sampling on a YAML.
    /// Asserts every legacy `(question, answer)` pair appears in the
    /// enumerator's full unique set (modulo the GPU-eligibility check).
    Verify {
        /// Path to .yaml file or directory of .yaml files
        path: PathBuf,
        /// Legacy samples per file (more = stricter coverage assertion)
        #[arg(long, default_value = "200")]
        legacy_samples: usize,
        /// Enumerator target per file
        #[arg(long, default_value = "10000")]
        enum_target: usize,
        /// Max files to verify (random sample if directory has more)
        #[arg(long, default_value = "50")]
        max_files: usize,
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
        #[arg(short, long, default_value = "claude-sonnet-4-6")]
        model: String,
        /// Number of different problems to generate
        #[arg(short = 'n', long, default_value = "1")]
        count: usize,
        /// Max concurrent API requests
        #[arg(short = 'j', long, default_value = "5")]
        concurrency: usize,
    },
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let cli = Cli::parse();

    let uploader = match upload::Uploader::from_env() {
        Ok(opt) => opt,
        Err(e) => {
            eprintln!("Uploader init failed (continuing with inline SVG): {e}");
            None
        }
    };

    match cli.command {
        Command::Parse { file } => cmd_parse(&file),
        Command::Generate { file, count, fast } => {
            cmd_generate(&file, count, fast, &uploader).await
        }
        Command::Validate { path, runs } => cmd_validate(&path, runs),
        Command::Audit {
            path,
            runs,
            verbose,
        } => cmd_audit(&path, runs, verbose),
        Command::Batch {
            dir,
            output,
            count,
            fast,
            threads,
            executor,
        } => cmd_batch(&dir, &output, count, fast, threads, &executor),
        Command::Verify {
            path,
            legacy_samples,
            enum_target,
            max_files,
        } => cmd_verify(&path, legacy_samples, enum_target, max_files),
        Command::Ai {
            topic,
            difficulty,
            output,
            model,
            count,
            concurrency,
        } => cmd_ai(
            &topic,
            &difficulty,
            output.as_deref(),
            &model,
            count,
            concurrency,
        ),
    }
}

fn cmd_parse(file: &PathBuf) {
    let yaml = read_file(file);
    match locus_dsl::parse(&yaml) {
        Ok(spec) => {
            println!("OK: {}/{}", spec.topic.main, spec.topic.sub);
            println!("  Variants: {}", spec.variants.len());
            for v in &spec.variants {
                println!(
                    "    - {} ({} vars, {} constraints)",
                    v.name,
                    v.variables.len(),
                    v.constraints.len()
                );
            }
        }
        Err(e) => {
            eprintln!("ERROR: {e}");
            std::process::exit(1);
        }
    }
}

async fn cmd_generate(
    file: &PathBuf,
    count: usize,
    fast: bool,
    uploader: &Option<upload::Uploader>,
) {
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
    let max_consecutive_errors = 5;
    let mut consecutive_errors = 0;
    let gen_fn = if fast {
        locus_dsl::generate_random_fast
    } else {
        locus_dsl::generate_random
    };
    for _ in 0..count {
        match gen_fn(&spec) {
            Ok(mut problem) => {
                if let Err(e) = upload_question_image(&mut problem, uploader.as_ref()).await {
                    eprintln!("Upload error: {e}");
                }
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

/// If the rendered `question_image_url` field still holds an inline
/// (compressed) SVG and an uploader is available, push the bytes to the
/// bucket and replace the field with the resulting public URL.
async fn upload_question_image(
    problem: &mut locus_dsl::ProblemOutput,
    uploader: Option<&upload::Uploader>,
) -> Result<(), String> {
    let Some(up) = uploader else {
        return Ok(());
    };
    let img = std::mem::take(&mut problem.question_image_url);
    if img.is_empty() {
        return Ok(());
    }
    if img.starts_with("http://") || img.starts_with("https://") {
        problem.question_image_url = img;
        return Ok(());
    }
    let svg = locus_common::svg_compress::decompress_svg(&img);
    let url = up.put(svg.as_bytes(), "svg", "image/svg+xml").await?;
    problem.question_image_url = url;
    Ok(())
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
            match locus_dsl::generate_random(&spec) {
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
            println!("FAIL {} ({}/{} passed)", file.display(), file_ok, runs);
            total_err += 1;
        }
    }

    println!("\n{} files OK, {} files FAILED", total_ok, total_err);
    if total_err > 0 {
        std::process::exit(1);
    }
}

fn cmd_audit(path: &PathBuf, runs: usize, verbose: bool) {
    let files = collect_yaml_files(path);
    if files.is_empty() {
        eprintln!("No .yaml files found in {}", path.display());
        std::process::exit(1);
    }

    let mut total_ok = 0;
    let mut failures: Vec<(PathBuf, Vec<String>)> = Vec::new();

    for file in &files {
        let yaml = read_file(file);
        match ai::audit_yaml(&yaml, runs) {
            Ok(()) => total_ok += 1,
            Err(issues) => failures.push((file.clone(), issues)),
        }
    }

    // Group failures by leading issue category so the user sees a histogram —
    // makes it obvious whether one bug is responsible for many files.
    let mut by_category: std::collections::BTreeMap<&str, usize> =
        std::collections::BTreeMap::new();
    for (_, issues) in &failures {
        if let Some(first) = issues.first() {
            // Take everything before the first " — " or ":" as the category.
            let category = first
                .split_once(" — ")
                .map(|(a, _)| a)
                .or_else(|| first.split_once(": ").map(|(_, b)| b))
                .unwrap_or(first.as_str());
            *by_category.entry(category).or_insert(0) += 1;
        }
    }

    println!("=== AUDIT SUMMARY ===");
    println!(
        "{} OK, {} FAILED of {} total",
        total_ok,
        failures.len(),
        files.len()
    );
    if !by_category.is_empty() {
        println!("\nFailure categories (first issue per file):");
        let mut sorted: Vec<_> = by_category.iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(a.1));
        for (cat, n) in sorted {
            println!("  {n:4} × {cat}");
        }
    }

    if verbose && !failures.is_empty() {
        println!("\n=== PER-FILE DETAILS ===");
        for (path, issues) in &failures {
            println!("\n{}:", path.display());
            for issue in issues {
                println!("  - {issue}");
            }
        }
    } else if !failures.is_empty() {
        println!("\nFiles failing (first 20):");
        for (path, issues) in failures.iter().take(20) {
            println!(
                "  {} : {}",
                path.display(),
                issues.first().map(|s| s.as_str()).unwrap_or("")
            );
        }
        if failures.len() > 20 {
            println!(
                "  … and {} more (re-run with -v for full list)",
                failures.len() - 20
            );
        }
    }

    if !failures.is_empty() {
        std::process::exit(1);
    }
}

fn cmd_batch(
    dir: &PathBuf,
    output: &PathBuf,
    count: usize,
    fast: bool,
    threads: Option<usize>,
    executor_str: &str,
) {
    use rayon::prelude::*;
    use std::io::{BufWriter, Write};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};

    let executor = match executor_str {
        "auto" => Some(locus_dsl::Executor::Auto),
        "cpu" => Some(locus_dsl::Executor::Cpu),
        "gpu" => Some(locus_dsl::Executor::Gpu),
        "legacy" => None,
        other => {
            eprintln!("Unknown --executor '{other}' (auto|cpu|gpu|legacy)");
            std::process::exit(2);
        }
    };

    let n_threads = threads.unwrap_or_else(|| {
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(8)
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
        "{n_specs} specs × {count} = {total_target} problems, {n_threads} threads, executor={executor_str}{}",
        if fast { " [fast]" } else { "" }
    );

    let file_out = std::fs::File::create(output).unwrap_or_else(|e| {
        eprintln!("Failed to create {}: {e}", output.display());
        std::process::exit(1);
    });
    let writer = Arc::new(Mutex::new(BufWriter::with_capacity(1 << 20, file_out)));

    let gen_fn: fn(&locus_dsl::spec::ProblemSpec) -> Result<locus_dsl::ProblemOutput, _> = if fast {
        locus_dsl::generate_random_fast
    } else {
        locus_dsl::generate_random
    };

    let total_written = Arc::new(AtomicUsize::new(0));
    let total_errors = Arc::new(AtomicUsize::new(0));
    let files_done = Arc::new(AtomicUsize::new(0));
    let total_fallback = Arc::new(AtomicUsize::new(0));

    specs.par_iter().for_each(|(name, spec)| {
        let mut buf = Vec::with_capacity(count * 1024);
        let mut ok = 0usize;

        // Try enumeration path (skipped if --executor=legacy)
        let enumerated: Option<Vec<locus_dsl::ProblemOutput>> = match executor {
            Some(exec) => match locus_dsl::enumerate_problems(spec, count, exec) {
                Ok(Some(rows)) => Some(rows),
                Ok(None) => None,
                Err(e) => {
                    eprintln!("Enumerate {name}: {e} — falling back");
                    None
                }
            },
            None => None,
        };

        if let Some(rows) = enumerated {
            for p in rows.iter().take(count) {
                serde_json::to_writer(&mut buf, p).unwrap();
                buf.push(b'\n');
                ok += 1;
            }
        } else {
            if executor.is_some() {
                total_fallback.fetch_add(1, Ordering::Relaxed);
            }
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
        }

        if !buf.is_empty() {
            let mut w = writer.lock().unwrap();
            w.write_all(&buf).unwrap();
        }

        total_written.fetch_add(ok, Ordering::Relaxed);
        let done = files_done.fetch_add(1, Ordering::Relaxed) + 1;
        if done % 25 == 0 || done == n_specs {
            eprintln!(
                "[{done}/{n_specs}] {} written, {} errors, {} fallback",
                total_written.load(Ordering::Relaxed),
                total_errors.load(Ordering::Relaxed),
                total_fallback.load(Ordering::Relaxed)
            );
        }
    });

    writer.lock().unwrap().flush().unwrap();
    eprintln!(
        "Done: {} problems → {}, {} errors, {} fallback",
        total_written.load(Ordering::Relaxed),
        output.display(),
        total_errors.load(Ordering::Relaxed),
        total_fallback.load(Ordering::Relaxed)
    );
}

fn cmd_verify(path: &PathBuf, legacy_samples: usize, enum_target: usize, max_files: usize) {
    use std::collections::HashSet;
    let files: Vec<PathBuf> = collect_yaml_files(path)
        .into_iter()
        .take(max_files)
        .collect();
    if files.is_empty() {
        eprintln!("No .yaml files found in {}", path.display());
        std::process::exit(1);
    }

    let mut total_ok = 0;
    let mut total_fail = 0;
    let mut total_skipped = 0;
    let mut details: Vec<(String, String)> = Vec::new();

    for file in &files {
        let yaml = read_file(file);
        let spec = match locus_dsl::parse(&yaml) {
            Ok(s) => s,
            Err(e) => {
                details.push((file.display().to_string(), format!("parse: {e}")));
                total_fail += 1;
                continue;
            }
        };

        // Enumerator full unique set
        let enum_rows =
            match locus_dsl::enumerate_problems(&spec, enum_target, locus_dsl::Executor::Cpu) {
                Ok(Some(rows)) => rows,
                Ok(None) => {
                    total_skipped += 1;
                    continue;
                }
                Err(e) => {
                    details.push((file.display().to_string(), format!("enum err: {e}")));
                    total_fail += 1;
                    continue;
                }
            };
        let enum_set: HashSet<(String, String)> = enum_rows
            .iter()
            .map(|p| (p.question_latex.clone(), p.answer_key.clone()))
            .collect();

        // Legacy random samples
        let mut legacy_set: HashSet<(String, String)> = HashSet::new();
        for _ in 0..legacy_samples {
            if let Ok(p) = locus_dsl::generate_random_fast(&spec) {
                legacy_set.insert((p.question_latex, p.answer_key));
            }
        }

        let missing: Vec<_> = legacy_set.difference(&enum_set).cloned().collect();
        if missing.is_empty() {
            total_ok += 1;
        } else {
            total_fail += 1;
            details.push((
                file.display().to_string(),
                format!(
                    "legacy − enum = {} missing pairs (first: {:?})",
                    missing.len(),
                    missing.first()
                ),
            ));
        }
    }

    println!(
        "verified: {} OK, {} FAIL, {} SKIPPED",
        total_ok, total_fail, total_skipped
    );
    for (path, msg) in details.iter().take(20) {
        println!("  {path}: {msg}");
    }
    if total_fail > 0 {
        std::process::exit(1);
    }
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
    eprintln!("{total} to generate, concurrency {concurrency}",);
    if total == 0 {
        eprintln!("Nothing to generate — all files exist");
        return;
    }

    let rt = tokio::runtime::Runtime::new().unwrap();

    // Convert to (topic, index) pairs + separate difficulty vec for the API
    let task_pairs: Vec<(String, usize)> =
        all_tasks.iter().map(|(t, _, i)| (t.clone(), *i)).collect();
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
