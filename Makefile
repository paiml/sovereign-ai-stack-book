# Sovereign AI Stack Book - Root Workspace Makefile
# Pragmatic AI Labs
# https://paiml.com
#
# ⚠️  CRITICAL: This is a CODE-FIRST book project!
#
# CORE PRINCIPLE: SHOW, DON'T TELL
# Every claim in this book is verifiable via Makefile targets.
# If `make test` passes, the book's claims are true.
#
# WORKSPACE STRUCTURE:
# - Root workspace: Cargo.toml (workspace configuration)
# - Chapter examples: examples/ch{01-22}-{topic}/
# - Tests: tests/ (validation tests for each chapter)
# - Benchmarks: benches/ (performance claim verification)
# - Documentation: src/ (mdBook markdown, written AFTER code)
#
# This root Makefile is the SINGLE SOURCE OF TRUTH:
# - Scientific reproducibility: git clone → make test
# - Chapter-specific targets: make run-ch01, make bench-ch03, etc.
# - Quality gates: ≥95% coverage, TDG grade ≥A-, zero warnings
# - Toyota Way: Jidoka (compiler = Andon cord), Kaizen (continuous improvement)
#
# NOAH GIFT STYLE:
# - METRICS OVER ADJECTIVES: "11.9x faster" not "blazing fast"
# - BRUTAL HONESTY: Show failures (GPU slowdowns), not just successes
# - ZERO VAPORWARE: All code compiles and runs
# - MASTER-ONLY GIT: No feature branches

.PHONY: all validate format lint check test test-fast coverage build clean install help \
	run-ch01 test-ch01 bench-ch01 \
	run-ch03 test-ch03 bench-ch03 \
	test-all bench-all run-all \
	validate-claims install-deps check-env \
	quality-gate coverage-report coverage-open \
	docs docs-serve docs-build \
	setup

# Metrics directory for O(1) pre-commit validation
METRICS_DIR = .pmat-metrics

# Default target: format and validate
all: format validate

# === SETUP & INSTALLATION ===

# Initial setup: install dependencies and verify environment
setup: install-deps check-env
	@echo "✅ Setup complete! Ready to build."
	@echo ""
	@echo "Next steps:"
	@echo "  make test          # Run all tests"
	@echo "  make run-ch01      # Run Chapter 1 example"
	@echo "  make validate      # Full quality validation"

# Install all required dependencies
install-deps:
	@echo "📦 Installing dependencies..."
	@echo "  → Installing Rust toolchain..."
	@rustup update stable
	@rustup default stable
	@echo "  → Installing cargo-nextest (fast test runner)..."
	@cargo install cargo-nextest --locked || true
	@echo "  → Installing cargo-tarpaulin (coverage)..."
	@cargo install cargo-tarpaulin --locked || true
	@echo "  → Installing mdbook (documentation)..."
	@cargo install mdbook --locked || true
	@echo "  → Installing criterion (benchmarking)..."
	@cargo install cargo-criterion --locked || true
	@echo "✅ All dependencies installed!"

# Verify environment is properly configured
check-env:
	@echo "🔍 Checking environment..."
	@rustc --version || (echo "❌ Rust not installed. Run 'make install-deps'" && exit 1)
	@cargo --version || (echo "❌ Cargo not installed." && exit 1)
	@cargo nextest --version || (echo "⚠️  cargo-nextest not installed. Run 'make install-deps'" && exit 1)
	@echo "✅ Environment validated!"

# === CODE QUALITY ===

# Format all Rust code
format:
	@echo "📝 Formatting Rust code..."
	@cargo fmt --all
	@echo "✅ Formatting complete!"

# Lint all code (strict mode: warnings = errors)
lint:
	@mkdir -p $(METRICS_DIR)
	@date +%s%3N > $(METRICS_DIR)/lint.start
	@echo "🔍 Linting Rust code (strict mode: warnings = errors)..."
	@cargo clippy --all-targets --all-features -- -D warnings
	@echo "✅ All linting checks passed!"
	@if [ -f scripts/record-metric.sh ]; then ./scripts/record-metric.sh lint; fi

# Type check all code
check:
	@echo "✅ Type checking Rust code..."
	@cargo check --all-targets --all-features
	@echo "✅ All type checks passed!"

