use crate::error::AppError;
use crate::gateway::{Client, Message};
use crate::model::{EvalEntry, EvaluationBlock, RunEntry};
use crate::scaffold::{InitReport, ProjectLayout};
use chrono::Utc;
use futures::{stream, StreamExt};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs::File as AsyncFile;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;

const RUNS_DIR: &str = ".telescope/runs";
const EVALS_DIR: &str = ".telescope/evals";

pub fn init(project_root: &Path) -> Result<InitReport, AppError> {
    let layout = ProjectLayout::new(project_root);
    layout.init()
}

pub async fn run(
    project_root: &Path,
    with_metrics: bool,
    id: Option<&str>,
) -> Result<PathBuf, AppError> {
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

    let client = Arc::new(Client::new()?);
    let run_path = layout.next_run_path();
    let file = Arc::new(Mutex::new(AsyncFile::create(&run_path).await?));

    let mut tasks = Vec::new();
    for block in blocks {
        let block = Arc::new(block);
        for (idx, case) in block.dataset.iter().enumerate() {
            let block = Arc::clone(&block);
            let client = Arc::clone(&client);
            let case = case.clone();
            let file = Arc::clone(&file);

            tasks.push(async move {
                let system_prompt = block.prompts.system.clone();
                let messages = vec![
                    Message { role: "system".to_string(), content: system_prompt },
                    Message { role: "user".to_string(), content: case.input.clone() },
                ];

                let output_result = client.chat(&block.metadata.model, messages).await;

                let output = match output_result {
                    Ok(o) => o,
                    Err(e) => format!("Error: {}", e),
                };

                let entry = RunEntry {
                    block_id: block.metadata.id.clone(),
                    case_index: idx,
                    input: case.input.clone(),
                    expected: case.expected.clone(),
                    output: output.clone(),
                    timestamp: Utc::now(),
                };

                let line = match serde_json::to_string(&entry) {
                    Ok(l) => l,
                    Err(e) => {
                        eprintln!("Failed to serialize run entry: {}", e);
                        return;
                    }
                };

                let mut file_guard = file.lock().await;
                if let Err(e) = file_guard.write_all(format!("{}\n", line).as_bytes()).await {
                    eprintln!("Failed to write run entry: {}", e);
                }
            });
        }
    }

    stream::iter(tasks).buffer_unordered(10).collect::<Vec<()>>().await;

    Ok(run_path)
}

#[derive(Serialize, Deserialize)]
struct JudgeResponse {
    passed: bool,
    reason: String,
}

pub async fn eval(project_root: &Path) -> Result<PathBuf, AppError> {
    let layout = ProjectLayout::new(project_root);
    let run_path = latest_file(project_root.join(RUNS_DIR))?
        .ok_or_else(|| AppError::ConfigError("No run logs found".into()))?;

    let file = File::open(&run_path)?;
    let reader = BufReader::new(file);

    let client = Arc::new(Client::new()?);
    let judge_model = "gpt-4";

    let eval_path = layout.eval_path_for(&run_path);
    let out = Arc::new(Mutex::new(AsyncFile::create(&eval_path).await?));

    let mut tasks = Vec::new();

    for line_result in reader.lines() {
        let line = match line_result {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Failed to read line from run log: {}", e);
                continue;
            }
        };
        if line.trim().is_empty() {
            continue;
        }
        let run: RunEntry = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Failed to parse run entry from line '{}': {}", line, e);
                continue;
            }
        };

        let client = Arc::clone(&client);
        let out = Arc::clone(&out);

        tasks.push(async move {
            let passed;
            let output_reason;

            if let Some(expected) = &run.expected {
                let system_prompt = "You are an AI Judge. Evaluate the actual output against the expected output/criteria. Respond in JSON format with `passed` (boolean) and `reason` (string).";
                let user_content = format!(
                    "Input: {}\nExpected Criteria/Answer: {}\nActual Output: {}\n\nEvaluate if the Actual Output meets the Expected Criteria.",
                    run.input, expected, run.output
                );

                let messages = vec![
                    Message { role: "system".to_string(), content: system_prompt.to_string() },
                    Message { role: "user".to_string(), content: user_content },
                ];

                match client.chat(judge_model, messages).await {
                    Ok(response_content) => {
                        let clean_content = response_content
                            .trim()
                            .trim_start_matches("```json")
                            .trim_start_matches("```")
                            .trim_end_matches("```")
                            .trim();

                        match serde_json::from_str::<JudgeResponse>(clean_content) {
                            Ok(judge_res) => {
                                passed = judge_res.passed;
                                output_reason = judge_res.reason;
                            },
                            Err(e) => {
                                passed = false;
                                output_reason = format!("Failed to parse judge response: {}. Response was: {}", e, response_content);
                            }
                        }
                    },
                    Err(e) => {
                        passed = false;
                        output_reason = format!("Judge API call failed: {}", e);
                    }
                }

            } else {
                passed = true;
                output_reason = "No expectation provided.".to_string();
            }

            let eval = EvalEntry {
                block_id: run.block_id,
                case_index: run.case_index,
                expected: run.expected,
                output: run.output,
                passed,
                reason: Some(output_reason),
            };

            let line = match serde_json::to_string(&eval) {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("Failed to serialize eval entry: {}", e);
                    return;
                }
            };

            let mut out_guard = out.lock().await;
            if let Err(e) = out_guard.write_all(format!("{}\n", line).as_bytes()).await {
                eprintln!("Failed to write eval entry: {}", e);
            }
        });
    }

    stream::iter(tasks).buffer_unordered(10).collect::<Vec<()>>().await;

    Ok(eval_path)
}

pub async fn report(project_root: &Path) -> Result<PathBuf, AppError> {
    let run_path = latest_file(project_root.join(RUNS_DIR))?
        .ok_or_else(|| AppError::ConfigError("No run logs found".into()))?;
    let eval_path = latest_file(project_root.join(EVALS_DIR))?
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

fn latest_file(dir: PathBuf) -> Result<Option<PathBuf>, AppError> {
    let _ = match fs::read_dir(&dir) {
        Ok(rd) => rd,
        Err(_) => return Ok(None), // Or handle as error? The original code returned Option.
                                   // But the reviewer said: "The use of .ok()? will silently discard any I/O errors... propagate the io::Error".
                                   // The original code was: `fs::read_dir(dir).ok()?.filter_map(...).collect()`.
                                   // If read_dir failed, it returned None.
                                   // If I propagate error, I should match and return Err.
    };

    // Actually, if directory doesn't exist, read_dir fails. `ok()?` converts to None.
    // If permission denied, it also returns None.
    // I should check if dir exists first, or just propagate error?
    // Reviewer: "It would be more robust to propagate the io::Error".

    // But wait, `latest_file` signature was `Option<PathBuf>`.
    // Caller expects `Option`.
    // I changed signature to `Result<Option<PathBuf>, AppError>`.

    let mut entries: Vec<PathBuf> =
        fs::read_dir(dir)?.filter_map(|e| e.ok().map(|e| e.path())).collect();
    entries.sort();
    Ok(entries.pop())
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
