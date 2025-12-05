use crate::error::AppError;
use crate::model::EvaluationBlock;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

const DEFAULT_CONFIG: &str = r#"[project]
name = "my-telescope-project"
description = "Describe your evaluation focus here"

[target]
base_url = "https://api.openai.com/v1"
model = "gpt-4o-mini"
"#;

const EXAMPLE_BENCHMARK: &str = r#"{
  "metadata": {
    "id": "example-block",
    "description": "An example benchmark block",
    "version": "1.0",
    "model": "gpt-4o-mini"
  },
  "prompts": {
    "system": "You are a helpful assistant."
  },
  "dataset": [
    {
      "input": "Hello, how are you?",
      "expected": "I am fine, thank you."
    }
  ]
}"#;

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
        self.ensure_example_block(&mut created_paths)?;
        let gitignore_updated = self.ensure_gitignore(&mut created_paths)?;

        Ok(InitReport {
            created_paths,
            gitignore_updated,
        })
    }

    pub fn runs_dir(&self) -> PathBuf {
        self.root.join(".telescope/runs")
    }

    pub fn evals_dir(&self) -> PathBuf {
        self.root.join(".telescope/evals")
    }

    pub fn reports_dir(&self) -> PathBuf {
        self.root.join("reports")
    }

    pub fn benchmarks_dir(&self) -> PathBuf {
        self.root.join("benchmarks")
    }

    pub fn metrics_dir(&self) -> PathBuf {
        self.root.join("metrics")
    }

    pub fn next_run_path(&self) -> PathBuf {
        let timestamp = chrono::Utc::now().format("%Y%m%d%H%M%S%.3f");
        self.runs_dir().join(format!("run_{timestamp}.jsonl"))
    }

    pub fn eval_path_for(&self, run_path: &Path) -> PathBuf {
        let file_name = run_path
            .file_stem()
            .and_then(|name| name.to_str())
            .map(|name| format!("{name}_eval.jsonl"))
            .unwrap_or_else(|| "eval.jsonl".to_string());
        self.evals_dir().join(file_name)
    }

    pub fn next_report_path(&self) -> PathBuf {
        let timestamp = chrono::Utc::now().format("%Y%m%d%H%M%S%.3f");
        self.reports_dir().join(format!("{timestamp}_report.md"))
    }

    pub fn load_benchmarks(&self) -> Result<Vec<EvaluationBlock>, AppError> {
        self.load_blocks(&self.benchmarks_dir())
    }

    pub fn load_metrics(&self) -> Result<Vec<EvaluationBlock>, AppError> {
        self.load_blocks(&self.metrics_dir())
    }

    fn load_blocks(&self, dir: &Path) -> Result<Vec<EvaluationBlock>, AppError> {
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

    pub fn latest_run_file(&self) -> Result<Option<PathBuf>, AppError> {
        self.latest_file(&self.runs_dir())
    }

    pub fn latest_eval_file(&self) -> Result<Option<PathBuf>, AppError> {
        self.latest_file(&self.evals_dir())
    }

    fn latest_file(&self, dir: &Path) -> Result<Option<PathBuf>, AppError> {
        let mut entries: Vec<PathBuf> =
            fs::read_dir(dir)?.filter_map(|e| e.ok().map(|e| e.path())).collect();
        entries.sort();
        Ok(entries.pop())
    }

    pub fn read_jsonl<T: for<'de> serde::Deserialize<'de>>(&self, path: &Path) -> Result<Vec<T>, AppError> {
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

    fn ensure_example_block(&self, created: &mut Vec<PathBuf>) -> Result<(), AppError> {
        let block_path = self.root.join("benchmarks/example.json");
        if !block_path.exists() {
            let mut file = fs::File::create(&block_path)?;
            file.write_all(EXAMPLE_BENCHMARK.as_bytes())?;
            created.push(block_path);
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
