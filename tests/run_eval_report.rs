mod common;

use common::TestContext;
use predicates::str::contains;
use serde_json::Value;

#[test]
fn run_executes_benchmarks_only_by_default() {
    let ctx = TestContext::new();
    ctx.cli().arg("init").assert().success();

    ctx.write_block("benchmarks", "greeting", "hello", Some("echo: hello"));
    ctx.write_block("metrics", "farewell", "bye", Some("echo: bye"));

    ctx.cli().arg("run").assert().success();

    let run_path = ctx.latest_file(".telescope/runs");
    let contents = std::fs::read_to_string(run_path).unwrap();
    let entries: Vec<Value> =
        contents.lines().map(|line| serde_json::from_str(line).unwrap()).collect();

    assert!(entries.iter().all(|e| e["block_id"] == "greeting"));
}

#[test]
fn run_can_include_metrics_and_filter_by_id() {
    let ctx = TestContext::new();
    ctx.cli().arg("init").assert().success();

    ctx.write_block("benchmarks", "bench", "hello", Some("echo: hello"));
    ctx.write_block("metrics", "metric", "bye", Some("echo: bye"));

    ctx.cli().arg("run").arg("--with-metrics").assert().success();
    let run_path = ctx.latest_file(".telescope/runs");
    let contents = std::fs::read_to_string(run_path).unwrap();
    let ids: Vec<String> = contents
        .lines()
        .map(|line| {
            serde_json::from_str::<Value>(line).unwrap()["block_id"].as_str().unwrap().to_string()
        })
        .collect();
    assert!(ids.contains(&"bench".to_string()));
    assert!(ids.contains(&"metric".to_string()));

    ctx.cli().args(["run", "--id", "metric"]).assert().success().stdout(contains("Wrote run log"));
    let run_path = ctx.latest_file(".telescope/runs");
    let contents = std::fs::read_to_string(run_path).unwrap();
    let ids: Vec<String> = contents
        .lines()
        .map(|line| {
            serde_json::from_str::<Value>(line).unwrap()["block_id"].as_str().unwrap().to_string()
        })
        .collect();
    assert_eq!(ids, vec!["metric".to_string()]);
}

#[test]
fn eval_and_report_produce_outputs() {
    let ctx = TestContext::new();
    ctx.cli().arg("init").assert().success();
    ctx.write_block("benchmarks", "echo", "ping", Some("echo: ping"));

    ctx.cli().arg("run").assert().success();
    ctx.cli().arg("eval").assert().success();
    ctx.cli().arg("report").assert().success();

    let eval_path = ctx.latest_file(".telescope/evals");
    let eval_contents = std::fs::read_to_string(&eval_path).unwrap();
    let rows: Vec<Value> =
        eval_contents.lines().map(|line| serde_json::from_str(line).unwrap()).collect();

    assert!(rows.iter().all(|row| row["passed"].as_bool() == Some(true)));

    let report_path = ctx.latest_file("reports");
    let report = std::fs::read_to_string(report_path).unwrap();
    assert!(report.contains("Total cases: 1"));
    assert!(report.contains("Passed: 1"));
}
