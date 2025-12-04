use crate::error::AppError;
use crate::scaffold::{InitReport, ProjectLayout};
use std::path::Path;

pub fn init(project_root: &Path) -> Result<InitReport, AppError> {
    let layout = ProjectLayout::new(project_root);
    layout.init()
}
