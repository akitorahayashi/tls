"""Unit tests for tls models."""

from tls.models import (
    BlockMetadata,
    BlockPrompts,
    EvaluationBlock,
    TestCase,
    sanitize_model_name,
)


class TestSanitizeModelName:
    """Tests for model name sanitization."""

    def test_sanitize_colons(self):
        """Colons are replaced with dashes."""
        assert sanitize_model_name("qwen3:8b") == "qwen3-8b"

    def test_sanitize_slashes(self):
        """Slashes are replaced with dashes."""
        assert sanitize_model_name("mlx/model") == "mlx-model"

    def test_sanitize_spaces(self):
        """Spaces are replaced with dashes."""
        assert sanitize_model_name("my model") == "my-model"

    def test_sanitize_backslashes(self):
        """Backslashes are replaced with dashes."""
        assert sanitize_model_name("path\\model") == "path-model"

    def test_sanitize_mixed(self):
        """Multiple special characters are all replaced."""
        assert sanitize_model_name("mlx:8b/v1 test") == "mlx-8b-v1-test"


class TestEvaluationBlock:
    """Tests for EvaluationBlock model."""

    def test_create_minimal_block(self):
        """Create a block with minimal required fields."""
        block = EvaluationBlock(
            metadata=BlockMetadata(id="test-block"),
            prompts=BlockPrompts(system="You are a test assistant."),
            dataset=[TestCase(input="Hello")],
        )
        assert block.metadata.id == "test-block"
        assert block.metadata.active is True
        assert len(block.dataset) == 1

    def test_block_with_grading(self):
        """Create a block with grading configuration."""
        from tls.models import BlockGrading

        block = EvaluationBlock(
            metadata=BlockMetadata(id="graded-block"),
            prompts=BlockPrompts(system="System prompt"),
            grading=BlockGrading(template="Grade: {{output}}"),
            dataset=[TestCase(input="Test", expected="Expected")],
        )
        assert block.grading is not None
        assert block.grading.template == "Grade: {{output}}"

    def test_test_case_optional_fields(self):
        """Test case optional fields default to None."""
        case = TestCase(input="Hello")
        assert case.id is None
        assert case.expected is None
        assert case.context is None
        assert case.criteria is None
