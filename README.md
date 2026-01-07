<div align="center">

<p align="center">
  <img src=".github/sovereign-ai-stack-book-hero.svg" alt="Sovereign AI Stack Book" width="800">
</p>

<h1 align="center">Sovereign AI Stack Book</h1>

<p align="center">
  <b>EXTREME TDD guide to building EU-compliant AI systems in pure Rust — every claim scientifically reproducible</b>
</p>

<p align="center">
  <a href="https://github.com/paiml/sovereign-ai-stack-book/actions/workflows/ci.yml"><img src="https://github.com/paiml/sovereign-ai-stack-book/actions/workflows/ci.yml/badge.svg" alt="CI"></a>
  <a href="https://paiml.github.io/sovereign-ai-stack-book/"><img src="https://img.shields.io/badge/📚_book-online-brightgreen" alt="Book"></a>
  <a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License"></a>
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/rust-1.75%2B-orange.svg" alt="Rust"></a>
  <a href="https://rust-lang.github.io/mdBook/"><img src="https://img.shields.io/badge/mdBook-0.4-blue.svg" alt="mdBook"></a>
  <a href="https://github.com/paiml/sovereign-ai-stack-book/pulls"><img src="https://img.shields.io/badge/PRs-welcome-brightgreen.svg" alt="PRs Welcome"></a>
  <a href="https://crates.io/crates/pmat"><img src="https://img.shields.io/badge/pmat-quality-green" alt="PMAT"></a>
</p>

</div>

---

![Book Architecture](.github/book-architecture.svg)

## Table of Contents

