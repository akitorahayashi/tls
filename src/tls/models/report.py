"""Report models for benchmark run results."""

from datetime import datetime

from pydantic import BaseModel, Field

from tls.models.benchmark import GradingCriteria


class RunEntry(BaseModel):
    """Entry representing a single test case execution result."""

    block_id: str = Field(..., description="ID of the evaluation block")
    case_index: int = Field(..., description="Index of the test case within the block")
    input: str = Field(..., description="Input prompt sent to the model")
    output: str = Field(..., description="Model's response output")
    model: str = Field(..., description="Model used for this evaluation")
    expected: str | None = Field(default=None, description="Expected output")
    context: str | None = Field(default=None, description="Additional context used")
    criteria: GradingCriteria | None = Field(
        default=None, description="Grading criteria for this case"
    )
    grading_template: str | None = Field(
        default=None, description="Grading prompt template for reproducibility"
    )
    timestamp: datetime = Field(
        default_factory=datetime.utcnow, description="Timestamp of the execution"
    )
