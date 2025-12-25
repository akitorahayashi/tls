"""Default templates and constants for project initialization."""

DEFAULT_CONFIG = """\
[project]
name = my-telescope-project
description = Describe your evaluation focus here
# Directory containing benchmark JSON files
blocks_dir = ./benchmarks

[target]
# Models to evaluate
# models = llama3.2:3b, qwen3:8b
models = qwen3-vl:8b-instruct-q4_K_M
endpoint = http://127.0.0.1:11434

# Request timeout in seconds
timeout = 300

# Optional: API key for authenticated endpoints (not required for local LLMs)
# api_key = your-api-key-here

# Available Models (Reference):
# deepseek-r1:8b-0528-qwen3-q4_K_M
# deepseek-r1:8b-0528-qwen3-q8_0
# llama3.2:3b-instruct-q8_0
# llama3.2:3b-text-q8_0
# qwen3-vl:8b-instruct-q4_K_M
# qwen3-vl:8b-instruct-q8_0
# qwen3-vl:8b-thinking-q8_0
# qwen3-vl:8b-thinking-q4_K_M
#
# mlx-community/Llama-3.2-3B-Instruct-4bit
# mlx-community/UserLM-8b-8bit
"""

BENCHMARK_STRUCTURED = """\
{
  "metadata": {
    "id": "structured-output-progressive",
    "description": "5-stage difficulty test for Structured Output capabilities.",
    "active": true
  },
  "prompts": {
    "system": "You are a precise data extraction engine. Extract information from the input and format it as valid JSON based on the REQUIREMENTS. \\n\\nRULES:\\n1. Output ONLY JSON.\\n2. Do not include markdown formatting.\\n3. Follow the exact schema requested."
  },
  "dataset": [
    {
      "id": "200001",
      "input": "SOURCE: 'My name is Suzuki and I live in Tokyo.'\\nREQUIREMENT: Extract `name` (string) and `city` (string).",
      "expected": "{\\"name\\": \\"Suzuki\\", \\"city\\": \\"Tokyo\\"}"
    },
    {
      "id": "200002",
      "input": "SOURCE: 'Please buy ingredients: onions, carrots, and potatoes.'\\nREQUIREMENT: Extract a single field `ingredients` which is an array of strings.",
      "expected": "{\\"ingredients\\": [\\"onions\\", \\"carrots\\", \\"potatoes\\"]}"
    },
    {
      "id": "200003",
      "input": "SOURCE: 'User ID 5521 is currently active. Session duration: 45 minutes.'\\nREQUIREMENT: Extract `user_id` (integer), `is_active` (boolean), `duration_min` (integer). Do not output strings for numbers or booleans.",
      "expected": "{\\"user_id\\": 5521, \\"is_active\\": true, \\"duration_min\\": 45}"
    },
    {
      "id": "200004",
      "input": "SOURCE: 'Item A is in stock. Item B is sold out. Item C status is unknown.'\\nREQUIREMENT: Extract list of items. Each item has `name` and `status`. Status MUST be one of: 'AVAILABLE', 'OUT_OF_STOCK', null (if unknown).",
      "expected": "{\\"items\\": [{\\"name\\": \\"Item A\\", \\"status\\": \\"AVAILABLE\\"}, {\\"name\\": \\"Item B\\", \\"status\\": \\"OUT_OF_STOCK\\"}, {\\"name\\": \\"Item C\\", \\"status\\": null}]}"
    },
    {
      "id": "200005",
      "input": "SOURCE: 'Order #999 from Alice. She wants a red shirt (Size: M) and blue jeans. NOTE: The jeans are out of stock so cancel that item. Also send a catalog.'\\nREQUIREMENT: Extract `order_id` (string), `customer` (string), and `valid_items` (array of objects). Each item has `product`, `color`, `size` (null if unspecified). Do NOT include cancelled items. Do NOT include the catalog in items.",
      "expected": "{\\"order_id\\": \\"999\\", \\"customer\\": \\"Alice\\", \\"valid_items\\": [{\\"product\\": \\"shirt\\", \\"color\\": \\"red\\", \\"size\\": \\"M\\"}]}"
    }
  ]
}
"""

BENCHMARK_REASONING = """\
{
  "metadata": {
    "id": "reasoning-progressive",
    "description": "5-stage difficulty test for Logical Reasoning and Chain of Thought.",
    "active": true
  },
  "prompts": {
    "system": "You are a logical assistant. Think step-by-step before answering. \\n\\nRULES:\\n1. Keep your reasoning concise.\\n2. End your answer with 'FINAL ANSWER: <answer>'."
  },
  "dataset": [
    {
      "id": "100001",
      "input": "Level 1 (Direct Comparison): An elephant is bigger than a car. A car is bigger than a mouse. Is the elephant bigger than the mouse?",
      "expected": "Yes"
    },
    {
      "id": "100002",
      "input": "Level 2 (State Tracking): I put a green apple in a box. I close the box. I move the box to the kitchen. Then I open the box. What is inside the box?",
      "expected": "A green apple"
    },
    {
      "id": "100003",
      "input": "Level 3 (Negation Logic): The light switch is currently OFF. If I flip it once, it turns ON. If I flip it twice, it turns OFF. I flip the switch 3 times. Is the light ON or OFF?",
      "expected": "ON"
    },
    {
      "id": "100004",
      "input": "Level 4 (Attention Trap): David's father has three sons. The first son is named Snap. The second son is named Crackle. What is the name of the third son?",
      "expected": "David"
    },
    {
      "id": "100005",
      "input": "Level 5 (Multi-constraint Logic): Three friends (Alice, Bob, Charlie) are sitting in a row. 1. Alice is not at either end. 2. Bob is to the left of Alice. Who is sitting in the middle?",
      "expected": "Alice"
    }
  ]
}
"""

GITIGNORE_ENTRIES = ["reports/"]