- [Overview](#overview)
- [Quick Start](#quick-start)
- [Demo](#demo)
- [Installation](#installation)
- [Book Structure](#book-structure)
- [Stack Components](#stack-components)
- [Quality Standards](#quality-standards)
- [Design Principles](#design-principles)
- [Development](#development)
- [Contributing](#contributing)
- [License](#license)

## Overview

This book documents the **Sovereign AI Stack**, a complete pure-Rust ecosystem for organizations requiring full control over their ML infrastructure. Every claim is verified through working code examples that readers can execute locally.

### Key Capabilities

- **Scientific Reproducibility**: `git clone` → `make test` → all claims verified
- **EXTREME TDD**: 95%+ coverage, A- TDG grade, 80%+ mutation score
- **EU AI Act Compliance**: Articles 10, 13, 15 addressed with auditable code
- **Zero Vaporware**: Every example compiles, tests pass, benchmarks run
- **Brutal Honesty**: Shows failures (GPU 65x slower) not just successes

## Quick Start

```bash
# Clone and verify
git clone https://github.com/paiml/sovereign-ai-stack-book.git
cd sovereign-ai-stack-book

# Scientific reproducibility protocol
make test              # All examples compile and pass
make bench             # Reproduce performance claims
make validate          # Coverage, TDG, clippy checks

# Read the book
mdbook serve --open    # http://localhost:3000
```

**If `make test` passes, the book's claims are true. If not, [file an issue](https://github.com/paiml/sovereign-ai-stack-book/issues).**

## Demo

[![asciicast](https://asciinema.org/a/demo.svg)](https://paiml.github.io/sovereign-ai-stack-book/)

**Run a chapter example:**

```bash
cd examples/ch03-trueno
cargo run --release
```

**Example Output (SIMD Tensor Operations):**

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  🚀 Trueno SIMD Benchmark
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  Operation          Size       Time       Speedup
  ─────────────────────────────────────────────────
  Vector Add         1M elem    0.42ms     12.3x
  Matrix Mul         1K×1K      2.31ms     8.7x
  Dot Product        10M elem   1.87ms     15.2x
  ─────────────────────────────────────────────────

  ✅ All benchmarks passed
  📊 Average speedup vs scalar: 12.1x
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

**Run all chapter tests:**

```bash
make test-chapters    # Tests all 23 chapters
```

## Installation

### Prerequisites

- **Rust 1.75+**: Install via [rustup](https://rustup.rs/)
- **cargo-nextest**: Fast test runner (optional but recommended)
- **mdBook**: For building/viewing the book locally

### Setup

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install dependencies
make setup

# Or install manually:
cargo install cargo-nextest --locked
cargo install mdbook --locked
cargo install cargo-tarpaulin --locked  # Optional: coverage
```

### Verify Installation

```bash
# Check Rust version
rustc --version  # Should be 1.75.0 or higher

# Verify workspace builds
cargo check --workspace

# Run tests
make test
```

### Platform Support

| Platform | Status | Notes |
|----------|--------|-------|
| Linux x86_64 | ✅ Full | Primary development platform |
| macOS ARM64 | ✅ Full | M1/M2/M3 supported |
| macOS x86_64 | ✅ Full | Intel Macs supported |
| Windows x86_64 | ✅ Full | MSVC toolchain required |

## Book Structure

The book covers 23 chapters across 6 parts:

```
┌─────────────────────────────────────────────────────────────┐
│              Part 0: Crisis and Response                    │
│        Ch 1-4: Sovereign AI | Determinism | trueno | BFT    │
├─────────────────────────────────────────────────────────────┤
│            Part I: Infrastructure Foundations               │
│              Ch 5-7: pmat | trueno Core | GPU               │
├─────────────────────────────────────────────────────────────┤
│              Part II: Transpilation Layer                   │
│            Ch 8-11: bashrs | depyler | decy                 │
├─────────────────────────────────────────────────────────────┤
│            Part III: Machine Learning Pipeline              │
│           Ch 12-14: aprender | realizar | entrenar          │
├─────────────────────────────────────────────────────────────┤
│             Part IV: Database and Graph                     │
│              Ch 15-16: trueno-db | trueno-graph             │
├──────────────────────────┬──────────────────────────────────┤
│   Part V: Orchestration  │      Part VI: Production         │
│  Ch 17-19: batuta |      │  Ch 20-23: ML Pipeline |         │
│  renacer | repartir      │  Compliance | Deploy | CITL      │
└──────────────────────────┴──────────────────────────────────┘
```

## Stack Components

### Core ML Stack (crates.io versions as of January 2025)

| Component | Version | Description | Chapter |
|-----------|---------|-------------|---------|
| [trueno](https://crates.io/crates/trueno) | 0.11 | SIMD/GPU compute (AVX2/AVX-512/NEON, wgpu) | Ch 3, 6-7 |
| [aprender](https://crates.io/crates/aprender) | 0.24 | ML algorithms: regression, trees, clustering | Ch 12 |
| [realizar](https://crates.io/crates/realizar) | 0.5 | Inference engine for GGUF/SafeTensors | Ch 13 |
| [entrenar](https://crates.io/crates/entrenar) | 0.5 | Training: autograd, LoRA/QLoRA | Ch 14 |
| [pacha](https://crates.io/crates/pacha) | 0.2 | Model registry with signatures | — |
| [batuta](https://crates.io/crates/batuta) | 0.4 | Stack orchestration | Ch 17 |

### Transpilers

| Component | Version | Description | Chapter |
|-----------|---------|-------------|---------|
| [bashrs](https://crates.io/crates/bashrs) | 6.51 | Shell to Rust transpiler | Ch 9 |
| [depyler](https://github.com/paiml/depyler) | — | Python to Rust transpiler | Ch 10 |
| [decy](https://github.com/paiml/decy) | — | C to Rust transpiler | Ch 11 |

### Infrastructure

| Component | Version | Description | Chapter |
|-----------|---------|-------------|---------|
| [pmat](https://crates.io/crates/pmat) | 2.213 | Quality enforcement toolkit | Ch 5 |
| [trueno-db](https://crates.io/crates/trueno-db) | 0.3 | GPU-accelerated analytics | Ch 15 |
| [trueno-graph](https://crates.io/crates/trueno-graph) | 0.1 | Graph analytics | Ch 16 |
| [trueno-rag](https://crates.io/crates/trueno-rag) | 0.1 | RAG pipeline (BM25+vector) | — |
| [trueno-viz](https://crates.io/crates/trueno-viz) | 0.1 | Terminal/PNG visualization | — |
| [renacer](https://crates.io/crates/renacer) | 0.9 | Syscall tracing | Ch 18 |
| [repartir](https://crates.io/crates/repartir) | 2.0 | Distributed compute | Ch 19 |
| [alimentar](https://crates.io/crates/alimentar) | 0.2 | Zero-copy Parquet/Arrow | — |

## Quality Standards

The book enforces production-grade quality on all examples:

| Metric | Requirement | Enforcement |
|--------|-------------|-------------|
| Test Coverage | 95%+ | `cargo tarpaulin` |
| TDG Grade | A- (90+) | `pmat tdg` |
| Mutation Score | 80%+ | `cargo mutants` |
| Warnings | Zero | `clippy -D warnings` |

```bash
# Run full quality validation
make validate
```

## Design Principles

The book applies Toyota Production System principles:

| Principle | Application |
|-----------|-------------|
| **Jidoka** | Compiler stops on defects (Rust type system) |
| **Poka-Yoke** | Tests prevent errors before deployment |
| **Genchi Genbutsu** | Benchmarks verify claims empirically |
| **Muda** | SIMD eliminates computational waste |
| **Kaizen** | Continuous quality improvement via pmat |

## Development

```bash
# Build all examples
cargo build --workspace --all-targets

# Run tests
cargo test --workspace

# Build book
mdbook build

# Serve locally
mdbook serve --open
```

## Contributing

Contributions welcome! Please follow the PAIML quality standards:

1. Fork the repository
2. Create a feature branch
3. Ensure the book builds: `mdbook build`
4. Run tests: `mdbook test`
5. Submit a pull request

## License

MIT License — see [LICENSE](LICENSE) for details.

## Links

- [Online Book](https://paiml.github.io/sovereign-ai-stack-book/)
- [GitHub Repository](https://github.com/paiml/sovereign-ai-stack-book)
- [Pragmatic AI Labs](https://paiml.com)

---

**Sovereign AI Stack Book** — CODE IS THE WAY.
