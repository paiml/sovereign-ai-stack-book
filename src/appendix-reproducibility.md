# Appendix B: Scientific Reproducibility

## Reproducibility Protocol

Every claim in this book is verifiable:

```bash
git clone https://github.com/nogibjj/sovereign-ai-stack-book.git
cd sovereign-ai-stack-book
make test
```

If `make test` passes, all claims are validated.

## Test Environment Documentation

All benchmarks include:
- Hardware specifications
- Software versions
- Date measured
- Variance tolerance (±5%)

Example from Chapter 3:
```
Test Environment:
- CPU: AMD Ryzen 9 5950X
- RAM: 64GB DDR4-3200
- Rust: 1.75.0
- trueno: 0.1.0
- Date: 2025-11-23
```