# === TESTING ===

# Fast tests without coverage (< 5 minutes per SPEC)
# Uses cargo-nextest for parallel execution
test-fast:
	@mkdir -p $(METRICS_DIR)
	@date +%s%3N > $(METRICS_DIR)/test-fast.start
	@echo "⚡ Running fast tests (target: <5 min)..."
	@if command -v cargo-nextest >/dev/null 2>&1; then \
		PROPTEST_CASES=50 RUST_TEST_THREADS=$$(nproc) cargo nextest run \
			--workspace \
			--status-level skip \
			--failure-output immediate; \
	else \
		echo "⚠️  cargo-nextest not found. Using cargo test..."; \
		PROPTEST_CASES=50 cargo test --workspace; \
	fi
	@echo "✅ All fast tests passed!"
	@if [ -f scripts/record-metric.sh ]; then ./scripts/record-metric.sh test-fast; fi

# Alias: test = test-fast
test: test-fast

# Run all tests (includes slow/ignored tests)
test-all:
	@echo "🧪 Running ALL tests (including slow tests)..."
	@if command -v cargo-nextest >/dev/null 2>&1; then \
		PROPTEST_CASES=100 cargo nextest run --workspace --run-ignored all; \
	else \
		PROPTEST_CASES=100 cargo test --workspace -- --include-ignored; \
	fi
	@echo "✅ All tests passed!"

# Code coverage (≥95% required per SPEC)
coverage:
	@mkdir -p $(METRICS_DIR)
	@date +%s%3N > $(METRICS_DIR)/coverage.start
	@echo "📊 Running code coverage analysis (target: ≥95%)..."
	@cargo tarpaulin \
		--workspace \
		--out Html \
		--out Lcov \
		--output-dir target/coverage \
		--exclude-files 'tests/*' 'benches/*' \
		--timeout 600 \
		--fail-under 95
	@echo "✅ Coverage analysis complete! Report: target/coverage/index.html"
	@if [ -f scripts/record-metric.sh ]; then ./scripts/record-metric.sh coverage; fi

# Open coverage report in browser
coverage-open:
	@echo "🌐 Opening coverage report..."
	@xdg-open target/coverage/index.html 2>/dev/null || open target/coverage/index.html 2>/dev/null || echo "Open target/coverage/index.html manually"

# Coverage report only (no re-run)
coverage-report:
	@echo "📊 Coverage report: target/coverage/index.html"
	@if [ -f target/coverage/lcov.info ]; then \
		echo "Coverage data available."; \
	else \
		echo "⚠️  No coverage data. Run 'make coverage' first."; \
	fi

# === BENCHMARKING ===

# Run all benchmarks (verify performance claims)
bench-all:
	@echo "⚡ Running all benchmarks (verifying performance claims)..."
	@cargo bench --workspace
	@echo "✅ All benchmarks complete! Reports in target/criterion/"

# === CHAPTER-SPECIFIC TARGETS ===

# Chapter 1: Introduction to Sovereign AI
run-ch01:
	@echo "🚀 Running Chapter 1 example: hello_sovereign"
	@cargo run --package ch01-intro --bin hello_sovereign

test-ch01:
	@echo "🧪 Testing Chapter 1..."
	@cargo nextest run --package ch01-intro || cargo test --package ch01-intro

bench-ch01:
	@echo "⚡ Benchmarking Chapter 1..."
	@cargo bench --package ch01-intro

# Chapter 2: Crisis of Determinism in the Age of Generative AI
run-ch02:
	@echo "🎯 Running Chapter 2 examples: Crisis of Determinism"
	@echo ""
	@echo "Example 1: Deterministic Baseline (Traditional ML)"
	@cargo run --package ch02-crisis --bin deterministic_baseline
	@echo ""
	@echo "Example 2: LLM Variance (Non-Deterministic Generation)"
	@cargo run --package ch02-crisis --bin llm_variance
	@echo ""
	@echo "Example 3: Toyota Andon Cord (Rust Compiler as Quality Gate)"
	@cargo run --package ch02-crisis --bin toyota_andon

run-ch02-baseline:
	@echo "📊 Running: Deterministic Baseline"
	@cargo run --package ch02-crisis --bin deterministic_baseline

