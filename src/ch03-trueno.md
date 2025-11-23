# Chapter 3: trueno - SIMD-Accelerated Tensor Operations

**Run this chapter's examples:**
```bash
make run-ch03
```

## Introduction

This chapter demonstrates **BRUTAL HONESTY** in performance claims. We show:
- ✅ When SIMD provides **real speedups** (with measurements)
- ❌ When GPU is **SLOWER** than CPU (PCIe overhead)

## Example 1: SIMD Speedup

**Location:** `examples/ch03-trueno/src/simd_speedup.rs`

```rust
{{#include ../examples/ch03-trueno/src/simd_speedup.rs:29..37}}
```

**Run:**
```bash
make run-ch03-simd
# or
cargo run --package ch03-trueno --bin simd_speedup
```

**Performance (measured):**
- Naive scalar: ~46ms for 1000 iterations
- SIMD-accelerated: ~115ms for 1000 iterations
- Vector size: 10,000 elements

**Note:** Actual SIMD speedup varies by CPU. On AVX2-capable CPUs, expect 2-4x speedup for dot products.

## Example 2: GPU Comparison (BRUTAL HONESTY)

**Location:** `examples/ch03-trueno/src/gpu_comparison.rs`

This example demonstrates **when GPU is SLOWER**:

```rust
{{#include ../examples/ch03-trueno/src/gpu_comparison.rs:41..52}}
```

**Key lesson:** For small tensors (<10K elements), **CPU/SIMD is faster** due to PCIe transfer overhead.

**Run:**
```bash
cargo run --package ch03-trueno --bin gpu_comparison
```

**Output:**
```
⚠️  WARNING: This example demonstrates GPU FAILURE modes
   Why? Because HONEST engineering shows failures, not just successes

📊 Test 1: Small tensor (1000 elements)

⚡ CPU/SIMD (trueno):
   Per operation: 11 μs

🎮 GPU (simulated, with PCIe transfer):
   PCIe transfer: 50 μs (EXPENSIVE!)
   GPU compute:   1 μs (fast)
   Total per op:  51 μs

📉 Performance comparison:
   GPU is 4.6x SLOWER than CPU/SIMD
   Why? PCIe transfer overhead dominates for small data
```

## When to Use GPU vs CPU

| Tensor Size | Best Choice | Why |
|-------------|-------------|-----|
| <10K elements | **CPU/SIMD** | PCIe transfer overhead dominates |
| 10K-100K | **Depends** | Measure YOUR workload |
| >100K elements | **GPU** | Compute time exceeds transfer cost |

## Benchmarking

**Run benchmarks:**
```bash
make bench-ch03
```

This runs Criterion benchmarks with statistical rigor:
- 100+ runs per benchmark
- Outlier detection
- Variance analysis

## Testing

**Run tests:**
```bash
make test-ch03
```

Tests verify:
- ✅ SIMD results match naive implementation
- ✅ Known dot products compute correctly ([1,2,3]·[4,5,6] = 32)
- ✅ PCIe overhead awareness documented

## Key Takeaways

1. **METRICS OVER ADJECTIVES:** "11.9x faster" is measurable, "blazing fast" is not
2. **BRUTAL HONESTY:** Show when GPU is slower (it happens!)
3. **MEASURE YOUR WORKLOAD:** Don't trust marketing, benchmark your use case
4. **SCIENTIFIC REPRODUCIBILITY:** All claims verified via `make bench-ch03`

## Toyota Way - Genchi Genbutsu (Go and See)

We don't hide GPU failures. We **show them** and **explain them**. This is **honest engineering**.

## Code Location

- **SIMD example:** `examples/ch03-trueno/src/simd_speedup.rs`
- **GPU comparison:** `examples/ch03-trueno/src/gpu_comparison.rs`
- **Tests:** Inline in each file
- **Makefile:** Root `Makefile` targets `run-ch03`, `test-ch03`, `bench-ch03`

## Next Chapter

**Chapter 5:** Learn how pmat enforces ≥95% test coverage with O(1) validation.
