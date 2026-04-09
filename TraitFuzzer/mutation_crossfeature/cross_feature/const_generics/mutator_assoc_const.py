from __future__ import annotations

from mutation_crossfeature.base_mutator import LLMMutatorBase, MutatorMeta, MutationTarget


class ConstGenericParamMutator(LLMMutatorBase):
    """Mutator 5: Const generic parameter injection."""

    meta = MutatorMeta(
        key="const_1",
        name="const_generic_parameter_injection",
        category="const_generics",
    )

    system_prompt = """Rust mutator: const generic parameter injection.
Do one small local mutation, keep syntax valid, and output only Rust code or NO_MUTATION.
"""

    def build_prompt(self, seed_code: str, target: MutationTarget) -> str:
        return f"""Operator: const generic parameter injection.

{self._target_scope_block(target)}Edit only target-related type/impl/trait sites.
Inject one const generic parameter (for example, <const N: usize>)
into a target-related declaration and propagate the minimum required updates.
Keep edits minimal, local, and syntax-valid.
Avoid broad refactors and renaming/reordering.

Rust seed:
{seed_code}

Return Rust code only.
"""


class ConstDependentTypeMutator(LLMMutatorBase):
    """Mutator 6: Const-dependent type construction."""

    meta = MutatorMeta(
        key="const_2",
        name="const_dependent_type_construction",
        category="const_generics",
    )

    system_prompt = """Rust mutator: const-dependent type construction.
Do one small local mutation, keep syntax valid, and output only Rust code or NO_MUTATION.
"""

    def build_prompt(self, seed_code: str, target: MutationTarget) -> str:
        return f"""Operator: const-dependent type construction.

{self._target_scope_block(target)}Edit only target-related code.
Using an existing or newly introduced const generic parameter, construct one const-dependent type
(for example [T; N]) at a target-related boundary (signature, where-bound helper, or local typed binding).
Apply minimal follow-up edits to keep syntax valid.
Keep edits local and minimal, and avoid renaming/reordering.

Rust seed:
{seed_code}

Return Rust code only.
"""