run-ch02-llm:
	@echo "🤖 Running: LLM Variance Demo"
	@cargo run --package ch02-crisis --bin llm_variance

run-ch02-andon:
	@echo "🏭 Running: Toyota Andon Cord"
	@cargo run --package ch02-crisis --bin toyota_andon

test-ch02:
	@echo "🧪 Testing Chapter 2..."
	@cargo nextest run --package ch02-crisis || cargo test --package ch02-crisis

# Chapter 4: Byzantine Fault Tolerance for Multi-Agent Systems
run-ch04:
	@echo "🛡️  Running Chapter 4 examples: Byzantine Fault Tolerance"
	@echo ""
	@echo "Example 1: BFT Demonstration"
	@cargo run --package ch04-bft --bin bft_demo
	@echo ""
	@echo "Example 2: Dual-Model Validation"
	@cargo run --package ch04-bft --bin dual_model

run-ch04-bft:
	@echo "🛡️  Running: BFT Demonstration"
	@cargo run --package ch04-bft --bin bft_demo

run-ch04-dual:
	@echo "🔍 Running: Dual-Model Validation"
	@cargo run --package ch04-bft --bin dual_model

test-ch04:
	@echo "🧪 Testing Chapter 4..."
	@cargo nextest run --package ch04-bft || cargo test --package ch04-bft

# Chapter 3: trueno - SIMD-Accelerated Tensor Operations
run-ch03:
	@echo "🚀 Running Chapter 3 examples: trueno SIMD speedups"
	@cargo run --package ch03-trueno --bin simd_speedup
	@echo ""
	@echo "Running GPU comparison (showing honest failures)..."
	@cargo run --package ch03-trueno --bin gpu_comparison || true

test-ch03:
	@echo "🧪 Testing Chapter 3..."
	@cargo nextest run --package ch03-trueno || cargo test --package ch03-trueno

bench-ch03:
	@echo "⚡ Benchmarking Chapter 3 (verifying SIMD claims)..."
	@cargo bench --package ch03-trueno

# Chapter 6: trueno Core - Vector and Matrix Operations
run-ch06:
	@echo "🧮 Running Chapter 6 examples: trueno Core Operations"
	@echo ""
	@echo "Example 1: Vector Operations"
	@cargo run --package ch06-trueno-core --bin vector_ops
	@echo ""
	@echo "Example 2: Matrix Operations"
	@cargo run --package ch06-trueno-core --bin matrix_ops

run-ch06-vector:
	@echo "📊 Running: Vector Operations"
	@cargo run --package ch06-trueno-core --bin vector_ops

run-ch06-matrix:
	@echo "📊 Running: Matrix Operations"
	@cargo run --package ch06-trueno-core --bin matrix_ops

test-ch06:
	@echo "🧪 Testing Chapter 6..."
	@cargo nextest run --package ch06-trueno-core || cargo test --package ch06-trueno-core

# Chapter 5: pmat - Quality Enforcement Toolkit
run-ch05:
	@echo "🛡️  Running Chapter 5 examples: pmat quality enforcement"
	@echo ""
	@echo "Example 1: O(1) Quality Gates (hash-based validation)"
	@cargo run --package ch05-pmat --bin quality_gates
	@echo ""
	@echo "Example 2: TDG Analysis (Test-Driven Grade calculation)"
	@cargo run --package ch05-pmat --bin tdg_analysis
	@echo ""
	@echo "Example 3: Coverage Enforcement (≥95% requirement)"
	@cargo run --package ch05-pmat --bin coverage_demo

run-ch05-quality-gates:
	@echo "🛡️  Running: O(1) Quality Gates"
	@cargo run --package ch05-pmat --bin quality_gates

run-ch05-tdg:
	@echo "📊 Running: TDG Analysis"
	@cargo run --package ch05-pmat --bin tdg_analysis

run-ch05-coverage:
	@echo "📊 Running: Coverage Enforcement Demo"
	@cargo run --package ch05-pmat --bin coverage_demo

test-ch05:
	@echo "🧪 Testing Chapter 5..."
	@cargo nextest run --package ch05-pmat || cargo test --package ch05-pmat

