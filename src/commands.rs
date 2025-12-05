use crate::core::runner;
use crate::error::AppError;
use crate::gateway::Client;
use crate::storage::{InitReport, ProjectLayout};
use std::path::{Path, PathBuf};

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
    let client = Client::new()?;

    // We pass the client directly. runner::run_blocks takes &impl GenAiClient.
    runner::run_blocks(&layout, &client, with_metrics, id).await
}
