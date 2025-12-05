use crate::error::AppError;
use crate::model::EvalEntry;
use crate::storage::ProjectLayout;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub fn generate_report(layout: &ProjectLayout<'_>) -> Result<PathBuf, AppError> {
    let run_path = layout.latest_run_file()?
        .ok_or_else(|| AppError::ConfigError("No run logs found".into()))?;
    let eval_path = layout.latest_eval_file()?
        .ok_or_else(|| AppError::ConfigError("No eval logs found".into()))?;

    let eval_entries = layout.read_jsonl::<EvalEntry>(&eval_path)?;

    let total = eval_entries.len();
    let passed = eval_entries.iter().filter(|e| e.passed).count();
    let failed_cases: Vec<&EvalEntry> = eval_entries.iter().filter(|e| !e.passed).collect();

    let mut content = String::new();
    content.push_str("# Telescope Report\n\n");
    content.push_str(&format!("Run log: {}\n", run_path.display()));
    content.push_str(&format!("Eval log: {}\n\n", eval_path.display()));
    content.push_str(&format!("Total cases: {total}\n"));
    content.push_str(&format!("Passed: {passed}\n"));
    content.push_str(&format!("Failed: {}\n\n", total - passed));

    if !failed_cases.is_empty() {
        content.push_str("## Failures\n");
        for fail in failed_cases {
            content.push_str(&format!(
                "- {} case {}: expected {:?}, got {}\n",
                fail.block_id, fail.case_index, fail.expected, fail.output
            ));
        }
    }

    let report_path = layout.next_report_path();
    let mut file = File::create(&report_path)?;
    file.write_all(content.as_bytes())?;

    Ok(report_path)
}