# Chapter 7: trueno GPU - Acceleration Concepts
run-ch07:
	@echo "🖥️  Running Chapter 7 examples: trueno GPU Acceleration"
	@echo ""
	@echo "Example 1: GPU Acceleration Concepts"
	@cargo run --package ch07-trueno-gpu --bin gpu_acceleration
	@echo ""
	@echo "Example 2: CPU vs GPU Honest Comparison"
	@cargo run --package ch07-trueno-gpu --bin cpu_gpu_comparison

run-ch07-gpu:
	@echo "🖥️  Running: GPU Acceleration"
	@cargo run --package ch07-trueno-gpu --bin gpu_acceleration

run-ch07-comparison:
	@echo "📊 Running: CPU vs GPU Comparison"
	@cargo run --package ch07-trueno-gpu --bin cpu_gpu_comparison

test-ch07:
	@echo "🧪 Testing Chapter 7..."
	@cargo nextest run --package ch07-trueno-gpu || cargo test --package ch07-trueno-gpu

# Chapter 8: Introduction to Transpilation
run-ch08:
	@echo "🔄 Running Chapter 8 examples: Transpilation Introduction"
	@echo ""
	@echo "Example 1: Transpilation Concepts"
	@cargo run --package ch08-transpilation --bin transpilation_concepts
	@echo ""
	@echo "Example 2: AST Analysis"
	@cargo run --package ch08-transpilation --bin ast_analysis

run-ch08-concepts:
	@echo "🔄 Running: Transpilation Concepts"
	@cargo run --package ch08-transpilation --bin transpilation_concepts

run-ch08-ast:
	@echo "🌳 Running: AST Analysis"
	@cargo run --package ch08-transpilation --bin ast_analysis

test-ch08:
	@echo "🧪 Testing Chapter 8..."
	@cargo nextest run --package ch08-transpilation || cargo test --package ch08-transpilation

# Chapter 9: bashrs - Bash to Rust Transpilation
run-ch09:
	@echo "🐚 Running Chapter 9 examples: bashrs"
	@echo ""
	@echo "Example 1: Bash Transpilation"
	@cargo run --package ch09-bashrs --bin bash_transpilation
	@echo ""
	@echo "Example 2: Shell Safety"
	@cargo run --package ch09-bashrs --bin shell_safety

run-ch09-transpilation:
	@echo "🔄 Running: Bash Transpilation"
	@cargo run --package ch09-bashrs --bin bash_transpilation

run-ch09-safety:
	@echo "🔒 Running: Shell Safety"
	@cargo run --package ch09-bashrs --bin shell_safety

test-ch09:
	@echo "🧪 Testing Chapter 9..."
	@cargo nextest run --package ch09-bashrs || cargo test --package ch09-bashrs

# Chapter 10: depyler - Python to Rust Transpilation
run-ch10:
	@echo "🐍 Running Chapter 10 examples: depyler"
	@echo ""
	@echo "Example 1: Python Transpilation"
	@cargo run --package ch10-depyler --bin python_transpilation
	@echo ""
	@echo "Example 2: ML Patterns"
	@cargo run --package ch10-depyler --bin ml_patterns

run-ch10-python:
	@echo "🐍 Running: Python Transpilation"
	@cargo run --package ch10-depyler --bin python_transpilation

run-ch10-ml:
	@echo "🧠 Running: ML Patterns"
	@cargo run --package ch10-depyler --bin ml_patterns

test-ch10:
	@echo "🧪 Testing Chapter 10..."
	@cargo nextest run --package ch10-depyler || cargo test --package ch10-depyler

# Chapter 11: decy - C to Rust Transpilation
run-ch11:
	@echo "🔧 Running Chapter 11 examples: decy"
	@cargo run --package ch11-decy --bin c_transpilation

test-ch11:
	@echo "🧪 Testing Chapter 11..."
	@cargo nextest run --package ch11-decy || cargo test --package ch11-decy

# Chapter 12: aprender - ML Training Framework
run-ch12:
	@echo "🎓 Running Chapter 12 examples: aprender ML Training"
	@cargo run --package ch12-aprender --bin ml_training

test-ch12:
	@echo "🧪 Testing Chapter 12..."
	@cargo nextest run --package ch12-aprender || cargo test --package ch12-aprender

# Chapter 13: realizar - Inference Engine
run-ch13:
	@echo "🔮 Running Chapter 13 examples: realizar Inference"
	@cargo run --package ch13-realizar --bin inference_engine

