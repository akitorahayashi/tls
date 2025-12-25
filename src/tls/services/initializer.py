"""Project initialization service."""

import subprocess
from pathlib import Path

from rich.console import Console

from tls.config.templates import (
    BENCHMARK_REASONING,
    BENCHMARK_STRUCTURED,
    DEFAULT_CONFIG,
    GITIGNORE_ENTRIES,
)


class InitReport:
    """Report of initialization actions taken."""

    def __init__(self) -> None:
        self.created_paths: list[Path] = []
        self.gitignore_updated: bool = False
        self.git_initialized: bool = False


class Initializer:
    """Service for initializing new tls projects."""

    def __init__(self, console: Console | None = None) -> None:
        """
        Initialize the service.

        Args:
            console: Optional Rich console for output.
        """
        self.console = console or Console()

    def execute(self, root: Path) -> InitReport:
        """
        Initialize a new tls project at the given path.

        Args:
            root: Root directory for the project.

        Returns:
            Report of actions taken.
        """
        report = InitReport()

        # Create root directory if needed
        if not root.exists():
            root.mkdir(parents=True)
            report.created_paths.append(root)

        # Create directories
        self._ensure_dir(root / "benchmarks", report)
        self._ensure_dir(root / "reports", report)

        # Create config file
        self._ensure_config(root, report)

        # Create benchmark files
        self._ensure_benchmark_files(root, report)

        # Update .gitignore
        report.gitignore_updated = self._ensure_gitignore(root, report)

        # Initialize git repo
        report.git_initialized = self._init_git_repo(root)

        return report

    def _ensure_dir(self, path: Path, report: InitReport) -> None:
        """Create directory if it doesn't exist."""
        if not path.exists():
            path.mkdir(parents=True)
            report.created_paths.append(path)

    def _ensure_config(self, root: Path, report: InitReport) -> None:
        """Create telescope.ini if it doesn't exist."""
        config_path = root / "telescope.ini"
        if not config_path.exists():
            config_path.write_text(DEFAULT_CONFIG)
            report.created_paths.append(config_path)

    def _ensure_benchmark_files(self, root: Path, report: InitReport) -> None:
        """Create example benchmark files if they don't exist."""
        benchmarks_dir = root / "benchmarks"

        structured_path = benchmarks_dir / "structured_output.json"
        if not structured_path.exists():
            structured_path.write_text(BENCHMARK_STRUCTURED)
            report.created_paths.append(structured_path)

        reasoning_path = benchmarks_dir / "reasoning.json"
        if not reasoning_path.exists():
            reasoning_path.write_text(BENCHMARK_REASONING)
            report.created_paths.append(reasoning_path)

    def _ensure_gitignore(self, root: Path, report: InitReport) -> bool:
        """Create or update .gitignore with required entries."""
        gitignore_path = root / ".gitignore"
        existed = gitignore_path.exists()

        if existed:
            lines = gitignore_path.read_text().splitlines()
        else:
            lines = []

        updated = False
        for entry in GITIGNORE_ENTRIES:
            if entry not in lines:
                lines.append(entry)
                updated = True

        if updated or not existed:
            gitignore_path.write_text("\n".join(lines) + "\n")
            if not existed:
                report.created_paths.append(gitignore_path)

        return updated

    def _init_git_repo(self, root: Path) -> bool:
        """Initialize git repository if not already present."""
        git_dir = root / ".git"
        if git_dir.exists():
            return False

        try:
            result = subprocess.run(
                ["git", "init"],
                cwd=root,
                capture_output=True,
                text=True,
                check=False,
            )
            return result.returncode == 0
        except FileNotFoundError:
            # git command not found
            return False
