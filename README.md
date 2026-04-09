## TraitFuzzer

TraitFuzzer is a tool for systematically validating Rust compilers. It introduces a validation approach based on the Trait-Type Constraint Graph (TTCG). TTCG captures relationships between types and traits and enables derivation of the Trait Constraint Space (TCS) for mutation. Guided by TTCG, TraitFuzzer rewrites ordinary seeds into trait-dense variants and performs both trait-constraint mutation and cross-feature mutation to drive compilers into deeper trait-solving logic. In practice, TraitFuzzer has uncovered 45 confirmed compiler bugs, including 36 in rustc and 9 in gccrs, with 9 already fixed. It also outperforms existing baselines in bug-finding ability and coverage.

### Quick Start

1. Enter the runnable project directory.

```bash
cd TraitFuzzer
```

2. Install Rust toolchains (stable + nightly).

```bash
curl https://sh.rustup.rs -sSf | sh
source "$HOME/.cargo/env"
rustup toolchain install stable nightly
rustup default stable
```

3. Download Rust UI test suite into a local seeds directory.

```bash
git clone --depth 1 https://github.com/rust-lang/rust.git
mkdir -p seeds/rust_ui
cp -r rust/tests/ui/* seeds/rust_ui/
```

4. Enable LLM-based seed rewriting.
Install Ollama: https://ollama.com/download
Model page: https://ollama.com/library/qwen2.5-coder

```bash
ollama pull qwen2.5-coder:14b
printf '1\n' | ./run_agent.sh
```

5. Run fuzzing.

```bash
printf '1\n' | ./start.sh
```

6. Stop services (recommended when finished).

```bash
# Stop Traitor_Agent runner + ollama
printf '2\n' | ./run_agent.sh

# Stop fuzzer main.py (+ choose no for extra cleanup prompts)
printf '2\n0\n0\n' | ./start.sh
```

### Detected Bugs (rustc)
| Compiler | Type | Bug Link | Status |
|---|---|---|---|
| rustc | ICE | https://github.com/rust-lang/rust/issues/151069 | confirmed |
| rustc | ICE | https://github.com/rust-lang/rust/issues/151477 | fixed |
| rustc | ICE | https://github.com/rust-lang/rust/issues/151894 | confirmed |
| rustc | ICE | https://github.com/rust-lang/rust/issues/151964 | confirmed |
| rustc | ICE | https://github.com/rust-lang/rust/issues/152295 | confirmed |
| rustc | ICE | https://github.com/rust-lang/rust/issues/152405 | confirmed |
| rustc | ICE | https://github.com/rust-lang/rust/issues/153195 | dup |
| rustc | ICE | https://github.com/rust-lang/rust/issues/153241 | confirmed |
| rustc | ICE | https://github.com/rust-lang/rust/issues/150751 | dup |
| rustc | ICE | https://github.com/rust-lang/rust/issues/150753 | dup |
| rustc | ICE | https://github.com/rust-lang/rust/issues/150770 | dup |
| rustc | ICE | https://github.com/rust-lang/rust/issues/150854 | dup |
| rustc | ICE | https://github.com/rust-lang/rust/issues/151579 | fixed |
| rustc | ICE | https://github.com/rust-lang/rust/issues/154568 | confirmed |
| rustc | ICE | https://github.com/rust-lang/rust/issues/154533 | confirmed |
| rustc | ICE | https://github.com/rust-lang/rust/issues/151631 | fixed |
| rustc | ICE | https://github.com/rust-lang/rust/issues/153163 | fixed |
| rustc | ICE | https://github.com/rust-lang/rust/issues/153842 | fixed |
| rustc | ICE | https://github.com/rust-lang/rust/issues/152205 | fixed |
| rustc | ICE | https://github.com/rust-lang/rust/issues/154403 | fixed |
| rustc | ICE | https://github.com/rust-lang/rust/issues/154073 | dup |
| rustc | ICE | https://github.com/rust-lang/rust/issues/153912 | fixed |
| rustc | ICE | https://github.com/rust-lang/rust/issues/150954 | dup |
| rustc | ICE | https://github.com/rust-lang/rust/issues/154367 | fixed |
| rustc | Hang | https://github.com/rust-lang/rust/issues/150583 | confirmed |
| rustc | Hang | https://github.com/rust-lang/rust/issues/150794 | confirmed |
| rustc | Hang | https://github.com/rust-lang/rust/issues/150858 | confirmed |
| rustc | Hang | https://github.com/rust-lang/rust/issues/150989 | confirmed |
| rustc | Hang | https://github.com/rust-lang/rust/issues/151503 | confirmed |
| rustc | Hang | https://github.com/rust-lang/rust/issues/151599 | confirmed |
| rustc | Hang | https://github.com/rust-lang/rust/issues/151671 | confirmed |
| rustc | Hang | https://github.com/rust-lang/rust/issues/151723 | confirmed |
| rustc | Hang | https://github.com/rust-lang/rust/issues/150532 | dup |
| rustc | Hang | https://github.com/rust-lang/rust/issues/151632 | confirmed |
| rustc | Hang | https://github.com/rust-lang/rust/issues/151961 | confirmed |
| rustc | Hang | https://github.com/rust-lang/rust/issues/152169 | confirmed |
| rustc | Hang | https://github.com/rust-lang/rust/issues/151636 | confirmed |

### Detected Bugs (gccrs)
| Compiler | Type | Bug Link | Status |
|---|---|---|---|
| gccrs | ICE | https://github.com/Rust-GCC/gccrs/issues/4469 | confirmed |
| gccrs | ICE | https://github.com/Rust-GCC/gccrs/issues/4470 | confirmed |
| gccrs | ICE | https://github.com/Rust-GCC/gccrs/issues/4471 | confirmed |
| gccrs | ICE | https://github.com/Rust-GCC/gccrs/issues/4472 | confirmed |
| gccrs | ICE | https://github.com/Rust-GCC/gccrs/issues/4473 | confirmed |
| gccrs | ICE | https://github.com/Rust-GCC/gccrs/issues/4481 | confirmed |
| gccrs | ICE | https://github.com/Rust-GCC/gccrs/issues/4486 | confirmed |
| gccrs | ICE | https://github.com/Rust-GCC/gccrs/issues/4496 | confirmed |
| gccrs | Hang | https://github.com/Rust-GCC/gccrs/issues/4493 | confirmed |





