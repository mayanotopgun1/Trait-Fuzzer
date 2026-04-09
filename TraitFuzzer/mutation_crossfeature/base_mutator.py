from __future__ import annotations

from dataclasses import dataclass
from typing import Optional

from LLM import LLMConnector


@dataclass(frozen=True)
class MutatorMeta:
    key: str
    name: str
    category: str


@dataclass(frozen=True)
class MutationTarget:
    type_name: str
    trait_name: Optional[str] = None


class LLMMutatorBase:
    meta: MutatorMeta
    system_prompt: str

    def __init__(self, connector: LLMConnector):
        self.connector = connector

    def build_prompt(self, seed_code: str, target: MutationTarget) -> str:
        raise NotImplementedError

    def _target_scope_block(self, target: MutationTarget) -> str:
        type_name = (target.type_name or "").strip()
        trait_name = (target.trait_name or "").strip()

        weak_type_names = {"t", "u", "v", "s", "type", "self", "_"}
        use_type = bool(type_name) and type_name.lower() not in weak_type_names
        use_trait = bool(trait_name) and trait_name.lower() not in {"none", "null", "n/a"}

        if not (use_type or use_trait):
            return ""

        lines = []
        if use_type:
            lines.append(f"TARGET_TYPE: {type_name}")
        if use_trait:
            lines.append(f"TARGET_TRAIT: {trait_name}")
        return "\n".join(lines) + "\n\n"

    def mutate(self, seed_code: str, target: MutationTarget) -> Optional[str]:
        prompt = (
            self.build_prompt(seed_code, target)
            + "\n\nGlobal rules:"
            + "\n- Edit only code related to the selected target scope."
            + "\n- When you mutate a site, also apply necessary corresponding adjustments so the code stays consistent."
            + "\n- Keep Rust syntax valid."
            + "\n- Output only Rust code (no markdown fences)."
        )
        try:
            response = self.connector.query(prompt, system_prompt=self.system_prompt)
        except Exception:
            return None

        if response is None:
            return None

        cleaned = response.strip()
        if cleaned.startswith("```rust"):
            cleaned = cleaned[7:]
        if cleaned.startswith("```"):
            cleaned = cleaned[3:]
        if cleaned.endswith("```"):
            cleaned = cleaned[:-3]
        cleaned = cleaned.strip()
        if cleaned == "NO_MUTATION":
            return None
        return cleaned or None
