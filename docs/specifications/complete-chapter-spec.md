# Complete Chapter Specification

> Requirements for a chapter to be considered "complete" in the Sovereign AI Stack Book

## Status: DRAFT - Pending Review

## Standardized Work: Chapter Completion Checklist

A chapter is **complete** when ALL of the following criteria are met. This acts as our **Standardized Work**—the baseline for quality and future Kaizen.

### 1. Working Code Examples (REQUIRED)

| Requirement | Validation | Toyota Principle |
|-------------|------------|------------------|
| Example exists in `examples/chXX-topic/` | `ls examples/chXX-*` | Genchi Genbutsu |
| Code compiles without warnings | `cargo build -p chXX-*` | Jidoka (Stop on defect) |
| Binary is runnable | `cargo run -p chXX-*` | Poka-Yoke |
| Makefile target exists | `make run-chXX` | Standardization |

**Minimum code requirements:**
- At least one complete, runnable example (30-100 lines)
- Code demonstrates the chapter's core concept
- No placeholder TODOs in production code
- Uses `{{#include}}` to embed code in markdown

### 2. Test Coverage (REQUIRED)

| Metric | Minimum | Validation |
|--------|---------|------------|
| Unit tests | 3+ tests | `cargo test -p chXX-*` |
| Line coverage | 80%+ | `cargo tarpaulin -p chXX-*` |
| All tests pass | 100% | CI green |

**Test requirements:**
- Tests validate core functionality
- Tests are deterministic (no flaky tests) - *Respect for People*
- Edge cases covered where applicable

### 3. Documentation Structure (REQUIRED)

Every chapter markdown file must contain:

```markdown
# Chapter X: Title

**Run this chapter's example:**
`make run-chXX`
```

## Introduction
[2-3 paragraphs explaining the concept]

