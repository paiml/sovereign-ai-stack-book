# Chapter 1: Hello Sovereign AI

**Run this chapter's example:**
```bash
make run-ch01
```

## Introduction

This chapter demonstrates the **core principle** of sovereign AI: **complete local control** with **zero external dependencies**.

## What is Sovereign AI?

Sovereign AI systems are:

1. **Locally Executed** - No cloud dependencies
2. **Fully Controlled** - You own the data and computation
3. **Transparent** - All operations are visible and auditable
4. **EU Compliant** - GDPR and AI Act by design

## The Example: `hello_sovereign.rs`

**Location:** `examples/ch01-intro/src/hello_sovereign.rs`

```rust
{{#include ../examples/ch01-intro/src/hello_sovereign.rs}}
```

## Running the Example

```bash
# Method 1: Via Makefile
make run-ch01

# Method 2: Directly via cargo
cargo run --package ch01-intro --bin hello_sovereign
```

**Expected output:**
```
🇪🇺 Sovereign AI Stack - Chapter 1: Hello Sovereign AI

📊 Created local tensor: [1.0, 2.0, 3.0, 4.0, 5.0]
📈 Local computation results:
   Sum:  15.00
   Mean: 3.00

✅ Sovereign AI principles demonstrated:
   ✓ Zero network calls
   ✓ Full data control
   ✓ Transparent operations
   ✓ Deterministic results

🇪🇺 EU AI Act compliance:
   ✓ Data minimization (Article 13)
   ✓ Transparency (Article 13)
   ✓ Local processing (data residency)
```

## Key Principles Demonstrated

### 1. Zero Network Calls

The example creates a tensor and performs computations **entirely locally**. You can verify this with `strace`:

```bash
strace -e trace=network cargo run --package ch01-intro --bin hello_sovereign 2>&1 | grep -E "socket|connect|send|recv" || echo "No network calls detected!"
```

### 2. Deterministic Results

Run the example multiple times:

```bash
for i in {1..5}; do cargo run --package ch01-intro --bin hello_sovereign | grep "Mean:"; done
```

**Output (identical every time):**
```
   Mean: 3.00
   Mean: 3.00
   Mean: 3.00
   Mean: 3.00
   Mean: 3.00
```

### 3. EU AI Act Compliance

The example demonstrates compliance with:

- **Article 13 (Transparency):** All operations are documented and visible
- **Article 13 (Data Minimization):** Only uses necessary data (5 elements)
- **Data Residency:** All data stays on local machine (no cloud transfer)

## Testing

**Run tests:**
```bash
make test-ch01
```

**Tests validate:**
- ✅ Local tensor creation works
- ✅ Computations are deterministic
- ✅ No network dependencies (verified at compile time)

## Comparison: Sovereign vs Cloud AI

| Feature | Cloud AI | Sovereign AI (This Book) |
|---------|----------|--------------------------|
| **Data Location** | Cloud servers | Your machine |
| **Network Calls** | Required | Zero |
| **Latency** | 50-200ms (network) | <1ms (local) |
| **Privacy** | Data leaves your control | Data never leaves |
| **EU Compliance** | Complex (GDPR transfers) | Built-in (local only) |
| **Determinism** | No (LLM variance) | Yes (pure computation) |

## Next Steps

- **Chapter 3:** Learn how trueno achieves **11.9x speedup** with SIMD
- **Chapter 5:** Understand pmat's **≥95% coverage** enforcement
- **Chapter 12:** Build complete ML pipelines with aprender

## Code Location

- **Example:** `examples/ch01-intro/src/hello_sovereign.rs`
- **Tests:** `examples/ch01-intro/src/hello_sovereign.rs` (inline tests)
- **Makefile:** See root `Makefile` for `run-ch01` and `test-ch01` targets

## Key Takeaway

Sovereign AI is **local-first, privacy-preserving, and EU-compliant by design**. The `hello_sovereign.rs` example proves this with working code.

**Verification:** If `make run-ch01` works on your machine, you've just run a sovereign AI computation.
