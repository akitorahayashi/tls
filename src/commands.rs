use crate::core::{evaluate_run, generate_report, run_blocks};
use crate::error::AppError;
use crate::gateway::Client;
use crate::model::{EvalEntry, RunEntry};
use crate::storage::{FileStorage, InitReport, Storage};
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub fn init(project_root: &Path) -> Result<InitReport, AppError> {
    let storage = FileStorage::new(project_root);
    storage.init()
}

pub async fn run(
    project_root: &Path,
    with_metrics: bool,
    id: Option<&str>,
) -> Result<PathBuf, AppError> {
    let storage = FileStorage::new(project_root);
    let mut blocks = storage.load_blocks(&storage.benchmarks_dir())?;

    if with_metrics || id.is_some() {
        blocks.extend(storage.load_blocks(&storage.metrics_dir())?);
    }

    if let Some(target_id) = id {
        blocks.retain(|block| block.metadata.id == target_id);
    }

    if blocks.is_empty() {
        return Err(AppError::ConfigError("No evaluation blocks found".into()));
    }

    let client = Arc::new(Client::new()?);

    run_blocks(&storage, blocks, client).await
}

pub async fn eval(project_root: &Path) -> Result<PathBuf, AppError> {
    let storage = FileStorage::new(project_root);
    let run_path = storage
        .find_latest_run_file()?
        .ok_or_else(|| AppError::ConfigError("No run logs found".into()))?;

    let run_entries: Vec<RunEntry> = storage.read_jsonl_run_entries(&run_path)?;

    let client = Arc::new(Client::new()?);

    evaluate_run(&storage, run_entries, &run_path, client).await
}

pub async fn report(project_root: &Path) -> Result<PathBuf, AppError> {
    let storage = FileStorage::new(project_root);
    let run_path = storage
        .find_latest_run_file()?
        .ok_or_else(|| AppError::ConfigError("No run logs found".into()))?;
    let eval_path = storage
        .find_latest_eval_file()?
        .ok_or_else(|| AppError::ConfigError("No eval logs found".into()))?;

    let eval_entries: Vec<EvalEntry> = storage.read_jsonl_eval_entries(&eval_path)?;

    generate_report(&storage, &run_path, &eval_path, eval_entries)
}
