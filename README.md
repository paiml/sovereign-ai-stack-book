# Sovereign AI Stack Book

[![CI](https://github.com/nogibjj/sovereign-ai-stack-book/actions/workflows/ci.yml/badge.svg)](https://github.com/nogibjj/sovereign-ai-stack-book/actions/workflows/ci.yml)
[![Deploy Book](https://github.com/nogibjj/sovereign-ai-stack-book/actions/workflows/deploy-book.yml/badge.svg)](https://github.com/nogibjj/sovereign-ai-stack-book/actions/workflows/deploy-book.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**EXTREME TDD Guide to Building EU-Compliant AI Systems**

> **CODE IS THE WAY** - Every claim is scientifically reproducible via `make test`

## 📖 Read the Book

**Live Book:** https://nogibjj.github.io/sovereign-ai-stack-book/ *(auto-deploys on every push)*

## 🚀 Quick Start

```bash
# Clone the repository
git clone https://github.com/nogibjj/sovereign-ai-stack-book.git
cd sovereign-ai-stack-book

# Verify EVERYTHING works (scientific reproducibility)
make test          # All examples compile and pass (23+ tests)
make run-ch01      # Run Chapter 1: Hello Sovereign AI
make run-ch03      # Run Chapter 3: SIMD speedups
make run-ch05      # Run Chapter 5: Quality enforcement

# Run all examples
make run-all       # Execute all chapter examples

# Build the book locally
mdbook serve --open
```

**If `make test` passes, the book's claims are true. If not, [file an issue](https://github.com/nogibjj/sovereign-ai-stack-book/issues).**

## 🎯 What Makes This Book Different

### 1. METRICS OVER ADJECTIVES

❌ **Vaporware:** "Our tensor library is blazing fast!"
✅ **This book:** "trueno achieves 11.9x speedup via SIMD (see `make bench-ch03`)"

❌ **Vaporware:** "High test coverage ensures quality"
✅ **This book:** "95.3% line coverage, 82% mutation score, TDG grade A- (91.2)"

### 2. BRUTAL HONESTY

We show **failures**, not just successes:
- Chapter 3 demonstrates when GPU is **65x SLOWER** than CPU (PCIe overhead)
- Quality enforcement examples show real uncovered lines
- All benchmarks include variance and test environment specs

### 3. ZERO VAPORWARE

Every example:
- ✅ Compiles with `cargo build`
- ✅ Passes tests with `cargo test`
- ✅ Runs with `cargo run`
- ✅ Benchmarks with `cargo bench`

**No "coming soon" features. No "left as an exercise." All code works.**

### 4. SCIENTIFIC REPRODUCIBILITY

Following academic standards:
- **Test Environment Documentation:** Hardware specs, software versions, date measured
- **Statistical Rigor:** Criterion benchmarks with 100+ runs
- **Variance Tolerance:** ±5% acceptable variance documented
- **Reproducibility Protocol:** `git clone` → `make test` validates all claims

## 📚 Book Structure

### Part 0: The Crisis and The Response
- **Chapter 1:** Hello Sovereign AI ✅ *(complete)*
- **Chapter 2:** Crisis of Determinism *(in progress)*
- **Chapter 3:** trueno - SIMD Operations ✅ *(complete)*
- **Chapter 4:** Byzantine Fault Tolerance *(planned)*

### Part I: Infrastructure Foundations
- **Chapter 5:** pmat - Quality Enforcement ✅ *(complete)*
- **Chapter 6:** trueno Core *(planned)*
- **Chapter 7:** GPU Acceleration *(planned)*

### Part II-VI: Complete Toolchain
Transpilers, ML pipeline, databases, orchestration, production deployment *(22 chapters total)*

**Status:** 3 of 22 chapters complete with working code

## 🛡️ Quality Standards (EXTREME TDD)

This book enforces production-grade quality:

| Metric | Requirement | Current | Status |
|--------|-------------|---------|--------|
| **Test Coverage** | ≥95% | 95.3% | ✅ |
| **TDG Grade** | ≥A- (90) | A (91.2) | ✅ |
| **Compiler Warnings** | 0 | 0 | ✅ |
| **Tests Passing** | 100% | 23/23 | ✅ |
| **CI/CD** | All checks pass | ✅ | ✅ |

**Quality enforcement via pmat:**
- O(1) pre-commit validation (<30ms via hash-based caching)
- Automated TDG scoring
- Coverage enforcement in CI
- Zero tolerance for regressions

## 🔧 Technology Stack

### Core Dependencies
- **Rust** (stable) - Systems programming language
- **trueno** - SIMD-accelerated tensor operations
- **pmat** - Quality enforcement toolkit
- **cargo-nextest** - Fast test runner
- **mdbook** - Documentation

### Complete Sovereign AI Toolchain
- `pmat` - Quality enforcement (O(1) validation, TDG scoring)
- `trueno` - Tensor operations (SIMD/GPU)
- `trueno-db` - Vector database
- `trueno-graph` - Graph analytics
- `aprender` - ML training
- `realizar` - Inference engine
- `entrenar` - Distributed training
- `depyler` - Python→Rust transpiler
- `decy` - Deno→Rust transpiler
- `bashrs` - Bash→Rust transpiler
- `batuta` - Workflow orchestration
- `renacer` - Syscall profiling
- `repartir` - Work stealing scheduler

## 🚦 CI/CD

**Automated on every push:**
- ✅ Format checking (`cargo fmt`)
- ✅ Linting (`cargo clippy -D warnings`)
- ✅ All tests (cargo-nextest)
- ✅ All examples execute
- ✅ Book builds successfully
- ✅ Auto-deployment to GitHub Pages

**Scientific Reproducibility Check:**
- Every push validates `git clone → make test` workflow
- Ensures all claims in the book are reproducible

## 📊 Examples

### Chapter 1: Hello Sovereign AI
```rust
use trueno::Vector;

fn main() {
    // Create local tensor (no cloud, no external APIs)
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let vector = Vector::from_slice(&data);

    let sum: f32 = vector.as_slice().iter().sum();
    let mean = sum / vector.len() as f32;

    println!("Sum: {}, Mean: {}", sum, mean);
    // Output: Sum: 15, Mean: 3
}
```

**Zero network calls. Full data control. EU AI Act compliant.**

### Chapter 3: SIMD Speedup (BRUTAL HONESTY)
```bash
make run-ch03
# Shows both:
# ✅ When SIMD provides real speedups
# ❌ When GPU is SLOWER than CPU (PCIe overhead)
```

### Chapter 5: Quality Enforcement
```bash
make run-ch05-tdg
# Output: TDG Score: 91.2 (Grade: A)
# METRICS OVER ADJECTIVES
```

## 🌍 EU Regulatory Compliance

Built for EU AI Act compliance:
- **Article 13 (Transparency):** All operations documented and auditable
- **Article 13 (Data Minimization):** Only necessary data used
- **Data Residency:** All processing happens locally
- **Reproducibility:** Same input → same output (deterministic)

## 🧪 Development

```bash
# Setup development environment
make setup

# Run specific chapters
make run-ch01        # Chapter 1: Hello Sovereign AI
make run-ch03        # Chapter 3: trueno SIMD
make run-ch05        # Chapter 5: pmat quality

# Testing
make test            # Fast tests (<5 min)
make test-all        # All tests (including slow)
make coverage        # Generate coverage report (≥95%)

# Benchmarking
make bench-ch03      # Verify SIMD claims
make bench-all       # Run all benchmarks

# Validation
make validate        # Full quality validation
make quality-gate    # Comprehensive validation + coverage

# Build book
make docs-build      # Build mdBook
make docs-serve      # Serve at http://localhost:3000
```

## 🤝 Contributing

Found an issue? Example doesn't work?

1. **File an issue:** https://github.com/nogibjj/sovereign-ai-stack-book/issues
2. **Include:** Chapter number, error message, environment (`rustc --version`)
3. **Expected:** We fix it (reproducibility is our promise)

**CODE-FIRST contributions welcome:**
1. Write working code in `examples/`
2. Add tests (≥95% coverage required)
3. Update documentation
4. Ensure `make test` passes

## 📜 License

MIT License - See [LICENSE](LICENSE) for details

## 🙏 Acknowledgments

Built by [Noah Gift](https://github.com/noahgift) and the **Pragmatic AI Labs** team.

- Used in production at https://paiml.com
- Part of the Sovereign AI Stack ecosystem
- Open source: MIT/Apache-2.0 licensed

## 📖 Citation

If you use this book in your research or project:

```bibtex
@book{gift2025sovereign,
  title={Sovereign AI Stack: EXTREME TDD for EU-Compliant AI Systems},
  author={Gift, Noah},
  year={2025},
  publisher={Pragmatic AI Labs},
  url={https://github.com/nogibjj/sovereign-ai-stack-book}
}
```

## 🎯 Project Status

**Current Status:** Active development (3/22 chapters complete)

**Roadmap:** See [docs/roadmaps/roadmap.yaml](docs/roadmaps/roadmap.yaml) or run:
```bash
pmat work status
```

**Next Chapters:**
- CH02-001: Crisis of Determinism (in progress)
- CH04-001: Byzantine Fault Tolerance (planned)
- DEPYLER-001: Python→Rust Transpiler (planned)

---

**Remember:** If `make test` passes, the book's claims are true. **CODE IS THE WAY.**
