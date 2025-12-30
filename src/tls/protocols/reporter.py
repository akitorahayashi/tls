"""Protocol for report writer implementations."""

from pathlib import Path
from typing import Protocol

from tls.models.report import RunEntry


class ReporterProtocol(Protocol):
    """Protocol for report writer implementations."""

    async def init_run(
        self,
        category: str | None,
        model: str,
        block_ids: list[str],
    ) -> Path:
        """
        Initialize a new run directory and prepare files for all blocks.

        Args:
            category: Optional category (e.g., "benchmarks").
            model: Model name being tested.
            block_ids: List of block IDs to create report files for.

        Returns:
            Path to the created run directory.
        """
        ...

    async def write_entry(self, run_dir: Path, entry: RunEntry) -> None:
        """
        Write a single entry to a block's report.

        Args:
            run_dir: The run directory path.
            entry: The test case entry to write.
        """
        ...
