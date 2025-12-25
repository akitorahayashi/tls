"""Application-level settings for the tls CLI tool."""

import configparser
from pathlib import Path

from pydantic import Field
from pydantic_settings import BaseSettings

from tls.core.exceptions import ConfigError
from tls.models.project_config import Config, ProjectConfig, TargetConfig


class AppSettings(BaseSettings):
    """Settings exposed to the dependency container."""

    app_name: str = Field(
        default="tls",
        alias="TLS_APP_NAME",
        description="Public-facing application name reported in outputs and logs.",
    )
    use_mock_llm: bool = Field(
        default=False,
        alias="TLS_USE_MOCK_LLM",
        description="Toggle to inject the mock LLM client for local dev and tests.",
    )


def load_config(project_root: Path) -> Config:
    """
    Load configuration from telescope.ini in the given directory.

    Args:
        project_root: Root directory of the project.

    Returns:
        Parsed Config object.

    Raises:
        ConfigError: If config file is missing or malformed.
    """
    config_path = project_root / "telescope.ini"
    if not config_path.exists():
        raise ConfigError("telescope.ini not found. Run 'tls init' first.")

    parser = configparser.ConfigParser()
    parser.read(config_path)

    if "project" not in parser:
        raise ConfigError("Missing [project] section in telescope.ini")
    if "target" not in parser:
        raise ConfigError("Missing [target] section in telescope.ini")

    project_section = parser["project"]
    target_section = parser["target"]

    # Parse models - comma-separated list with whitespace trimming
    models_str = target_section.get("models", "")
    models = [m.strip() for m in models_str.split(",") if m.strip()]
    if not models:
        raise ConfigError("No models specified in [target] section")

    # Build config objects
    project_config = ProjectConfig(
        name=project_section.get("name", "unnamed"),
        description=project_section.get("description"),
        blocks_dir=Path(project_section.get("blocks_dir", "./benchmarks")),
    )

    target_config = TargetConfig(
        models=models,
        endpoint=target_section.get("endpoint", "http://127.0.0.1:11434"),
        timeout=int(target_section.get("timeout", "300")),
        api_key=target_section.get("api_key"),
    )

    return Config(project=project_config, target=target_config)


settings = AppSettings()
