use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EvaluationBlock {
    pub metadata: BlockMetadata,
    pub dataset: Vec<TestCase>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlockMetadata {
    pub id: String,
    pub description: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TestCase {
    pub input: String,
    pub expected: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RunEntry {
    pub block_id: String,
    pub case_index: usize,
    pub input: String,
    pub expected: Option<String>,
    pub output: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EvalEntry {
    pub block_id: String,
    pub case_index: usize,
    pub expected: Option<String>,
    pub output: String,
    pub passed: bool,
}
