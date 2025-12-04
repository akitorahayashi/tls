use crate::error::AppError;
use crate::model::{EvalEntry, EvaluationBlock, RunEntry};
use crate::scaffold::{InitReport, ProjectLayout};
use chrono::Utc;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

const RUNS_DIR: &str = ".telescope/runs";
const EVALS_DIR: &str = ".telescope/evals";

pub fn init(project_root: &Path) -> Result<InitReport, AppError> {
    let layout = ProjectLayout::new(project_root);
    layout.init()
}

pub fn run(project_root: &Path, with_metrics: bool, id: Option<&str>) -> Result<PathBuf, AppError> {
    let layout = ProjectLayout::new(project_root);
    let mut blocks = load_blocks(project_root.join("benchmarks"))?;

    if with_metrics || id.is_some() {
        blocks.extend(load_blocks(project_root.join("metrics"))?);
    }

    if let Some(target_id) = id {
        blocks.retain(|block| block.metadata.id == target_id);
    }

    if blocks.is_empty() {
        return Err(AppError::ConfigError("No evaluation blocks found".into()));
    }

    let run_path = layout.next_run_path();
    let mut file = File::create(&run_path)?;

    for block in blocks {
        for (idx, case) in block.dataset.iter().enumerate() {
            let output = format!("echo: {}", case.input);
            let entry = RunEntry {
                block_id: block.metadata.id.clone(),
                case_index: idx,
                input: case.input.clone(),
                expected: case.expected.clone(),
                output: output.clone(),
                timestamp: Utc::now(),
            };
            let line = serde_json::to_string(&entry)?;
            writeln!(file, "{line}")?;
        }
    }

    Ok(run_path)
}

pub fn eval(project_root: &Path) -> Result<PathBuf, AppError> {
    let layout = ProjectLayout::new(project_root);
    let run_path = latest_file(project_root.join(RUNS_DIR))
        .ok_or_else(|| AppError::ConfigError("No run logs found".into()))?;

    let file = File::open(&run_path)?;
    let reader = BufReader::new(file);
    let eval_path = layout.eval_path_for(&run_path);
    let mut out = File::create(&eval_path)?;

    for line in reader.lines() {
        let line = line?;
        let run: RunEntry = serde_json::from_str(&line)?;
        let passed = match &run.expected {
            Some(expected) => run.output.trim() == expected.trim(),
            None => true,
        };
        let eval = EvalEntry {
            block_id: run.block_id,
            case_index: run.case_index,
            expected: run.expected,
            output: run.output,
            passed,
        };
        let line = serde_json::to_string(&eval)?;
        writeln!(out, "{line}")?;
    }

    Ok(eval_path)
}

pub fn report(project_root: &Path) -> Result<PathBuf, AppError> {
    let run_path = latest_file(project_root.join(RUNS_DIR))
        .ok_or_else(|| AppError::ConfigError("No run logs found".into()))?;
    let eval_path = latest_file(project_root.join(EVALS_DIR))
        .ok_or_else(|| AppError::ConfigError("No eval logs found".into()))?;

    let eval_entries = read_jsonl::<EvalEntry>(&eval_path)?;

    let total = eval_entries.len();
    let passed = eval_entries.iter().filter(|e| e.passed).count();
    let failed_cases: Vec<&EvalEntry> = eval_entries.iter().filter(|e| !e.passed).collect();

    let mut content = String::new();
    content.push_str("# Telescope Report\n\n");
    content.push_str(&format!("Run log: {}\n", run_path.display()));
    content.push_str(&format!("Eval log: {}\n\n", eval_path.display()));
    content.push_str(&format!("Total cases: {total}\n"));
    content.push_str(&format!("Passed: {passed}\n"));
    content.push_str(&format!("Failed: {}\n\n", total - passed));

    if !failed_cases.is_empty() {
        content.push_str("## Failures\n");
        for fail in failed_cases {
            content.push_str(&format!(
                "- {} case {}: expected {:?}, got {}\n",
                fail.block_id, fail.case_index, fail.expected, fail.output
            ));
        }
    }

    let report_path = ProjectLayout::new(project_root).next_report_path();
    let mut file = File::create(&report_path)?;
    file.write_all(content.as_bytes())?;

    Ok(report_path)
}

fn load_blocks(dir: PathBuf) -> Result<Vec<EvaluationBlock>, AppError> {
    if !dir.exists() {
        return Ok(Vec::new());
    }

    let mut blocks = Vec::new();
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.extension().is_some_and(|ext| ext == "json") {
            let content = fs::read_to_string(&path)?;
            let block: EvaluationBlock = serde_json::from_str(&content)?;
            blocks.push(block);
        }
    }
    Ok(blocks)
}

fn latest_file(dir: PathBuf) -> Option<PathBuf> {
    let mut entries: Vec<PathBuf> =
        fs::read_dir(dir).ok()?.filter_map(|e| e.ok().map(|e| e.path())).collect();
    entries.sort();
    entries.pop()
}

fn read_jsonl<T: for<'de> serde::Deserialize<'de>>(path: &Path) -> Result<Vec<T>, AppError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut items = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let value: T = serde_json::from_str(&line)?;
        items.push(value);
    }
    Ok(items)
}
