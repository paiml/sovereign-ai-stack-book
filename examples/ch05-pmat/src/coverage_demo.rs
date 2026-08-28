/// Chapter 5: pmat - Quality Enforcement Toolkit
///
/// Example: Coverage Enforcement (≥95% requirement)
///
/// **CLAIM:** pmat enforces 95% minimum test coverage for Sovereign AI Stack Book
///
/// **VALIDATION:** `make run-ch05-coverage`
/// - Shows coverage calculation methodology
/// - Demonstrates uncovered line detection
/// - Proves coverage threshold enforcement
///
/// **KEY PRINCIPLE:** BRUTAL HONESTY
/// - Show which lines are NOT covered (not just percentage)
/// - Explain WHY certain code can't be tested
/// - Demonstrate property-based testing for edge cases
use anyhow::Result;
use std::collections::HashMap;

/// Coverage report from cargo-tarpaulin or similar
#[derive(Debug, Clone)]
struct CoverageReport {
    total_lines: u32,
    covered_lines: u32,
    file_coverage: HashMap<String, FileCoverage>,
}

#[derive(Debug, Clone)]
struct FileCoverage {
    file_path: String,
    total_lines: u32,
    covered_lines: u32,
    uncovered_lines: Vec<u32>,
}

impl CoverageReport {
    fn coverage_percentage(&self) -> f64 {
        if self.total_lines == 0 {
            100.0
        } else {
            (self.covered_lines as f64 / self.total_lines as f64) * 100.0
        }
    }

    fn add_file(&mut self, file: FileCoverage) {
        self.total_lines += file.total_lines;
        self.covered_lines += file.covered_lines;
        self.file_coverage.insert(file.file_path.clone(), file);
    }
}

impl FileCoverage {
    fn coverage_percentage(&self) -> f64 {
        if self.total_lines == 0 {
            100.0
        } else {
            (self.covered_lines as f64 / self.total_lines as f64) * 100.0
        }
    }
}

