mod common;

use common::TestContext;
use predicates::prelude::*;

#[test]
fn init_scaffolds_layout_and_config() {
    let ctx = TestContext::new();

    ctx.cli()
        .arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("Initialized Telescope workspace"));

    for path in [
        "benchmarks",
        "metrics",
        "reports",
        ".telescope/runs",
        ".telescope/evals",
        "telescope.toml",
        ".gitignore",
    ] {
        ctx.assert_exists(path);
    }

    let config = ctx.read_to_string("telescope.toml");
    assert!(config.contains("[project]"), "config should contain project section");
    assert!(config.contains("[target]"), "config should contain target section");

    let gitignore = ctx.read_to_string(".gitignore");
    assert!(gitignore.lines().any(|line| line.trim() == ".telescope/"));
    assert!(gitignore.lines().any(|line| line.trim() == ".env"));
}

#[test]
fn init_is_idempotent_and_preserves_gitignore() {
    let ctx = TestContext::new();
    let gitignore_path = ctx.path(".gitignore");
    std::fs::write(&gitignore_path, "node_modules/\n.env\n")
        .expect("failed to seed .gitignore for test");

    ctx.cli().arg("init").assert().success();
    ctx.cli().arg("init").assert().success();

    let gitignore = ctx.read_to_string(".gitignore");
    let env_count = gitignore.lines().filter(|line| line.trim() == ".env").count();
    let telescope_count = gitignore.lines().filter(|line| line.trim() == ".telescope/").count();

    assert_eq!(env_count, 1, "`.env` should not be duplicated");
    assert_eq!(telescope_count, 1, "`.telescope/` should be added once");
    assert!(
        gitignore.lines().any(|line| line.trim() == "node_modules/"),
        "Custom gitignore entries should be preserved"
    );
}
