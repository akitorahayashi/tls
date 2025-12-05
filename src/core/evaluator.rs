use crate::error::AppError;
use crate::gateway::{GenAiClient, Message};
use crate::model::{EvalEntry, RunEntry};
use crate::storage::Storage;
use futures::{stream, StreamExt};
use serde::Deserialize;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;
use tokio::fs::File as AsyncFile;

#[derive(Deserialize)]
struct JudgeResponse {
    passed: bool,
    reason: String,
}

pub async fn evaluate_run(
    storage: &impl Storage,
    run_entries: Vec<RunEntry>,
    run_path: &std::path::Path,
    client: Arc<impl GenAiClient + 'static>,
) -> Result<std::path::PathBuf, AppError> {
    let judge_model = "gpt-4";

    let eval_path = storage.eval_path_for(run_path);
    let out = Arc::new(Mutex::<AsyncFile>::new(storage.create_async_file(&eval_path).await?));

    let mut tasks = Vec::new();

    for run in run_entries {
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