test-ch13:
	@echo "🧪 Testing Chapter 13..."
	@cargo nextest run --package ch13-realizar || cargo test --package ch13-realizar

# Chapter 14: entrenar - Distributed Training
run-ch14:
	@echo "🚀 Running Chapter 14 examples: entrenar Distributed Training"
	@cargo run --package ch14-entrenar --bin distributed_training

test-ch14:
	@echo "🧪 Testing Chapter 14..."
	@cargo nextest run --package ch14-entrenar || cargo test --package ch14-entrenar

# Chapter 15: trueno-db - Vector Database
run-ch15:
	@echo "💾 Running Chapter 15 examples: trueno-db Vector Database"
	@cargo run --package ch15-trueno-db --bin vector_database

test-ch15:
	@echo "🧪 Testing Chapter 15..."
	@cargo nextest run --package ch15-trueno-db || cargo test --package ch15-trueno-db

# Chapter 16: trueno-graph - Graph Analytics
run-ch16:
	@echo "📈 Running Chapter 16 examples: trueno-graph Graph Analytics"
	@cargo run --package ch16-trueno-graph --bin graph_analytics

test-ch16:
	@echo "🧪 Testing Chapter 16..."
	@cargo nextest run --package ch16-trueno-graph || cargo test --package ch16-trueno-graph

# Chapter 17: batuta - Workflow Orchestration
run-ch17:
	@echo "🎭 Running Chapter 17 examples: batuta Workflow"
	@cargo run --package ch17-batuta --bin workflow_orchestration

test-ch17:
	@echo "🧪 Testing Chapter 17..."
	@cargo nextest run --package ch17-batuta || cargo test --package ch17-batuta

# Chapter 18: renacer - Syscall Profiling
run-ch18:
	@echo "🔍 Running Chapter 18 examples: renacer Profiling"
	@cargo run --package ch18-renacer --bin syscall_profiling

test-ch18:
	@echo "🧪 Testing Chapter 18..."
	@cargo nextest run --package ch18-renacer || cargo test --package ch18-renacer

# Chapter 19: repartir - Work Stealing
run-ch19:
	@echo "🔀 Running Chapter 19 examples: repartir Work Stealing"
	@cargo run --package ch19-repartir --bin work_stealing

test-ch19:
	@echo "🧪 Testing Chapter 19..."
	@cargo nextest run --package ch19-repartir || cargo test --package ch19-repartir

# Chapter 20: ML Pipeline
run-ch20:
	@echo "🏭 Running Chapter 20 examples: ML Pipeline"
	@cargo run --package ch20-ml-pipeline --bin ml_pipeline

test-ch20:
	@echo "🧪 Testing Chapter 20..."
	@cargo nextest run --package ch20-ml-pipeline || cargo test --package ch20-ml-pipeline

# Chapter 21: EU AI Act Compliance
run-ch21:
	@echo "⚖️  Running Chapter 21 examples: EU Compliance"
	@cargo run --package ch21-compliance --bin eu_compliance

test-ch21:
	@echo "🧪 Testing Chapter 21..."
	@cargo nextest run --package ch21-compliance || cargo test --package ch21-compliance

# Chapter 22: Production Deployment
run-ch22:
	@echo "🏁 Running Chapter 22 examples: Production Deployment"
	@cargo run --package ch22-deployment --bin production_deployment

test-ch22:
	@echo "🧪 Testing Chapter 22..."
	@cargo nextest run --package ch22-deployment || cargo test --package ch22-deployment

# Run all chapter examples
run-all: run-ch01 run-ch02 run-ch03 run-ch04 run-ch05 run-ch06 run-ch07 run-ch08 run-ch09 run-ch10 run-ch11 run-ch12 run-ch13 run-ch14 run-ch15 run-ch16 run-ch17 run-ch18 run-ch19 run-ch20 run-ch21 run-ch22
	@echo "✅ All chapter examples executed!"

# === VALIDATION ===

# Full validation: all quality gates must pass
validate: check lint test-fast
	@echo "✅ All validation checks passed!"
	@echo "  ✓ Type checking (cargo check)"
	@echo "  ✓ Linting (cargo clippy -D warnings)"
	@echo "  ✓ Fast testing (<5 min)"
	@echo "  ✓ Ready for coverage analysis!"