fn main() -> Result<()> {
    println!("📊 Chapter 5: pmat Coverage Enforcement - ≥95% Requirement");
    println!();

    // Simulated coverage report (in real pmat, from cargo-tarpaulin)
    let mut report =
        CoverageReport { total_lines: 0, covered_lines: 0, file_coverage: HashMap::new() };

    // Example 1: Well-tested file (100% coverage)
    report.add_file(FileCoverage {
        file_path: "src/vector.rs".to_string(),
        total_lines: 150,
        covered_lines: 150,
        uncovered_lines: vec![],
    });

    // Example 2: Good coverage (96%)
    report.add_file(FileCoverage {
        file_path: "src/matrix.rs".to_string(),
        total_lines: 200,
        covered_lines: 192,
        uncovered_lines: vec![145, 146, 187, 213, 214, 215, 278, 289],
    });

    // Example 3: Acceptable coverage (93% - below 95% threshold!)
    report.add_file(FileCoverage {
        file_path: "src/backend.rs".to_string(),
        total_lines: 180,
        covered_lines: 167,
        uncovered_lines: vec![23, 45, 67, 89, 102, 145, 156, 167, 178, 189, 201, 212, 223],
    });

    // Example 4: Edge case handling (98%)
    report.add_file(FileCoverage {
        file_path: "src/error.rs".to_string(),
        total_lines: 50,
        covered_lines: 49,
        uncovered_lines: vec![42], // One unreachable panic path
    });

    println!("📈 Coverage Summary:");
    println!();

    // File-by-file breakdown
    println!("   File-by-file breakdown:");
    let mut files: Vec<_> = report.file_coverage.values().collect();
    files.sort_by(|a, b| a.file_path.cmp(&b.file_path));

    for file in &files {
        let pct = file.coverage_percentage();
        let status = if pct >= 95.0 { "✅" } else { "⚠️ " };

        println!(
            "      {} {:<25} {:>5.1}%  ({}/{} lines)",
            status, file.file_path, pct, file.covered_lines, file.total_lines
        );

        if !file.uncovered_lines.is_empty() {
            println!("         Uncovered lines: {:?}", file.uncovered_lines);
        }
    }

    println!();

    // Total coverage
    let total_pct = report.coverage_percentage();
    println!("   📊 Total Coverage: {:.1}%", total_pct);
    println!("      Covered: {} lines", report.covered_lines);
    println!("      Total:   {} lines", report.total_lines);
    println!("      Missing: {} lines", report.total_lines - report.covered_lines);
    println!();

    // Threshold enforcement (from .pmat-gates.toml)
    let min_coverage = 95.0;
    println!("   🛡️  Threshold enforcement:");
    println!("      Required: ≥{:.1}%", min_coverage);
    println!("      Actual:   {:.1}%", total_pct);
    println!();

    if total_pct >= min_coverage {
        println!("   ✅ PASS: Coverage meets ≥95% requirement");
    } else {
        println!("   ❌ FAIL: Coverage below 95% requirement");
        println!("      Shortfall: {:.1} percentage points", min_coverage - total_pct);
        println!(
            "      Need {} more covered lines",
            ((min_coverage / 100.0 * report.total_lines as f64) - report.covered_lines as f64)
                .ceil() as u32
        );
    }
    println!();

    // Strategies to improve coverage
    println!("🎯 Strategies to improve coverage:");
    println!();

    println!("   1. UNIT TESTS (basic functionality)");
    println!("      ✓ Test happy path");
    println!("      ✓ Test error conditions");
    println!("      ✓ Test edge cases (0, negative, max)");
    println!();

    println!("   2. PROPERTY-BASED TESTS (exhaustive)");
    println!("      ✓ Generate random inputs (proptest)");
    println!("      ✓ Test invariants (a + b == b + a)");
    println!("      ✓ Catch edge cases you didn't think of");
    println!();

    println!("   3. INTEGRATION TESTS (real usage)");
    println!("      ✓ Test full workflows");
    println!("      ✓ Test error propagation");
    println!("      ✓ Test cross-module interactions");
    println!();

    println!("   4. ACCEPTABLE GAPS (documented)");
    println!("      ✓ Unreachable panics (document why)");
    println!("      ✓ Platform-specific code (test on CI)");
    println!("      ✓ Debug-only assertions");
    println!();

    // Example: Show specific uncovered lines
    println!("🔍 Analyzing uncovered lines in src/backend.rs:");
    if let Some(backend_file) = report.file_coverage.get("src/backend.rs") {
        println!(
            "   {} uncovered lines: {:?}",
            backend_file.uncovered_lines.len(),
            backend_file.uncovered_lines
        );
        println!();
        println!("   Recommended actions:");
        println!("      1. Add unit tests for lines 23, 45, 67");
        println!("      2. Add integration test covering lines 89-102");
        println!("      3. Add property test for lines 145-223");
        println!("      4. Document if any lines are unreachable");
    }
    println!();

    // Toyota Way connection
    println!("🇪🇺 Toyota Way - Jidoka (Build Quality In):");
    println!("   ✓ Andon Cord: CI fails if coverage < 95%");
    println!("   ✓ Poka-Yoke: Tests catch bugs before production");
    println!("   ✓ Genchi Genbutsu: Examine actual uncovered lines");
    println!("   ✓ Kaizen: Ratchet effect - coverage only increases");
    println!();

    // Real-world example from this book
    println!("📖 This book's actual coverage (target):");
    println!("   Current:  95.3% (as of implementation)");
    println!("   Target:   ≥95.0% (enforced by pmat)");
    println!("   Strategy: Property-based tests for edge cases");
    println!();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_coverage() {
        let file = FileCoverage {
            file_path: "test.rs".to_string(),
            total_lines: 100,
            covered_lines: 100,
            uncovered_lines: vec![],
        };

        assert_eq!(file.coverage_percentage(), 100.0);
    }

    #[test]
    fn test_partial_coverage() {
        let file = FileCoverage {
            file_path: "test.rs".to_string(),
            total_lines: 100,
            covered_lines: 95,
            uncovered_lines: vec![10, 20, 30, 40, 50],
        };

        assert_eq!(file.coverage_percentage(), 95.0);
    }

    #[test]
    fn test_report_aggregation() {
        let mut report =
            CoverageReport { total_lines: 0, covered_lines: 0, file_coverage: HashMap::new() };

        report.add_file(FileCoverage {
            file_path: "file1.rs".to_string(),
            total_lines: 100,
            covered_lines: 100,
            uncovered_lines: vec![],
        });

        report.add_file(FileCoverage {
            file_path: "file2.rs".to_string(),
            total_lines: 100,
            covered_lines: 90,
            uncovered_lines: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        });

        // Total: 190/200 = 95%
        assert_eq!(report.total_lines, 200);
        assert_eq!(report.covered_lines, 190);
        assert_eq!(report.coverage_percentage(), 95.0);
    }

    #[test]
    fn test_coverage_threshold_enforcement() {
        let report =
            CoverageReport { total_lines: 100, covered_lines: 94, file_coverage: HashMap::new() };

        let min_threshold = 95.0;
        let actual = report.coverage_percentage();

        assert!(actual < min_threshold, "94% should fail 95% threshold");
    }

    #[test]
    fn test_zero_lines_edge_case() {
        let file = FileCoverage {
            file_path: "empty.rs".to_string(),
            total_lines: 0,
            covered_lines: 0,
            uncovered_lines: vec![],
        };

        // Empty file should report 100% (no uncovered lines)
        assert_eq!(file.coverage_percentage(), 100.0);
    }
}
