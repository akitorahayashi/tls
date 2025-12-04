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
}
