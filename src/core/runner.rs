use crate::error::AppError;
use crate::gateway::{GenAiClient, Message};
use crate::model::{EvaluationBlock, RunEntry};
use crate::storage::Storage;
use chrono::Utc;
use futures::{stream, StreamExt};
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;

pub async fn run_blocks(
    storage: &impl Storage,
    blocks: Vec<EvaluationBlock>,
    client: Arc<impl GenAiClient + 'static>,
) -> Result<std::path::PathBuf, AppError> {
    let run_path = storage.next_run_path();
    use tokio::fs::File as AsyncFile;
    let file = Arc::new(Mutex::<AsyncFile>::new(storage.create_async_file(&run_path).await?));

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
                    Message {
                        role: "system".to_string(),
                        content: system_prompt,
                    },
                    Message {
                        role: "user".to_string(),
                        content: case.input.clone(),
                    },
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

    stream::iter(tasks)
        .buffer_unordered(10)
        .collect::<Vec<()>>()
        .await;

    Ok(run_path)
}
