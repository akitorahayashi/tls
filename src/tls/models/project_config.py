"""Project configuration models for telescope.ini."""

import re
from pathlib import Path

from pydantic import BaseModel, Field


def sanitize_model_name(model: str) -> str:
    """Sanitize a model name for use in filesystem paths."""
    return re.sub(r"[:/\\ ]", "-", model)


class ProjectConfig(BaseModel):
    """Project-level configuration from [project] section."""

    name: str = Field(..., description="Project name")
    description: str | None = Field(default=None, description="Project description")
    blocks_dir: Path = Field(
        default=Path("./benchmarks"),
        description="Directory containing benchmark JSON files",
    )


class TargetConfig(BaseModel):
    """Target LLM configuration from [target] section."""

    models: list[str] = Field(
        ..., description="Models to evaluate (whitespace trimmed)"
    )
    endpoint: str = Field(
        default="http://127.0.0.1:11434", description="API endpoint URL"
    )
    timeout: int = Field(default=300, description="Request timeout in seconds")
    api_key: str | None = Field(
        default=None, description="Optional API key for authenticated endpoints"
    )


class Config(BaseModel):
    """Complete telescope.ini configuration."""

    project: ProjectConfig
    target: TargetConfig
