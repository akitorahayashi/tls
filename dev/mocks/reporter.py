"""In-memory reporter for testing."""

from pathlib import Path

from tls.models.report import RunEntry


class InMemoryReporter:
    """In-memory report writer for testing."""

    def __init__(self) -> None:
        """Initialize the in-memory reporter."""
        self.entries: dict[str, list[RunEntry]] = {}
        self.run_dir: Path | None = None

    async def init_run(
        self,
        category: str | None,
        model: str,
        block_ids: list[str],
    ) -> Path:
        """Initialize a mock run."""
        self.entries = {block_id: [] for block_id in block_ids}
        self.run_dir = Path("/mock/run/dir")
        return self.run_dir

    async def write_entry(self, run_dir: Path, entry: RunEntry) -> None:
        """Store entry in memory."""
        if entry.block_id in self.entries:
            self.entries[entry.block_id].append(entry)
