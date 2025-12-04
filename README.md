# Telescope CLI (`tls`)

Telescope is a local-first CLI for managing LLM evaluation projects. It scaffolds a
standard layout for evaluation blocks, metrics, execution logs, and human-friendly
reports so teams can treat "evaluation as code" from day one.

## What `tls init` creates

Running `tls init` in a project root lays down the expected folders and config files:

```text
./telescope.toml         # project-wide settings (model, endpoints, etc.)
./benchmarks/            # baseline evaluation blocks
./metrics/               # optional deep-dive metrics
./reports/               # human-readable summaries (kept in git)
./.telescope/runs/       # raw run logs (JSONL, typically gitignored)
./.telescope/evals/      # judge/eval logs (JSONL, typically gitignored)
```

It also ensures `.gitignore` contains entries for `.telescope/` and `.env` while
preserving any existing content.

## Usage

```bash
# build and run from source
cargo run -- init

# or install a development copy
cargo install --path .
# then run anywhere
 tls init
```

Re-running `tls init` is safe: missing folders/files are created, `.gitignore` is
patched only when needed, and existing content is left intact.

## Development

- `cargo fmt` — format the codebase
- `cargo test` — run the integration test suite

The project intentionally keeps the `init` workflow small and well-tested so future
commands (`run`, `eval`, `report`, `clean`) can be layered on without breaking the
workspace conventions.
