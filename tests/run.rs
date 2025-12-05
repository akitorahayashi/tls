mod common;

use common::TestContext;
use predicates::str::contains;
use serde_json::Value;
use std::env;
use std::sync::LazyLock;
use tokio::sync::Mutex;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

static ENV_MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

#[tokio::test]
async fn run_executes_benchmarks_only_by_default() {
    let _lock = ENV_MUTEX.lock().await;
    let mock_server = MockServer::start().await;
    env::set_var("OPENAI_API_COMPATIBLE_LLM_ENDPOINT", format!("{}/", mock_server.uri()));
    env::set_var("OPENAI_API_KEY", "test-key");

    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "choices": [{
                "message": { "role": "assistant", "content": "echo: hello" }
            }]
        })))
        .mount(&mock_server)
        .await;

    let ctx = TestContext::new();
    ctx.cli().arg("init").assert().success();

    // Remove the example block to avoid polluting the test
    std::fs::remove_file(ctx.path("benchmarks/example.json")).ok();

    ctx.write_block("benchmarks", "greeting", "hello", Some("echo: hello"));
    ctx.write_block("metrics", "farewell", "bye", Some("echo: bye"));

    ctx.cli().arg("run").assert().success();

    let run_path = ctx.latest_file(".telescope/runs");
    let contents = std::fs::read_to_string(run_path).unwrap();
    let entries: Vec<Value> =
        contents.lines().map(|line| serde_json::from_str(line).unwrap()).collect();

    assert!(entries.iter().all(|e| e["block_id"] == "greeting"));
}

#[tokio::test]
async fn run_can_include_metrics_and_filter_by_id() {
    let _lock = ENV_MUTEX.lock().await;
    let mock_server = MockServer::start().await;
    env::set_var("OPENAI_API_COMPATIBLE_LLM_ENDPOINT", format!("{}/", mock_server.uri()));
    env::set_var("OPENAI_API_KEY", "test-key");

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "choices": [{
                "message": { "role": "assistant", "content": "echo: hello" }
            }]
        })))
        .mount(&mock_server)
        .await;

    let ctx = TestContext::new();
    ctx.cli().arg("init").assert().success();
    std::fs::remove_file(ctx.path("benchmarks/example.json")).ok();

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
