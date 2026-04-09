# TraitFuzzer Framework Overview

This directory is the runnable workspace for TraitFuzzer.

## Project Tree (Short Guide)

```text
TraitFuzzer/
|- .github/                      - Repository automation and Copilot-related metadata.
|- .pids/                        - Runtime PID files created by launcher scripts.
|- LLM/                          - LLM connector layer and model-side helper agents.
|  |- LLM_connector.py           - Unified interface for calling the configured LLM backend.
|  |- agents/                    - Extra LLM-side helpers used by rewriting/mutation flows.
|- Traitor_Agent/                - Two-stage seed rewriting pipeline (trait-dense generation).
|  |- Traitorconfig.json         - Traitor_Agent runtime configuration.
|  |- run_agent.py               - Single-input Traitor_Agent runner entry.
|  |- run_dataset.py             - Batch dataset runner for Traitor_Agent.
|  |- agent.py                   - Core Stage-I / Stage-II transformation logic.
|  |- structure_complexity.py    - Structure/TCS scoring utilities for acceptance decisions.
|  |- pools/                     - Few-shot example pools used by rewriting prompts.
|  |- part_trait_dense_seeds/    - Prepared seed set used by default in many experiments.
|- mutation/                     - Trait-constraint mutation framework.
|  |- mutator_pool.py            - Strategy selection and mutator dispatch logic.
|  |- mutation-AST/              - Rust AST mutation engine and TTCG metric extractor.
|- mutation_crossfeature/        - Cross-feature mutation subsystem (lifetime/ownership/const).
|  |- config_cross.json          - Cross-feature operator and run settings.
|  |- main_cross.py              - Standalone cross-feature driver.
|  |- mutator_registry.py        - Registration map of cross-feature mutators.
|  |- cross_feature/             - Concrete operator implementations by feature family.
|- utils/                        - Shared utility scripts (compiler wrappers, dataset tooling, TTCG helpers).
|- test/                         - Quick helper scripts for local validation.
|- seeds/                        - Input Rust seeds used by mutators/fuzzing.
|- results/                      - Output corpus (crash/hang/error/fate cases) from fuzzing.
|- logs/                         - Runtime logs produced by runners and launch scripts.
|- config.json                   - Main fuzzer configuration (iterations, compiler, paths, modes).
|- main.py                       - Main TraitFuzzer orchestrator.
|- start.sh                      - Interactive launcher for main fuzzing workflow.
|- run_agent.sh                  - Interactive launcher for Traitor_Agent workflow.
|- clean.py                      - Cleanup helper for generated outputs.
|- my_fuzz.log                   - Main fuzzer runtime log (generated file).
|- ollama.log                    - Ollama service log (generated file).
```

## Notes

- `logs/`, `results/`, `.pids/`, `my_fuzz.log`, and `ollama.log` are runtime/generated artifacts.
- Most daily usage starts from `run_agent.sh` (rewrite seeds) and `start.sh` (main fuzzing).
