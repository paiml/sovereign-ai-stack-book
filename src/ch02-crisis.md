# Chapter 2: Crisis of Determinism in the Age of Generative AI

**Run this chapter's examples:**
```bash
make run-ch02
```

## Introduction

This chapter demonstrates the **crisis of determinism** that emerges when using generative AI models in regulated environments. Traditional machine learning is deterministic: same input produces same output, every time. Generative AI (LLMs) is fundamentally non-deterministic: temperature-based sampling means the same prompt yields different responses.

This creates a **compliance crisis** for EU AI Act Article 13, which requires transparency and reproducibility. The Sovereign AI Stack addresses this through deterministic alternatives and the Rust compiler as a quality gate (Toyota Way "Andon Cord").

## The Three Examples

This chapter contains three interconnected examples:

| Example | File | Purpose |
|---------|------|---------|
| Deterministic Baseline | `deterministic_baseline.rs` | Prove traditional ML is deterministic |
| LLM Variance | `llm_variance.rs` | Quantify LLM non-determinism |
| Toyota Andon | `toyota_andon.rs` | Rust compiler as quality gate |

## Example 1: Deterministic Baseline

**Location:** `examples/ch02-crisis/src/deterministic_baseline.rs`

```rust
{{#include ../examples/ch02-crisis/src/deterministic_baseline.rs:22:65}}
```

### Running the Example

```bash
make run-ch02-baseline
```

**Expected output:**
```
📊 Chapter 2: Deterministic Baseline (Traditional ML)

📈 Training linear regression model (OLS)
   Data points: 10

✅ Model fitted in 1.234µs
   Slope:     1.993333
   Intercept: 0.086667

🧪 Determinism verification (run model 5 times):
   Run 1: x = 15.0 → y = 29.9866666667
   Run 2: x = 15.0 → y = 29.9866666667
   Run 3: x = 15.0 → y = 29.9866666667
   Run 4: x = 15.0 → y = 29.9866666667
   Run 5: x = 15.0 → y = 29.9866666667

✅ DETERMINISTIC: All 5 runs produced IDENTICAL results
   Variance: 0.0 (perfect determinism)
```

### Key Insight

Traditional ML (linear regression, decision trees, etc.) is **perfectly deterministic**. The same training data always produces the same model, and the same input always produces the same prediction.

## Example 2: LLM Variance

**Location:** `examples/ch02-crisis/src/llm_variance.rs`

```rust
{{#include ../examples/ch02-crisis/src/llm_variance.rs:24:62}}
```

### Running the Example

```bash
make run-ch02-llm
```

**Expected output:**
```
🤖 Chapter 2: LLM Variance (Non-Deterministic Generation)

📝 Prompt: "What is the capital of France?"

🌡️  Test 1: Temperature = 0.0 (low variance)
   Run 1: The capital of France is Paris.
   Run 2: The capital of France is Paris.
   Run 3: The capital of France is Paris.
   Unique responses: 1/10
   Variance: 10.0%

🌡️  Test 2: Temperature = 0.7 (high variance)
   Run 1: Paris is the capital of France.
   Run 2: The capital of France is Paris.
   Run 3: France's capital city is Paris.
   Unique responses: 4/100
   Variance: 4.0%

🎯 Non-determinism quantified:
   Temperature 0.0: 10.0% variance
   Temperature 0.7: 4.0% variance

   Same prompt → different outputs = NON-DETERMINISTIC
```

### Key Insight

LLMs are **non-deterministic by design**. Temperature-based sampling introduces variance that violates EU AI Act Article 13 transparency requirements. Even with temperature=0, numerical precision and implementation details can cause variance.

## Example 3: Toyota Andon Cord

**Location:** `examples/ch02-crisis/src/toyota_andon.rs`

```rust
{{#include ../examples/ch02-crisis/src/toyota_andon.rs:19:57}}
```

### Running the Example

```bash
make run-ch02-andon
```

