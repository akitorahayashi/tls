use crate::error::AppError;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

const DEFAULT_CONFIG: &str = r#"[project]
name = "my-telescope-project"
description = "Describe your evaluation focus here"

[target]
base_url = "https://api.openai.com/v1"
model = "gpt-4o-mini"
"#;

const GITIGNORE_ENTRIES: [&str; 2] = [".telescope/", ".env"];

pub struct ProjectLayout<'a> {
    root: &'a Path,
}

pub struct InitReport {
    pub created_paths: Vec<PathBuf>,
    pub gitignore_updated: bool,
}

impl<'a> ProjectLayout<'a> {
    pub fn new(root: &'a Path) -> Self {
        Self { root }
    }

    pub fn init(&self) -> Result<InitReport, AppError> {
        let mut created_paths = Vec::new();

        self.ensure_dir("benchmarks", &mut created_paths)?;
        self.ensure_dir("metrics", &mut created_paths)?;
        self.ensure_dir("reports", &mut created_paths)?;
        self.ensure_dir(Path::new(".telescope").join("runs"), &mut created_paths)?;
        self.ensure_dir(Path::new(".telescope").join("evals"), &mut created_paths)?;

        self.ensure_config(&mut created_paths)?;
        let gitignore_updated = self.ensure_gitignore(&mut created_paths)?;

        Ok(InitReport { created_paths, gitignore_updated })
    }

    fn ensure_dir<P: AsRef<Path>>(
        &self,
        relative: P,
        created: &mut Vec<PathBuf>,
    ) -> Result<(), AppError> {
        let path = self.root.join(relative);
        if !path.exists() {
            fs::create_dir_all(&path)?;
            created.push(path);
        }
        Ok(())
    }

    fn ensure_config(&self, created: &mut Vec<PathBuf>) -> Result<(), AppError> {
        let config_path = self.root.join("telescope.toml");
        if !config_path.exists() {
            let mut file = fs::File::create(&config_path)?;
            file.write_all(DEFAULT_CONFIG.as_bytes())?;
            created.push(config_path);
        }
        Ok(())
    }

    fn ensure_gitignore(&self, created: &mut Vec<PathBuf>) -> Result<bool, AppError> {
        let path = self.root.join(".gitignore");
        let existed = path.exists();
        let mut lines = if existed {
            fs::read_to_string(&path)?
                .lines()
                .map(|line| line.trim().to_string())
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };

        let mut updated = false;
        for entry in GITIGNORE_ENTRIES {
            if !lines.iter().any(|line| line == entry) {
                lines.push(entry.to_string());
                updated = true;
            }
        }

        if updated || !existed {
            let mut file = fs::File::create(&path)?;
            let content = lines.into_iter().map(|line| format!("{line}\n")).collect::<String>();
            file.write_all(content.as_bytes())?;
            if !existed {
                created.push(path);
            }
        }

        Ok(updated)
    }
}
