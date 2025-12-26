"""Benchmark evaluation block models."""

from pydantic import BaseModel, Field

# Type alias for grading criteria
GradingCriteria = list[str]


class BlockGrading(BaseModel):
    """Grading settings applied to the entire block."""

    template: str = Field(
        ...,
        description="Prompt template for grading (Jinja2 format). "
        "Variables available: {{input}}, {{output}}, {{expected}}, {{criteria}}",
    )
    model: str | None = Field(
        default=None,
        description="LLM model to use for grading (if not specified, use default)",
    )


class BlockMetadata(BaseModel):
    """Metadata for an evaluation block."""

    id: str = Field(..., description="Unique identifier for the block")
    description: str | None = Field(
        default=None, description="Optional description of the block"
    )
    active: bool = Field(default=True, description="Control execution status")


class BlockPrompts(BaseModel):
    """Prompts configuration for an evaluation block."""

    system: str = Field(..., description="System prompt for the block")


class TestCase(BaseModel):
    """A single test case within an evaluation block."""

    id: str | None = Field(default=None, description="Optional unique identifier")
    input: str = Field(..., description="Input prompt for the test case")
    expected: str | None = Field(
        default=None, description="Expected output for comparison"
    )
    context: str | None = Field(
        default=None, description="Additional context for the test case"
    )
    criteria: GradingCriteria | None = Field(
        default=None, description="Specific grading criteria for this test case"
    )


class EvaluationBlock(BaseModel):
    """Complete evaluation block containing metadata, prompts, and test cases."""

    metadata: BlockMetadata
    prompts: BlockPrompts
    grading: BlockGrading | None = Field(
        default=None, description="Grading settings (optional)"
    )
    dataset: list[TestCase]
