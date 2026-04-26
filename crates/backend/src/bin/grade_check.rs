//! Grade-check binary — reads JSONL from stdin, self-grades each answer_key.
//!
//! Input (one JSON per line):
//!   {"answer_key":"2*x","answer_type":"expression","grading_mode":"equivalent"}
//!
//! Output (one JSON per line):
//!   {"ok":true,"result":"Correct"}

use std::io::{self, BufRead, Write};

use locus_common::grader::{GradeResult, grade_answer};
use locus_common::{AnswerType, GradingMode};

fn main() {
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
            Err(e) => {
                let _ = writeln!(out, r#"{{"ok":false,"result":"parse error: {}"}}"#, e);
                continue;
            }
        };

        let answer_key = parsed["answer_key"].as_str().unwrap_or("");
        let answer_type_str = parsed["answer_type"].as_str().unwrap_or("expression");
        let grading_mode_str = parsed["grading_mode"].as_str().unwrap_or("equivalent");

        let answer_type = AnswerType::from_str(answer_type_str).unwrap_or_default();
        let grading_mode = match grading_mode_str {
            "factor" => GradingMode::Factor,
            "expand" => GradingMode::Expand,
            _ => GradingMode::Equivalent,
        };

        // Self-grade: answer_key against itself should always be Correct
        let result = grade_answer(answer_key, answer_key, answer_type, grading_mode);

        let (ok, result_str) = match &result {
            GradeResult::Correct => (true, "Correct".to_string()),
            GradeResult::Incorrect => (false, "Incorrect".to_string()),
            GradeResult::Invalid(msg) => (false, format!("Invalid: {}", msg)),
            GradeResult::Error(msg) => (false, format!("Error: {}", msg)),
        };

        let _ = writeln!(
            out,
            "{}",
            serde_json::json!({"ok": ok, "result": result_str})
        );
    }
}
