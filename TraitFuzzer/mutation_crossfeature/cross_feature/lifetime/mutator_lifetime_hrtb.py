from __future__ import annotations

from mutation_crossfeature.base_mutator import LLMMutatorBase, MutatorMeta, MutationTarget


class LifetimeHigherRankMutator(LLMMutatorBase):
    """Mutator 2: Higher-ranked lifetime injection."""

    meta = MutatorMeta(
        key="lifetime_2",
        name="higher_rank_lifetime_injection",
        category="lifetime",
    )

    system_prompt = """Rust mutator: higher-ranked lifetime injection.
Do one small local mutation, keep syntax valid, and output only Rust code or NO_MUTATION.
"""

    def build_prompt(self, seed_code: str, target: MutationTarget) -> str:
        return f"""Operator: higher-ranked lifetime injection.

{self._target_scope_block(target)}Edit only target-related impl/where constraints.
Insert one local HRTB bound (for<'a> ...).
Keep edits minimal, local, and syntax-valid.
Avoid global refactors and renaming/reordering.

Rust seed:
{seed_code}

Return Rust code only.
"""
