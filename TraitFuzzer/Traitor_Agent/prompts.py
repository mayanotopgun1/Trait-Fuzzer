from typing import List

from .fewshot_pool import ShotExample


_FEATURE_BRIEF = {
    "GAT": "GAT: associated types can depend on lifetimes/types (e.g., type Out<'a>).",
    "specialization": "specialization: provide a default impl/method, then override for specific types.",
    "RPITIT": "RPITIT: trait method itself returns `impl Trait`.",
    "TAIT": "TAIT: define `type Alias = impl Trait` and reuse that opaque alias.",
    "const_trait": "const_trait: use `#![feature(const_trait_impl)]` and `impl const Trait for Type` where valid.",
}


def _feature_guide_text() -> str:
    order = ["GAT", "specialization", "RPITIT", "TAIT", "const_trait"]
    return "\n".join(f"- {name}: {_FEATURE_BRIEF[name]}" for name in order)


def _format_shots(shots: List[ShotExample]) -> str:
    if not shots:
        return "(No examples available)"

    blocks = []
    for i, s in enumerate(shots, start=1):
        blocks.append(
            f"Example {i} [{s.source}]\n"
            f"[Input]\n```rust\n{s.input_code}\n```\n"
            f"[Output]\n```rust\n{s.output_code}\n```"
        )
    return "\n\n".join(blocks)


def stage1_prompt(program: str, shots: List[ShotExample]) -> str:
    return (
        "Stage-I: conservative transformation.\n"
        "Task: rewrite the Rust program to increase trait participation while preserving behavior.\n"
        "Rules:\n"
        "1) Extract standalone/inherent methods into traits when possible.\n"
        "2) Implement created traits for corresponding types.\n"
        "3) Replace direct calls with trait-based dispatch where natural.\n"
        "4) Keep structure conservative and avoid over-aggressive changes.\n"
        "5) Output ONLY Rust code.\n\n"
        f"Few-shot examples:\n{_format_shots(shots)}\n\n"
        f"Program:\n```rust\n{program}\n```\n"
    )


def feature_select_prompt(program: str) -> str:
    return (
        "Evaluate whether each advanced trait feature can be integrated into this program without changing original semantics.\n"
        "Candidate feature guide:\n"
        f"{_feature_guide_text()}\n"
        "Decision rules:\n"
        "1) Mark YES only if the feature can be added naturally with minimal structural disruption.\n"
        "2) Mark NO if adding it would require forced or unrelated refactors.\n"
        "3) Keep original semantics unchanged.\n"
        "4) If the program already contains the feature, answer NO for that feature.\n"
        "Output format (EXACTLY 5 lines, one per feature):\n"
        "GAT: yes/no | short reason\n"
        "specialization: yes/no | short reason\n"
        "RPITIT: yes/no | short reason\n"
        "TAIT: yes/no | short reason\n"
        "const_trait: yes/no | short reason\n\n"
        f"Program:\n```rust\n{program}\n```\n"
    )


def stage2_prompt(program: str, feature: str, shots: List[ShotExample]) -> str:
    brief = _FEATURE_BRIEF.get(feature, f"{feature}: apply this advanced trait feature explicitly.")
    return (
        "Stage-II: aggressive transformation.\n"
        "Inject advanced trait feature(s) while preserving core logic.\n"
        "Selected feature: " + feature + "\n"
        "Feature brief: " + brief + "\n"
        "Feature guide (reference while transforming):\n"
        f"{_feature_guide_text()}\n"
        "Rules:\n"
        "1) Integrate selected feature explicitly.\n"
        "2) Preserve behavioral intent.\n"
        "3) Keep code syntactically valid Rust.\n"
        "4) Output ONLY Rust code.\n\n"
        f"Few-shot examples:\n{_format_shots(shots)}\n\n"
        f"Program:\n```rust\n{program}\n```\n"
    )


def repair_prompt(
    original_code: str,
    transformed_code: str,
    compiler_error: str,
    stage_name: str,
    intent: str,
) -> str:
    return (
        f"{stage_name} repair task.\n"
        "We attempted to transform the original code, but the transformed version failed to compile.\n"
        f"Transformation intent: {intent}\n"
        "Please fix the transformed code so that it compiles, while preserving original semantics and keeping the intended transformation as much as possible.\n"
        "Focus on ERROR diagnostics; ignore WARNING diagnostics unless they directly mention an error root cause.\n"
        "Output ONLY Rust code.\n\n"
        f"Original code (before transformation):\n```rust\n{original_code}\n```\n\n"
        f"Current transformed code (to be fixed):\n```rust\n{transformed_code}\n```\n\n"
        f"Compiler error:\n```\n{compiler_error}\n```\n\n"
    )
