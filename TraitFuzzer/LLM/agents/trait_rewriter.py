from typing import Optional
from ..LLM_connector import LLMConnector

class TraitRewriterAgent:
    def __init__(self, connector: LLMConnector):
        self.connector = connector
        self.system_prompt = """You are an expert Rust programmer specializing in refactoring and trait-based design.
Your task is to rewrite the given Rust code to make it more idiomatic and trait-oriented.
Requirements:
1. Abstract concrete behaviors into clear, well-defined traits.
2. Implement these traits for existing structs.
3. Prefer trait-based method calls over concrete implementations to reduce coupling.
4. When appropriate, use advanced trait features such as trait bounds, associated types, default methods, and generic constraints.
5. Avoid unnecessary or meaningless abstractions.
6. Keep logic, behavior, and functionality fully equivalent.
Output ONLY valid Rust code. No explanations.
"""

    def rewrite(self, code: str) -> Optional[str]:
        prompt = f"""Rewrite the following Rust code to use more Traits:

```rust
{code}
```

Return strictly the code.
"""
        try:
            response = self.connector.query(prompt, system_prompt=self.system_prompt)
            # Basic cleanup if the LLM includes markdown backticks despite instructions
            cleaned = response.strip()
            if cleaned.startswith("```rust"):
                cleaned = cleaned[7:]
            if cleaned.startswith("```"):
                cleaned = cleaned[3:]
            if cleaned.endswith("```"):
                cleaned = cleaned[:-3]
            return cleaned.strip()
        except Exception as e:
            print(f"LLM Rewrite failed: {e}")
            return None
