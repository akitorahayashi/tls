use crate::error::AppError;
use crate::model::EvaluationBlock;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

const DEFAULT_CONFIG: &str = r#"[project]
name = "my-telescope-project"
description = "Describe your evaluation focus here"

[target]
model = "llama3.2:3b"
"#;

const EXAMPLE_BENCHMARK: &str = r#"{
  "metadata": {
    "id": "example-block",
    "description": "An example benchmark block",
    "version": "1.0",
    "model": "llama3.2:3b"
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

const ENV_EXAMPLE_CONTENT: &str = r#"# Telescope Environment Configuration
# 
# Endpoint for OpenAI-compatible LLM API (default: Ollama)
OPENAI_API_COMPATIBLE_LLM_ENDPOINT=http://127.0.0.1:11434

# Optional: API key for the LLM service (not required for local Ollama)
# OPENAI_API_KEY=your-api-key-here
"#;

const GITIGNORE_ENTRIES: [&str; 2] = [".telescope/", ".env"];

pub struct ProjectLayout<'a> {
    root: &'a Path,
}

pub struct InitReport {
    pub created_paths: Vec<PathBuf>,
    pub gitignore_updated: bool,
    pub git_initialized: bool,
}

impl<'a> ProjectLayout<'a> {
    pub fn new(root: &'a Path) -> Self {
        Self { root }
    }

    pub fn init(&self) -> Result<InitReport, AppError> {
        let mut created_paths = Vec::new();

        self.ensure_dir("benchmarks", &mut created_paths)?;
        self.ensure_dir("metrics", &mut created_paths)?;
        self.ensure_dir(Path::new(".telescope").join("runs"), &mut created_paths)?;

        self.ensure_config(&mut created_paths)?;
        self.ensure_example_block(&mut created_paths)?;
        let gitignore_updated = self.ensure_gitignore(&mut created_paths)?;
        self.ensure_env_files(&mut created_paths)?;
        let git_initialized = self.init_git_repo()?;

        Ok(InitReport { created_paths, gitignore_updated, git_initialized })
    }

    pub fn runs_dir(&self) -> PathBuf {
        self.root.join(".telescope/runs")
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

    fn latest_file(&self, dir: &Path) -> Result<Option<PathBuf>, AppError> {
        let mut entries: Vec<PathBuf> = fs::read_dir(dir)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()?;
        entries.sort();
        Ok(entries.pop())
    }

    pub fn read_jsonl<T: for<'de> serde::Deserialize<'de>>(
        &self,
        path: &Path,
    ) -> Result<Vec<T>, AppError> {
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

    fn ensure_env_files(&self, created: &mut Vec<PathBuf>) -> Result<(), AppError> {
        // Create .env.example with recommended settings
        let env_example_path = self.root.join(".env.example");
        if !env_example_path.exists() {
            let mut file = fs::File::create(&env_example_path)?;
            file.write_all(ENV_EXAMPLE_CONTENT.as_bytes())?;
            created.push(env_example_path);
        }

        // Create .env as a copy of .env.example (user can edit immediately)
        let env_path = self.root.join(".env");
        if !env_path.exists() {
            let mut file = fs::File::create(&env_path)?;
            file.write_all(ENV_EXAMPLE_CONTENT.as_bytes())?;
            created.push(env_path);
        }

        Ok(())
    }

    fn init_git_repo(&self) -> Result<bool, AppError> {
        let git_dir = self.root.join(".git");
        if git_dir.exists() {
            return Ok(false);
        }

        let output =
            Command::new("git").arg("init").current_dir(self.root).output().map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    AppError::ConfigError(
                        "git command not found. Please install Git to initialize a repository."
                            .to_string(),
                    )
                } else {
                    AppError::Io(e)
                }
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::ConfigError(format!(
                "Failed to initialize git repository: {}",
                stderr
            )));
        }

        Ok(true)
    }
}
