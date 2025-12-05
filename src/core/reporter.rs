use crate::error::AppError;
use crate::model::EvalEntry;
use crate::storage::Storage;
use std::io::Write;
use std::path::Path;

pub fn generate_report(
    storage: &impl Storage,
    run_path: &Path,
    eval_path: &Path,
    eval_entries: Vec<EvalEntry>,
) -> Result<std::path::PathBuf, AppError> {
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

    let report_path = storage.next_report_path();
    let mut file = storage.create_file(&report_path)?;
    file.write_all(content.as_bytes())?;

    Ok(report_path)
}