## The Example: `example_name.rs`
[Code included via {{#include}}]

## Running the Example
[Commands to run with expected output]

## Key Concepts
[3-5 subsections explaining principles demonstrated]

## Testing
[How to run tests, what they validate]

## Comparison/Benchmarks (if applicable)
[Tables comparing approaches, benchmark results]

## Next Steps
[Links to related chapters]

## Code Location
[Paths to example, tests, Makefile targets]

## Key Takeaway
[One sentence summary]
```

### 4. Scientific Reproducibility (REQUIRED)

| Requirement | Validation |
|-------------|------------|
| Deterministic output | Run 5x, same result |
| No network dependencies | `strace` shows no network calls |
| Environment documented | Hardware/software specs if benchmarking |
| Claims are verifiable | Reader can reproduce via `make` |

### 5. Minimum Content Length (Heijunka)

| Section | Minimum |
|---------|---------|
| Total chapter | 100 lines |
| Introduction | 50 words |
| Code example | 30 lines |
| Key Concepts | 3 subsections |

### 6. Quality Gates (REQUIRED)

```bash
# All must pass before chapter is "complete"
# Any failure here is an Andon pull (stop the line).
cargo build -p chXX-* --all-targets    # Compiles
cargo test -p chXX-*                    # Tests pass
cargo clippy -p chXX-* -- -D warnings  # No warnings (Jidoka)
make run-chXX                           # Runs successfully
pmat validate-docs                      # Links valid
```

## Chapter Status Definitions

| Status | Criteria |
|--------|----------|
| **Planned** | Stub file exists, no working code |
| **In Progress** | Working code exists, <80% of checklist complete |
| **Complete** | 100% of checklist complete, CI passing |
| **Published** | Complete + reviewed + deployed to book site |

## Current Chapter Status (Visual Control)

| Chapter | Lines | Code | Tests | Docs | Status |
|---------|-------|------|-------|------|--------|
| ch01-hello-sovereign | 134 | Yes | Yes | Yes | Complete |
| ch02-crisis | 13 | No | No | No | Planned |
| ch03-trueno | 125 | Yes | Yes | Yes | Complete |
| ch04-bft | 18 | No | No | No | Planned |
| ch05-pmat | 190 | Yes | Yes | Yes | Complete |
| ch06-trueno-core | 18 | No | No | No | Planned |
| ch07-trueno-gpu | 18 | No | No | No | Planned |
| ch08-transpilation-intro | 18 | No | No | No | Planned |
| ch09-bashrs | 18 | No | No | No | Planned |
| ch10-depyler | 18 | No | No | No | Planned |
| ch11-decy | 18 | No | No | No | Planned |
| ch12-aprender | 18 | No | No | No | Planned |
| ch13-realizar | 18 | No | No | No | Planned |
| ch14-entrenar | 18 | No | No | No | Planned |
| ch15-trueno-db | 18 | No | No | No | Planned |
| ch16-trueno-graph | 18 | No | No | No | Planned |
| ch17-batuta | 18 | No | No | No | Planned |
| ch18-renacer | 18 | No | No | No | Planned |
| ch19-repartir | 18 | No | No | No | Planned |
| ch20-ml-pipeline | 18 | No | No | No | Planned |
| ch21-compliance | 18 | No | No | No | Planned |
| ch22-deployment | 18 | No | No | No | Planned |
| ch23-citl | 490 | Yes | Yes | Yes | Complete |

**Summary:** 4/23 chapters complete (17%)

## Validation Commands

```bash
# Check chapter status
pmat work status

# Validate all documentation links
pmat validate-docs

# Run all chapter tests
make test

# Build entire book
mdbook build

# Full quality gate
make validate
```

## Example: Complete Chapter Structure

```
examples/ch01-intro/
├── Cargo.toml
├── src/
│   └── hello_sovereign.rs    # Main example with inline tests
└── README.md                  # Optional: additional notes

src/ch01-hello-sovereign.md    # Chapter markdown (134+ lines)
```

## Automation: Chapter Completion Check

Future: `pmat` command to validate chapter completion (Kaizen):

```bash
# Proposed command
pmat book validate-chapter ch01

# Output:
# ✅ Code exists: examples/ch01-intro/
# ✅ Tests pass: 3/3
# ✅ Coverage: 87%
# ✅ Docs complete: 134 lines
# ✅ Links valid: 5/5
# ✅ Chapter ch01 is COMPLETE
```

## Toyota Way Review Annotations

The following annotations track the alignment of this specification with Toyota Production System (TPS) principles:

1.  **Standardized Work:** This specification serves as the "Standardized Work" for chapter creation, serving as a baseline for Kaizen.
2.  **Jidoka (Andon):** The requirement "Code compiles without warnings" acts as an Andon cord; any warning stops the process immediately.
3.  **Genchi Genbutsu (Go & See):** "Validation" steps require direct observation of the process (running commands), not just theoretical correctness.
4.  **Muda (Waste Elimination):** The "Minimum Content Length" checks prevent "gold plating" (over-processing) while ensuring sufficient value.
5.  **Visual Control:** The "Current Chapter Status" table provides immediate visual feedback on the project's state, exposing bottlenecks.
6.  **Poka-Yoke (Error Proofing):** The `make run-chXX` requirement mistake-proofs the execution process for readers.
7.  **Heijunka (Leveling):** Standardizing chapter structure (3-5 key concepts, 30-100 lines of code) helps level the cognitive load for readers and the workload for authors.
8.  **Scientific Thinking:** The "Scientific Reproducibility" section enforces hypothesis testing (verifiable claims) over assumptions.
9.  **Respect for People:** Ensuring "No flaky tests" respects the time of future maintainers and readers.
10. **Kaizen (Continuous Improvement):** The "Automation" section identifies a clear path for future improvement, moving from manual checks to automated validation.

## References

- [SPEC.md](../../SPEC.md) - Book specification
- [batuta oracle](https://github.com/paiml/batuta) - Stack recommendations
- [pmat](https://github.com/paiml/pmat) - Quality enforcement