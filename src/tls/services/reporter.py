"""Reporter service for writing benchmark run results."""

from datetime import datetime, timezone
from pathlib import Path

import aiofiles
import aiofiles.os

from tls.errors import TlsError
from tls.models.project_config import sanitize_model_name
from tls.models.report import RunEntry


def sanitize_block_id(block_id: str) -> str:
    """Sanitize block ID for use as filename."""
    return block_id.replace("/", "-").replace("\\", "-").replace(" ", "-")


def format_entry(entry: RunEntry) -> str:
    """Format a run entry as Markdown."""
    lines = [
        f"## Block: {entry.block_id} (Case {entry.case_index})",
        f"- **Input**: {entry.input}",
        f"- **Output**: {entry.output}",
    ]
    if entry.expected:
        lines.append(f"- **Expected**: {entry.expected}")
    if entry.context:
        lines.append(f"- **Context**: {entry.context}")
    lines.append("---\n")
    return "\n".join(lines)


class FileSystemReporter:
    """File system-based report writer that creates Markdown files."""

    def __init__(self, reports_dir: Path) -> None:
        """
        Initialize the reporter.

        Args:
            reports_dir: Base directory for reports.
        """
        self.reports_dir = reports_dir

    async def init_run(
        self,
        category: str | None,
        model: str,
        block_ids: list[str],
    ) -> Path:
        """Initialize a new run directory."""
        now = datetime.now(timezone.utc)
        timestamp = now.strftime("%Y%m%d%H%M%S.%f")[:-3]
        sanitized_model = sanitize_model_name(model)

        if category:
            run_dir = self.reports_dir / category / sanitized_model / timestamp
        else:
            run_dir = self.reports_dir / sanitized_model / timestamp

        await aiofiles.os.makedirs(run_dir, exist_ok=True)

        timestamp_str = now.isoformat()

        # Create report files with headers for all blocks
        for block_id in block_ids:
            filename = sanitize_block_id(block_id)
            file_path = run_dir / f"{filename}.md"

            header = (
                f"# Telescope Run Report\n"
                f"**Model**: {model}\n"
                f"**Date**: {timestamp_str}\n\n"
            )
            async with aiofiles.open(file_path, "w") as f:
                await f.write(header)

        return run_dir

    async def write_entry(self, run_dir: Path, entry: RunEntry) -> None:
        """Write a single entry to a block's report."""
        filename = sanitize_block_id(entry.block_id)
        file_path = run_dir / f"{filename}.md"

        if not file_path.exists():
            raise TlsError(f"Report file not found: {file_path}")

        entry_content = format_entry(entry)

        async with aiofiles.open(file_path, "a") as f:
            await f.write(entry_content)
