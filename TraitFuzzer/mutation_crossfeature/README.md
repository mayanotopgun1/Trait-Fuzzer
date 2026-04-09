Cross-Feature Mutation Operators (M1-M6)

This module uses six cross-feature mutators grouped into three dimensions.
Each operator applies a small, target-scoped edit to increase pressure on trait solving and its interacting compiler subsystems.

A. Lifetime Dimension

M1(C) Lifetime Parameter Injection
- Inject one additional lifetime parameter (for example, 'a) into a target-related type or impl definition.
- Propagate only the minimum required signature updates to keep the program consistent.
- This introduces extra lifetime constraints and increases validation workload in lifetime reasoning.

M2(C) Higher-Rank Trait Bound Injection
- Insert one higher-rank lifetime bound (for<'a> ...) in target-related where/impl constraints.
- HRTB requires the bound to hold for all lifetime instantiations.
- This increases obligation generation and checking complexity during trait solving.

B. Ownership/Borrowing Dimension

M3(C) Local Borrowing Transformation
- Convert one target-related by-value flow into a borrowed flow (&T or &mut T), or convert one receiver from self to &self / &mut self.
- Keep edits local and apply minimum consistency fixes.
- This forces additional borrow-checking interactions with trait solving (lifetime matching and autoderef paths).

M4(C) Smart Pointer Encapsulation
- Wrap one target-related value/path using a standard smart pointer such as Box<T> or Rc<T>.
- Apply minimal call-site/type fixes when required.
- This introduces ownership indirection, deref resolution, and potentially auto-trait (for example Send/Sync) pressure.

C. Constant Evaluation Dimension

M5(C) Const Generic Parameter Injection
- Inject one const generic parameter (for example, <const N: usize>) into a target-related declaration.
- Propagate only necessary type/signature updates.
- This couples trait solving with constant-level typing and const-generic validation.

M6(C) Const-Dependent Type Construction
- Construct one const-dependent type (for example, [T; N]) at a target-related boundary, using an existing or newly introduced const parameter.
- Keep edits local and syntax-valid with minimal follow-up fixes.
- This stresses const normalization/evaluatability interaction during trait solving.

Operator Keys Used in Code
- M1: lifetime_1
- M2: lifetime_2
- M3: ownership_2
- M4: ownership_1
- M5: const_1
- M6: const_2
