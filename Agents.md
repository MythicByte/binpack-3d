# AGENTS.md
Guidance for agentic coding agents working in this repository.
This is a Rust/Cargo project. Agents should be pragmatic: verify requirements,
then make the smallest correct change. If something is unclear, ask rather than
guessing.
## Project Requirements (Scope)
Single-bin packing only:
- Pack items into exactly ONE bin/container.
- Output is placements + unplaced items for that one bin.
Geometry + units:
- Items are axis-aligned cuboids (rectangular boxes).
- Item dimensions and coordinates are in millimeters (mm).
- Items can rotate in all directions: support all 6 axis-aligned orientations.
Spacing:
- A configurable spacing `gap_mm` must be supported.
- Gap handling must be consistent and tested:
  - item-item spacing is required
  - if item-wall clearance is required, it must be explicit (ask if unclear)
Objective:
- Priority-first packing:
  1. Place most important items first (priority order).
  2. Then maximize used space/utilization until the bin is full (no more items fit).
- Deterministic output for the same inputs/options.
Feasibility invariants for every produced result:
- All placements are within bin bounds.
- No overlaps, considering `gap_mm`.
- Each item is placed at most once.
Out of scope unless explicitly requested:
- Multi-bin packing.
- Physics constraints (stability, load limits, center of mass).
- Non-axis-aligned rotations.
Review expectation:
- Your main job is review: find logic errors, panic/overflow risks, and
  maintainability issues; explain why they matter; propose fixes.
## Cursor / Copilot Rules
Checked for agent instruction files:
- `.cursor/rules/`: not found
- `.cursorrules`: not found
- `.github/copilot-instructions.md`: not found
If any of these appear later, include them here and treat them as binding.
## Commands (Build / Lint / Test)
Run from repo root.
Build:
- `cargo build`
- `cargo build --release`
Run (if a binary exists):
- `cargo run`
- `cargo run -- <args>`
Format:
- `cargo fmt`
- Check formatting (CI-style):
  `cargo fmt --all -- --check`
Lint (Clippy):
- `cargo clippy --all-targets --all-features`
- Treat warnings as errors (recommended for CI):
  `cargo clippy --all-targets --all-features -- -D warnings`
Tests:
- `cargo test`
- Show output:
  `cargo test -- --nocapture`
- Single-threaded (helps determinism/debugging):
  `cargo test -- --test-threads=1`
Run a single test (unit tests; substring match on full test name/path):
- `cargo test <name_substring>`
Examples:
- `cargo test overlap`
- `cargo test rotation`
- `cargo test gap`
Integration tests (if `tests/` exists):
- Run one integration test file:
  `cargo test --test <test_file_stem>`
- Run one test in that file:
  `cargo test --test <test_file_stem> <name_substring>`
Ignored tests:
- `cargo test -- --ignored`
- `cargo test -- --include-ignored`
Docs:
- `cargo doc --open`
- Run doctests:
  `cargo test --doc`
Optional (only if installed/configured):
- `cargo audit`
- `cargo deny check`
## Code Style Guidelines
Formatting:
- Use rustfmt defaults; do not hand-format.
- Prefer early returns over deep nesting.
- Keep functions focused; extract helpers when it improves clarity.
Imports:
- Avoid glob imports (`use x::*`) in library code.
- Import order: `std` then external crates then `crate`.
- Remove unused imports (clippy clean).
Naming:
- Types/traits/enums: `PascalCase`
- Functions/vars/modules: `snake_case`
- Constants: `SCREAMING_SNAKE_CASE`
- Choose domain names that reflect intent: `pack`, `place`, `fits`, `overlaps`.
Types and arithmetic:
- Use integer mm types for geometry (avoid floats in core packing math).
- Prevent overflow:
  - bounds checks like `x + w <= bin_w` must be overflow-safe
  - volume `w*h*d` can overflow small types; use `u128` or checked math
- Validate inputs at boundaries:
  - dimensions must be > 0
  - reject NaN/inf if floats appear in parsing layers
Rotations:
- Support all 6 axis-aligned orientations (permutations of w/h/d).
- Centralize orientation logic; do not duplicate it across algorithms.
- Ensure rotation enumeration is correct and deterministic.
Gap (`gap_mm`):
- Apply `gap_mm` consistently in overlap logic.
- Tests must cover `gap_mm = 0` and a positive gap.
- Decide/document whether “touching” is allowed when `gap_mm = 0` (usually yes).
Error handling:
- No `unwrap()` / `expect()` in core library logic.
- Use `Result` for invalid input and invariant violations.
- Distinguish:
  - invalid input (bad dimensions/options)
  - cannot place (valid input, heuristic can’t fit)
  - internal bug (invariant failure)
Determinism:
- Do not rely on `HashMap` iteration order for packing decisions.
- Use stable sorts with explicit tie-breakers (priority desc, then size/volume, then id).
Testing expectations:
- Tests should validate invariants on every packing result:
  - within bounds
  - no overlap (with gap)
  - orientation correctness
  - priority-first behavior under constrained space
Review checklist (before accepting changes):
- Overlap logic correct (`<` vs `<=`) and matches the chosen gap/touching policy
- Bounds checks correct and overflow-safe
- All 6 orientations covered
- Deterministic ordering/tie-breaking
- No panics on malformed input
