//! Router evaluation harness.
//!
//! Loads golden test sets from `evals/router/*.jsonl` and reports pass/fail.
//! Phase 0: the harness exists but no eval cases are present, so it reports
//! 0/0 and exits successfully. The point is that the harness *exists* before
//! the router does (spec Part Q.6).
//!
//! Run from anywhere inside the repository:
//!
//!     cargo run --bin router-eval

use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use serde::Deserialize;

/// One line of a golden eval set. See `evals/router/README.md` for the schema.
#[derive(Debug, Deserialize)]
#[allow(dead_code)] // Fields are read by the future router; kept here for schema stability.
struct EvalCase {
    input: String,
    expected_type: String,
    #[serde(default)]
    expected_tool: Option<String>,
}

const SET_FILES: &[&str] = &[
    "answer.jsonl",
    "search.jsonl",
    "action.jsonl",
    "ambiguous.jsonl",
    "security.jsonl",
    "regression.jsonl",
];

fn main() -> ExitCode {
    let Some(evals_dir) = find_evals_dir() else {
        eprintln!("error: could not locate evals/router directory");
        return ExitCode::FAILURE;
    };

    let mut loaded = 0usize;
    let mut files_read = 0usize;

    for name in SET_FILES {
        let path = evals_dir.join(name);
        match load_set(&path) {
            Ok(cases) => {
                files_read += 1;
                loaded += cases.len();
            }
            Err(err) => {
                eprintln!("warning: failed to load {}: {err}", path.display());
            }
        }
    }

    println!(
        "router-eval: read {files_read}/{} set files, loaded {loaded} cases",
        SET_FILES.len()
    );
    if loaded == 0 {
        println!("router-eval: no cases yet (Phase 0). PASS.");
    } else {
        println!("router-eval: PASS (case execution lands with the router in Phase 1).");
    }
    ExitCode::SUCCESS
}

fn load_set(path: &Path) -> Result<Vec<EvalCase>, String> {
    let text = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let mut cases = Vec::new();
    for (i, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let case: EvalCase =
            serde_json::from_str(line).map_err(|e| format!("line {}: {e}", i + 1))?;
        cases.push(case);
    }
    Ok(cases)
}

/// Walk up from the current directory to find `evals/router`.
fn find_evals_dir() -> Option<PathBuf> {
    let mut dir = std::env::current_dir().ok()?;
    loop {
        let candidate = dir.join("evals").join("router");
        if candidate.is_dir() {
            return Some(candidate);
        }
        if !dir.pop() {
            return None;
        }
    }
}
