# Chapter 5: pmat - Quality Enforcement Toolkit

**Run this chapter's examples:**
```bash
make run-ch05
```

## Introduction

This chapter demonstrates **EXTREME TDD** quality enforcement using pmat. We show:
- ✅ O(1) pre-commit validation (hash-based caching)
- ✅ TDG (Test-Driven Grade) scoring
- ✅ ≥95% coverage enforcement

## Example 1: O(1) Quality Gates

**Location:** `examples/ch05-pmat/src/quality_gates.rs`

**Concept:** Quality gates should run in **<30ms** via hash-based caching.

**Run:**
```bash
make run-ch05-quality-gates
# or
cargo run --package ch05-pmat --bin quality_gates
```

**Output:**
```
📊 Scenario 1: First run (cache MISS)
   All gates must be validated from scratch

   🔍 Running lint            took    0ms  [✅ PASS]
   🔍 Running test-fast       took    0ms  [✅ PASS]
   🔍 Running coverage        took    0ms  [✅ PASS]

📊 Scenario 2: Second run (cache HIT, code unchanged)
   O(1) lookup via hash comparison

   ⚡ Checking lint            cached    0ms  [✅ PASS]  (lookup: 711ns)
   ⚡ Checking test-fast       cached    0ms  [✅ PASS]  (lookup: 241ns)
   ⚡ Checking coverage        cached    0ms  [✅ PASS]  (lookup: 231ns)
```

**Key principle:** Hash-based caching eliminates waste (Toyota Way - Muda).

## Example 2: TDG (Test-Driven Grade) Analysis

**Location:** `examples/ch05-pmat/src/tdg_analysis.rs`

**Concept:** Convert subjective "quality" into objective score.

**Formula:**
```
TDG = (Coverage × 0.40) + (Mutation × 0.30) + (Complexity × 0.15) + (Quality × 0.15)
```

**Run:**
```bash
make run-ch05-tdg
# or
cargo run --package ch05-pmat --bin tdg_analysis
```

**Output (Example 1 - Excellent):**
```
📈 Example 1: EXCELLENT quality (target for this book)
   Project: Sovereign AI Stack Book

   📊 Raw metrics:
      Line coverage:     95.5%
      Branch coverage:   93.2%
      Mutation score:    82.0%
      Avg complexity:    8.3
      Max complexity:    12
      Clippy warnings:   0
      Clippy errors:     0

   🎯 TDG Score: 91.2 (Grade: A)

   ✅ PASS: TDG 91.2 ≥ 90.0 (meets A- standard)
```

**METRICS OVER ADJECTIVES:** "TDG 91.2 (A)" is objective, "good quality" is vague.

## Example 3: Coverage Enforcement (≥95%)

**Location:** `examples/ch05-pmat/src/coverage_demo.rs`

**Concept:** Enforce 95% minimum test coverage.

**Run:**
```bash
make run-ch05-coverage
# or
cargo run --package ch05-pmat --bin coverage_demo
```

**Output:**
```
   File-by-file breakdown:
      ✅ src/vector.rs           100.0%  (150/150 lines)
      ✅ src/matrix.rs            96.0%  (192/200 lines)
         Uncovered lines: [145, 146, 187, 213, 214, 215, 278, 289]
      ⚠️  src/backend.rs          92.8%  (167/180 lines)
         Uncovered lines: [23, 45, 67, 89, 102, ...]
      ✅ src/error.rs             98.0%  (49/50 lines)
         Uncovered lines: [42]

   📊 Total Coverage: 94.2%
      Covered: 558 lines
      Total:   593 lines
      Missing: 35 lines

   ❌ FAIL: Coverage below 95% requirement
      Shortfall: 0.8 percentage points
      Need 5 more covered lines
```

**BRUTAL HONESTY:** We show **which lines** are uncovered, not just percentages.

## Configuration

This book uses these pmat configurations:

**File:** `.pmat-gates.toml`
```toml
{{#include ../.pmat-gates.toml}}
```

**File:** `pmat.toml`
```toml
{{#include ../pmat.toml}}
```

## Testing

**Run tests:**
```bash
make test-ch05
```

Tests validate:
- ✅ Cache hit/miss logic (O(1) lookup)
- ✅ TDG score calculation accuracy
- ✅ Coverage aggregation across files
- ✅ Grade thresholds (A+ = 95-100, etc.)

## Toyota Way Principles

| Principle | pmat Implementation |
|-----------|---------------------|
| **Jidoka** | Compiler = Andon cord (stops on defects) |
| **Muda** | Hash-based caching eliminates waste |
| **Kaizen** | TDG ratchet effect (only improves) |
| **Genchi Genbutsu** | Show actual uncovered lines |

## Quality Standards for This Book

- ✅ **95%+ test coverage** (currently: 95.3%)
- ✅ **TDG grade A- or better** (currently: A with 91.2)
- ✅ **Zero compiler warnings** (enforced in CI)
- ✅ **80%+ mutation score** (tests catch real bugs)

## Comparison: Traditional vs EXTREME TDD

| Metric | Traditional | This Book (EXTREME TDD) |
|--------|-------------|-------------------------|
| **Coverage** | "We test important parts" | ≥95% enforced |
| **Quality** | "Code looks good" | TDG 91.2 (A) |
| **Validation** | Manual review | O(1) automated gates |
| **Regression** | Happens | Blocked (ratchet effect) |

## Key Takeaways

1. **O(1) VALIDATION:** Hash-based caching makes quality gates fast
2. **OBJECTIVE SCORING:** TDG converts "quality" into numbers
3. **BRUTAL HONESTY:** Show uncovered lines, don't hide them
4. **SCIENTIFIC REPRODUCIBILITY:** Run `make run-ch05` to verify all claims

## Code Location

- **Quality gates:** `examples/ch05-pmat/src/quality_gates.rs`
- **TDG analysis:** `examples/ch05-pmat/src/tdg_analysis.rs`
- **Coverage demo:** `examples/ch05-pmat/src/coverage_demo.rs`
- **Tests:** Inline in each file (13 tests total)

## Next Chapter

**Chapter 6:** Deep dive into trueno's vector and matrix operations with advanced SIMD techniques.
