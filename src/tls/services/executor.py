"""Benchmark execution service."""

import json
from dataclasses import dataclass, field
from datetime import datetime, timezone
from pathlib import Path

from rich.console import Console
from rich.progress import (
    BarColumn,
    Progress,
    SpinnerColumn,
    TaskProgressColumn,
    TextColumn,
    TimeElapsedColumn,
)

from tls.errors import ConfigError
from tls.models.benchmark import EvaluationBlock
from tls.models.report import RunEntry
from tls.protocols.llm import LlmClientProtocol, Message
from tls.protocols.reporter import ReporterProtocol


@dataclass
class BlockSummary:
    """Summary of a single block execution."""

    block_id: str
    total_cases: int
    completed_cases: int = 0
    failed_cases: int = 0


@dataclass
class ModelSummary:
    """Summary of execution for a single model."""

    model: str
    blocks: list[BlockSummary] = field(default_factory=list)
    run_dir: Path | None = None


@dataclass
class RunSummary:
    """Overall summary of a benchmark run."""

    start_time: datetime
    end_time: datetime
    models: list[ModelSummary] = field(default_factory=list)
    total_cases: int = 0
    successful_cases: int = 0
    failed_cases: int = 0

    @property
    def duration_seconds(self) -> float:
        """Calculate run duration in seconds."""
        return (self.end_time - self.start_time).total_seconds()


class Executor:
    """Service for running benchmark evaluations."""

    def __init__(
        self,
        client: LlmClientProtocol,
        reporter: ReporterProtocol,
        console: Console | None = None,
    ) -> None:
        """
        Initialize the executor.

        Args:
            client: LLM client for API calls.
            reporter: Report writer for results.
            console: Optional Rich console for output.
        """
        self.client = client
        self.reporter = reporter
        self.console = console or Console()

    def load_blocks(self, path: Path) -> list[EvaluationBlock]:
        """
        Load evaluation blocks from a file or directory.

        Args:
            path: Path to a JSON file or directory containing JSON files.

        Returns:
            List of loaded evaluation blocks.
        """
        blocks = []

        if path.is_file():
            blocks.extend(self._load_block_file(path))
        elif path.is_dir():
            for file_path in sorted(path.glob("*.json")):
                blocks.extend(self._load_block_file(file_path))

        return blocks

    def _load_block_file(self, path: Path) -> list[EvaluationBlock]:
        """Load a single block file."""
        try:
            content = path.read_text()
            data = json.loads(content)
            block = EvaluationBlock.model_validate(data)
            return [block]
        except json.JSONDecodeError as e:
            self.console.print(f"[yellow]Warning: Failed to parse {path}: {e}[/yellow]")
            return []
        except Exception as e:
            self.console.print(f"[yellow]Warning: Failed to load {path}: {e}[/yellow]")
            return []

    async def execute(
        self,
        blocks_dir: Path,
        models: list[str],
        target_file: Path | None = None,
        target_id: str | None = None,
    ) -> RunSummary:
        """
        Execute benchmark evaluations.

        Args:
            blocks_dir: Directory containing benchmark files.
            models: List of model names to evaluate.
            target_file: Optional specific file to run.
            target_id: Optional specific test case ID to run.

        Returns:
            Summary of the run.
        """
        start_time = datetime.now(timezone.utc)

        # Load blocks
        if target_file:
            blocks = self.load_blocks(target_file)
            category = (
                target_file.parent.name if target_file.is_file() else target_file.name
            )
        else:
            blocks = self.load_blocks(blocks_dir)
            category = "benchmarks"

        # Filter inactive blocks (unless targeting specific file)
        if not target_file:
            blocks = [b for b in blocks if b.metadata.active]

        # Filter by ID if specified
        if target_id:
            blocks = self._filter_by_id(blocks, target_id)

        if not blocks:
            raise ConfigError("No evaluation blocks found")

        block_ids = [b.metadata.id for b in blocks]

        # Calculate total cases
        total_cases_per_model = sum(len(b.dataset) for b in blocks)
        total_cases = total_cases_per_model * len(models)

        model_summaries = []
        successful_cases = 0
        failed_cases = 0

        with Progress(
            SpinnerColumn(),
            TextColumn("[progress.description]{task.description}"),
            BarColumn(style="cyan"),
            TaskProgressColumn(),
            TimeElapsedColumn(),
            console=self.console,
        ) as progress:
            overall_task = progress.add_task(
                "[cyan]Running benchmarks...", total=total_cases
            )

            for model in models:
                run_dir = await self.reporter.init_run(category, model, block_ids)
                model_summary = ModelSummary(model=model, run_dir=run_dir)

                for block in blocks:
                    block_summary = BlockSummary(
                        block_id=block.metadata.id,
                        total_cases=len(block.dataset),
                    )

                    for idx, case in enumerate(block.dataset):
                        # Build messages
                        system_prompt = block.prompts.system
                        if case.context:
                            system_prompt += f"\n\nContext:\n{case.context}"

                        messages = [
                            Message(role="system", content=system_prompt),
                            Message(role="user", content=case.input),
                        ]

                        # Call LLM
                        try:
                            output = await self.client.chat(model, messages)
                            is_error = False
                        except Exception as e:
                            output = f"Error: {e}"
                            is_error = True

                        # Update counters
                        if is_error:
                            block_summary.failed_cases += 1
                            failed_cases += 1
                        else:
                            block_summary.completed_cases += 1
                            successful_cases += 1

                        # Write entry
                        entry = RunEntry(
                            block_id=block.metadata.id,
                            case_index=idx,
                            input=case.input,
                            output=output,
                            model=model,
                            expected=case.expected,
                            context=case.context,
                            criteria=case.criteria,
                            grading_template=block.grading.template
                            if block.grading
                            else None,
                        )
                        await self.reporter.write_entry(run_dir, entry)

                        progress.update(overall_task, advance=1)

                    model_summary.blocks.append(block_summary)
                model_summaries.append(model_summary)

        end_time = datetime.now(timezone.utc)

        return RunSummary(
            start_time=start_time,
            end_time=end_time,
            models=model_summaries,
            total_cases=total_cases,
            successful_cases=successful_cases,
            failed_cases=failed_cases,
        )

    def _filter_by_id(
        self, blocks: list[EvaluationBlock], target_id: str
    ) -> list[EvaluationBlock]:
        """Filter blocks to only include cases with the target ID."""
        filtered_blocks = []
        total_matches = 0

        for block in blocks:
            matching_cases = [c for c in block.dataset if c.id == target_id]
            if matching_cases:
                total_matches += len(matching_cases)
                # Create a copy with only matching cases
                filtered_block = EvaluationBlock(
                    metadata=block.metadata,
                    prompts=block.prompts,
                    grading=block.grading,
                    dataset=matching_cases,
                )
                filtered_blocks.append(filtered_block)

        if total_matches == 0:
            raise ConfigError(f"No test case found with ID: {target_id}")
        if total_matches > 1:
            raise ConfigError(
                f"Multiple test cases found with ID: {target_id}. IDs must be unique."
            )

        return filtered_blocks
