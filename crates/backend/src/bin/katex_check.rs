//! KaTeX rendering validator — scans the database for problems with LaTeX
//! that will render incorrectly in KaTeX.
//!
//! Usage:
//!   cargo run --bin katex-check             # Scan all problems
//!   cargo run --bin katex-check -- --topic algebra1  # Scan specific topic
//!   echo '$\frac{x}$' | cargo run --bin katex-check --stdin  # Validate stdin
//!
//! Exits with code 1 if any errors are found, 0 otherwise.

use std::io::{self, BufRead, Write};

use locus_common::katex_validate::{
    Severity, ValidationResult, prepare_for_rendering, validate_and_fix, validate_katex,
};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.iter().any(|a| a == "--json-stdin") {
        run_json_stdin();
    } else if args.iter().any(|a| a == "--stdin") {
        run_stdin();
    } else {
        run_db_scan(&args);
    }
}

/// JSON mode: reads JSONL with {"q": "..."}, outputs {"issues": [...], "fixed": "..."|null}
fn run_json_stdin() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        let line = line.trim().to_string();
        if line.is_empty() {
            continue;
        }

        let parsed: serde_json::Value = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(_) => continue,
        };

        let q = parsed["q"].as_str().unwrap_or("");
        let (result, fix) = validate_and_fix(q);
        let prepared = prepare_for_rendering(q);

        let issues: Vec<serde_json::Value> = result
            .issues
            .iter()
            .map(|i| {
                serde_json::json!({
                    "code": i.code,
                    "severity": match i.severity { Severity::Error => "error", Severity::Warning => "warning" },
                    "message": i.message,
                })
            })
            .collect();

        let output = serde_json::json!({
            "id": parsed.get("id"),
            "topic": parsed.get("topic"),
            "issues": issues,
            "fixed": fix,
            "prepared": prepared,
        });

        let _ = writeln!(out, "{}", output);
    }
}

fn run_stdin() {
    let stdin = io::stdin();
    let mut total = 0;
    let mut errors = 0;
    let mut warnings = 0;

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        let line = line.trim().to_string();
        if line.is_empty() {
            continue;
        }

        total += 1;
        let result = validate_katex(&line);
        if !result.is_ok() {
            print_result(&line, &result, None);
            errors += result.error_count();
            warnings += result.warning_count();
        }
    }

    println!("\n--- Summary ---");
    println!("Checked: {}", total);
    println!("Errors:  {}", errors);
    println!("Warnings: {}", warnings);

    if errors > 0 {
        std::process::exit(1);
    }
}

fn run_db_scan(args: &[String]) {
    let topic_filter = args
        .windows(2)
        .find(|w| w[0] == "--topic")
        .map(|w| w[1].as_str());

    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        // Try .env file
        if let Ok(content) = std::fs::read_to_string(".env") {
            for line in content.lines() {
                if let Some(url) = line.strip_prefix("DATABASE_URL=") {
                    return url.to_string();
                }
            }
        }
        eprintln!("ERROR: DATABASE_URL not set. Set it or create a .env file.");
        std::process::exit(2);
    });

    // Use tokio for async DB access
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let pool = sqlx::PgPool::connect(&database_url)
            .await
            .unwrap_or_else(|e| {
                eprintln!("ERROR: Failed to connect to database: {}", e);
                std::process::exit(2);
            });

        let query = if let Some(topic) = topic_filter {
            format!(
                "SELECT id, question_latex, solution_latex, main_topic, subtopic \
                 FROM problems WHERE main_topic = '{}' ORDER BY main_topic, subtopic",
                topic.replace('\'', "''")
            )
        } else {
            "SELECT id, question_latex, solution_latex, main_topic, subtopic \
             FROM problems ORDER BY main_topic, subtopic"
                .to_string()
        };

        let rows: Vec<(uuid::Uuid, String, String, String, String)> = sqlx::query_as(&query)
            .fetch_all(&pool)
            .await
            .unwrap_or_else(|e| {
                eprintln!("ERROR: Query failed: {}", e);
                std::process::exit(2);
            });

        let total = rows.len();
        let mut problems_with_errors = 0;
        let mut problems_with_warnings = 0;
        let mut total_errors = 0;
        let mut total_warnings = 0;

        println!("Scanning {} problems...\n", total);

        for (id, question_latex, solution_latex, main_topic, subtopic) in &rows {
            let label = format!("{}/{}", main_topic, subtopic);

            // Validate question_latex
            let q_result = validate_katex(question_latex);
            if !q_result.is_ok() {
                let id_str = format!("{} [{}] question_latex", id, label);
                print_result(question_latex, &q_result, Some(&id_str));
                total_errors += q_result.error_count();
                total_warnings += q_result.warning_count();
                if q_result.has_errors() {
                    problems_with_errors += 1;
                } else {
                    problems_with_warnings += 1;
                }
            }

            // Validate solution_latex (if present)
            if !solution_latex.is_empty() {
                // Solutions can be multi-line (steps separated by newlines)
                for (i, step) in solution_latex.lines().enumerate() {
                    let step = step.trim();
                    if step.is_empty() {
                        continue;
                    }
                    let s_result = validate_katex(step);
                    if !s_result.is_ok() {
                        let id_str = format!("{} [{}] solution_latex step {}", id, label, i + 1);
                        print_result(step, &s_result, Some(&id_str));
                        total_errors += s_result.error_count();
                        total_warnings += s_result.warning_count();
                    }
                }
            }
        }

        println!("\n=== Summary ===");
        println!("Total problems scanned: {}", total);
        println!(
            "Problems with errors:   {} ({:.1}%)",
            problems_with_errors,
            if total > 0 {
                problems_with_errors as f64 / total as f64 * 100.0
            } else {
                0.0
            }
        );
        println!("Problems with warnings: {}", problems_with_warnings);
        println!("Total errors:           {}", total_errors);
        println!("Total warnings:         {}", total_warnings);

        if total_errors > 0 {
            std::process::exit(1);
        }
    });
}

fn print_result(content: &str, result: &ValidationResult, label: Option<&str>) {
    if let Some(label) = label {
        println!("--- {} ---", label);
    }

    // Show truncated content
    let display = if content.len() > 80 {
        format!("{}...", &content[..80])
    } else {
        content.to_string()
    };
    println!("  Content: {}", display);

    for issue in &result.issues {
        let marker = match issue.severity {
            Severity::Error => "\x1b[31mERROR\x1b[0m",
            Severity::Warning => "\x1b[33mWARN\x1b[0m",
        };
        println!("  [{}] {}: {}", marker, issue.code, issue.message);
    }
    println!();
}
