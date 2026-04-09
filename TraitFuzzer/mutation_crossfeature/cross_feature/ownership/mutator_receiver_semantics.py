from __future__ import annotations

from mutation_crossfeature.base_mutator import LLMMutatorBase, MutatorMeta, MutationTarget


class OwnershipReceiverSemanticsMutator(LLMMutatorBase):
    """Mutator 4: Local borrowing transformation."""

    meta = MutatorMeta(
        key="ownership_2",
        name="local_borrowing_transformation",
        category="ownership",
    )

    system_prompt = """Rust mutator: local borrowing transformation.
Do one small local mutation, keep syntax valid, and output only Rust code or NO_MUTATION.
"""

    def build_prompt(self, seed_code: str, target: MutationTarget) -> str:
        return f"""Operator: local borrowing transformation.

{self._target_scope_block(target)}Edit only target-related functions/methods or local bindings.
Convert one by-value flow into a borrowed flow (&T or &mut T),
or change one receiver from self to &self / &mut self.
Apply minimal local updates so signatures/usages stay consistent.
Keep edits minimal, local, and avoid renaming/reordering.

Rust seed:
{seed_code}

Return Rust code only.
"""
