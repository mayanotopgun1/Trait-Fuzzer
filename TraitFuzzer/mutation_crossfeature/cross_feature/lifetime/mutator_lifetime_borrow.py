from __future__ import annotations

from mutation_crossfeature.base_mutator import LLMMutatorBase, MutatorMeta, MutationTarget


class LifetimeBorrowMutator(LLMMutatorBase):
    """Mutator 1: Lifetime parameter injection.

    Inject one extra lifetime parameter (for example `'a`) into a target-related
    type or impl definition, then apply the minimum consistent signature updates.
    """

    meta = MutatorMeta(
        key="lifetime_1",
        name="lifetime_parameter_injection",
        category="lifetime",
    )

    system_prompt = """Rust mutator: lifetime parameter injection.
Do one small local mutation, keep syntax valid, and output only Rust code or NO_MUTATION.
"""

    def build_prompt(self, seed_code: str, target: MutationTarget) -> str:
        return f"""Operator: lifetime parameter injection.

{self._target_scope_block(target)}Edit only target-related code.
Introduce one additional lifetime parameter (e.g., 'a) in a target-related type or impl definition.
Propagate only the required signature/type updates to keep the code consistent.
Keep edits local and minimal, and avoid renaming/reordering.

Rust seed:
{seed_code}

Return Rust code only.
"""
