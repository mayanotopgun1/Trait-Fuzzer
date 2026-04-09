from __future__ import annotations

from mutation_crossfeature.base_mutator import LLMMutatorBase, MutatorMeta, MutationTarget


class OwnershipPointerWrapMutator(LLMMutatorBase):
    """Mutator 3: Smart-pointer wrapping (Box/Rc)."""

    meta = MutatorMeta(
        key="ownership_1",
        name="smart_pointer_wrapping",
        category="ownership",
    )

    system_prompt = """Rust mutator: smart-pointer wrapping.
Do one small local mutation, keep syntax valid, and output only Rust code or NO_MUTATION.
"""

    def build_prompt(self, seed_code: str, target: MutationTarget) -> str:
        return f"""Operator: smart-pointer wrapping.

{self._target_scope_block(target)}Edit only target-related code.
Wrap one local target-related value/path as Box<T> or Rc<T>.
Do minimal call-site fixes if needed.
Keep edits minimal, local, and syntax-valid.
Avoid full refactors and renaming/reordering.

Rust seed:
{seed_code}

Return Rust code only.
"""