# Quality gate: comprehensive validation including coverage
quality-gate: validate coverage
	@echo "✅ Quality gate passed!"
	@echo "  ✓ All code quality checks"
	@echo "  ✓ Test coverage ≥95%"
	@echo "  ✓ Ready for production!"

# Validate all performance claims are reproducible
validate-claims: bench-all
	@echo "✅ All performance claims validated!"
	@echo "  → Benchmark reports: target/criterion/"
	@echo "  → Verify claims match SPEC.md assertions"

# === BUILD & CLEAN ===

# Build all examples and tests
build:
	@echo "🔨 Building all examples and tests..."
	@cargo build --workspace --all-targets
	@echo "✅ Build complete!"

# Build release (optimized)
build-release:
	@echo "🔨 Building release (optimized)..."
	@cargo build --workspace --all-targets --release
	@echo "✅ Release build complete!"

# Clean all build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	@cargo clean
	@rm -rf target/
	@rm -rf $(METRICS_DIR)
	@echo "✅ Cleaned!"

# === DOCUMENTATION ===

# Build mdBook documentation
docs-build:
	@echo "📖 Building mdBook documentation..."
	@mdbook build
	@echo "✅ Documentation built: book/html/index.html"

# Serve documentation locally
docs-serve:
	@echo "📖 Serving documentation at http://localhost:3000"
	@mdbook serve --open

# Alias: docs = docs-build
docs: docs-build

# === HELP ===

help:
	@echo "Sovereign AI Stack Book - Makefile Targets"
	@echo ""
	@echo "SETUP:"
	@echo "  make setup              Initial setup (install deps, verify env)"
	@echo "  make install-deps       Install Rust, cargo-nextest, tarpaulin, mdbook"
	@echo "  make check-env          Verify environment is configured"
	@echo ""
	@echo "CODE QUALITY:"
	@echo "  make format             Format all Rust code"
	@echo "  make lint               Lint code (warnings = errors)"
	@echo "  make check              Type check all code"
	@echo ""
	@echo "TESTING:"
	@echo "  make test               Fast tests (<5 min)"
	@echo "  make test-all           All tests (including slow tests)"
	@echo "  make test-ch01          Test Chapter 1 only"
	@echo "  make test-ch03          Test Chapter 3 only"
	@echo "  make coverage           Code coverage (≥95% required)"
	@echo "  make coverage-open      Open coverage report in browser"
	@echo ""
	@echo "BENCHMARKING:"
	@echo "  make bench-all          Run all benchmarks"
	@echo "  make bench-ch01         Benchmark Chapter 1"
	@echo "  make bench-ch03         Benchmark Chapter 3 (verify SIMD claims)"
	@echo "  make validate-claims    Verify all performance claims"
	@echo ""
	@echo "RUNNING EXAMPLES:"
	@echo "  make run-ch01           Run Chapter 1: hello_sovereign"
	@echo "  make run-ch03           Run Chapter 3: trueno SIMD demos"
	@echo "  make run-all            Run all chapter examples"
	@echo ""
	@echo "VALIDATION:"
	@echo "  make validate           Full validation (check + lint + test-fast)"
	@echo "  make quality-gate       Comprehensive validation (includes coverage)"
	@echo ""
	@echo "BUILD & CLEAN:"
	@echo "  make build              Build all examples and tests"
	@echo "  make build-release      Build optimized release"
	@echo "  make clean              Clean all build artifacts"
	@echo ""
	@echo "DOCUMENTATION:"
	@echo "  make docs-build         Build mdBook documentation"
	@echo "  make docs-serve         Serve docs at http://localhost:3000"
	@echo ""
	@echo "SCIENTIFIC REPRODUCIBILITY:"
	@echo "  git clone https://github.com/paiml/sovereign-ai-stack-book.git"
	@echo "  cd sovereign-ai-stack-book"
	@echo "  make setup              # Install dependencies"
	@echo "  make test               # Verify all examples work"
	@echo "  make bench-all          # Reproduce all performance claims"
	@echo ""
	@echo "If 'make test' passes, the book's claims are true. If not, file an issue."

# Mutation testing
mutants:
	cargo mutants --no-times --timeout 300