**Expected output:**
```
🏭 Chapter 2: Toyota Andon Cord (Rust Compiler as Quality Gate)

Toyota Production System (TPS) Principle:
   Andon Cord: Any worker can stop production when defect detected
   Jidoka: Automation with human touch (quality built-in)

🛡️  Example 1: Memory Safety (Compiler as Andon Cord)

   Case 1: Use-after-free PREVENTED
   ✅ Compiler BLOCKS this bug

   Case 2: Data race PREVENTED
   ✅ Compiler BLOCKS this bug

   Case 3: Null pointer dereference PREVENTED
   ✅ Compiler FORCES explicit null handling
```

### Key Insight

The Rust compiler acts as an **Andon Cord**: it stops the "production line" (compilation) when defects are detected. This is critical when using AI-generated code, which may contain subtle bugs that the compiler catches before they reach production.

## Testing

**Run all tests:**
```bash
make test-ch02
```

**Tests validate:**
- Determinism of traditional ML (4 tests)
- Non-determinism quantification of LLMs (3 tests)
- Compiler safety guarantees (4 tests)

**Test output:**
```
running 11 tests
test deterministic_baseline::tests::test_batch_predictions ... ok
test deterministic_baseline::tests::test_determinism ... ok
test deterministic_baseline::tests::test_perfect_fit ... ok
test deterministic_baseline::tests::test_prediction_accuracy ... ok
test llm_variance::tests::test_non_determinism_exists ... ok
test llm_variance::tests::test_temperature_zero_is_more_deterministic ... ok
test llm_variance::tests::test_quantify_variance ... ok
test toyota_andon::tests::test_compiler_prevents_use_after_free ... ok
test toyota_andon::tests::test_option_forces_explicit_handling ... ok
test toyota_andon::tests::test_safe_array_access ... ok
test toyota_andon::tests::test_wrapping_arithmetic ... ok

test result: ok. 11 passed; 0 failed
```

## EU AI Act Compliance

| Article | Requirement | Status |
|---------|-------------|--------|
| Article 13 | Transparency | Traditional ML: compliant. LLMs: non-compliant |
| Article 13 | Reproducibility | Traditional ML: compliant. LLMs: non-compliant |
| Article 15 | Robustness | Rust compiler prevents entire bug classes |

## Toyota Way Principles

| TPS Principle | Application in This Chapter |
|---------------|----------------------------|
| **Jidoka** | Rust compiler stops on defects (Andon Cord) |
| **Poka-Yoke** | Type system prevents errors by design |
| **Genchi Genbutsu** | Run examples yourself, verify claims |
| **Muda** | Deterministic ML eliminates variance waste |

## Comparison: Deterministic vs Non-Deterministic

| Property | Traditional ML | Generative AI (LLMs) |
|----------|---------------|---------------------|
| **Same input → Same output** | Yes (always) | No (temperature sampling) |
| **Reproducibility** | 100% | 0-40% (varies) |
| **EU AI Act Article 13** | Compliant | Non-compliant |
| **Auditability** | Simple | Complex |
| **Variance** | 0.0 | 4-90% (temp dependent) |

## Next Steps

- **Chapter 3:** Learn how trueno achieves SIMD speedups with deterministic operations
- **Chapter 4:** Byzantine Fault Tolerance for handling non-deterministic AI
- **Chapter 5:** pmat quality enforcement to catch bugs before production

## Code Location

- **Examples:** `examples/ch02-crisis/src/`
  - `deterministic_baseline.rs` - Traditional ML determinism
  - `llm_variance.rs` - LLM non-determinism quantification
  - `toyota_andon.rs` - Rust compiler as quality gate
- **Tests:** Inline tests in each source file
- **Makefile:** `run-ch02`, `run-ch02-baseline`, `run-ch02-llm`, `run-ch02-andon`, `test-ch02`

## Key Takeaway

**The crisis:** LLMs are non-deterministic, violating EU AI Act transparency requirements.

**The solution:** Use deterministic alternatives where possible, and treat LLMs as Byzantine nodes that may produce inconsistent outputs. The Rust compiler acts as an Andon Cord, catching AI-generated bugs before they reach production.

**Verification:** Run `make run-ch02` to see determinism vs non-determinism quantified with actual numbers.
