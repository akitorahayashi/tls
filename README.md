# Telescope CLI (`tls`)

Telescope is a local-first CLI for running LLM prompts against local endpoints like Ollama or MLX.
It scaffolds a standard layout for benchmark blocks and execution logs, enabling quick prompt testing
and iteration during development.

## What `tls init` creates

Running `tls init` in a project root lays down the expected folders and config files:

```text
./telescope.toml         # project-wide settings (model, etc.)
./benchmarks/            # evaluation blocks with prompts and test cases
./metrics/               # optional deep-dive metrics blocks
./.telescope/runs/       # raw run logs (JSONL, typically gitignored)
./.env                   # environment configuration (gitignored)
./.env.example           # template for environment configuration
```

It also:
- Initializes a Git repository if one doesn't exist
- Ensures `.gitignore` contains entries for `.telescope/` and `.env`

## Configuration

### Environment Variables

Telescope uses environment variables for LLM endpoint configuration:

| Variable | Description | Default |
|----------|-------------|---------|
| `OPENAI_API_COMPATIBLE_LLM_ENDPOINT` | Base URL for the LLM API | `http://127.0.0.1:11434` |
| `OPENAI_API_KEY` | API key (optional for local LLMs) | `dummy` |

The endpoint should be an OpenAI-compatible API base URL. Telescope automatically appends
`/v1/chat/completions` to the endpoint.

### Local LLM Setup (Ollama)

1. Install Ollama: https://ollama.ai
2. Pull a model: `ollama pull llama3.2:3b`
3. Start the server: `ollama serve` (runs on `http://127.0.0.1:11434` by default)
4. Run `tls init` and you're ready to go!

## Usage

```bash
# build and run from source
cargo run -- init

# or install a development copy
cargo install --path .
# then run anywhere
tls init
```

### Benchmark Blocks

Blocks live in `benchmarks/` (always run) and `metrics/` (opt-in). A block file
is JSON with minimal metadata and a dataset:

```json
{
  "metadata": { "id": "echo-smoke", "model": "llama3.2:3b" },
  "prompts": {
    "system": "You are an echo bot."
  },
  "dataset": [
    { "input": "ping", "expected": "echo: ping" }
  ]
}
```

### Running Prompts

Execute runs and write JSONL logs under `.telescope/runs/`:

```bash
tls run                  # benchmarks only
tls run --with-metrics   # benchmarks + metrics
tls run --id echo-smoke  # target a specific block id
```

### Idempotent Initialization

Re-running `tls init` is safe: missing folders/files are created, `.gitignore`
is patched only when needed, and existing content is left intact.

## Development

- `cargo fmt` — format the codebase
- `cargo clippy` — run lints
- `cargo test` — run the test suite

The project intentionally keeps the workflow focused on running prompts against local LLMs,
making it easy to iterate on prompt engineering without external API dependencies.
