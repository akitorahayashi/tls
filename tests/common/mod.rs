//! Shared testing utilities for Telescope integration tests.

use assert_cmd::Command;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

pub struct TestContext {
    root: TempDir,
}

impl TestContext {
    pub fn new() -> Self {
        let root = TempDir::new().expect("Failed to create temp directory for tests");
        Self { root }
    }

    pub fn workspace(&self) -> &Path {
        self.root.path()
    }

    pub fn cli(&self) -> Command {
        let mut cmd = Command::cargo_bin("tls").expect("Failed to locate tls binary");
        cmd.current_dir(self.workspace());
        cmd
    }

    pub fn path(&self, relative: impl AsRef<Path>) -> PathBuf {
        self.workspace().join(relative)
    }

    pub fn read_to_string(&self, relative: impl AsRef<Path>) -> String {
        fs::read_to_string(self.path(relative)).expect("expected file to be readable")
    }

    pub fn assert_exists(&self, relative: impl AsRef<Path>) {
        let relative = relative.as_ref();
        assert!(self.path(relative).exists(), "{} should exist", relative.display());
    }

    pub fn write_block(&self, dir: &str, id: &str, input: &str, expected: Option<&str>) {
        let content = serde_json::json!({
            "metadata": {"id": id},
            "dataset": [
                {"input": input, "expected": expected}
            ]
        });

        let path = self.path(format!("{dir}/{id}.json"));
        fs::create_dir_all(path.parent().unwrap()).expect("dir creation failed");
        fs::write(&path, serde_json::to_string_pretty(&content).unwrap())
            .expect("failed to write block file");
    }

    pub fn latest_file(&self, dir: &str) -> PathBuf {
        let mut entries: Vec<PathBuf> = fs::read_dir(self.path(dir))
            .expect("dir should exist")
            .filter_map(|e| e.ok().map(|e| e.path()))
            .collect();
        entries.sort();
        entries.pop().expect("latest file should exist")
    }
}
