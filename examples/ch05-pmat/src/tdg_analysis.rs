/// Chapter 5: pmat - Quality Enforcement Toolkit
///
/// Example: TDG (Test-Driven Grade) Analysis
///
/// **CLAIM:** TDG provides objective quality score based on test coverage,
/// mutation score, complexity, and other metrics.
///
/// **VALIDATION:** `make run-ch05-tdg`
/// - Calculates TDG score from quality metrics
/// - Shows grade mapping (A+ = 95-100, A = 90-94, etc.)
/// - Demonstrates threshold enforcement (≥ A- = 90)
///
/// **KEY PRINCIPLE:** METRICS OVER ADJECTIVES
/// - Not "good quality" → "TDG grade: A- (91.2)"
/// - Not "well-tested" → "95.3% coverage, 82% mutation score"
/// - Not "maintainable" → "Cyclomatic complexity: 8.3 avg"

use anyhow::Result;
use std::fmt;

/// TDG grade levels (matching pmat spec)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Grade {
    F,      // < 60
    D,      // 60-69
    C,      // 70-79
    B,      // 80-84
    BMinus, // 85-89
    A,      // 90-94
    APlus,  // 95-100
}

impl Grade {
    fn from_score(score: f64) -> Self {
        match score {
            s if s >= 95.0 => Grade::APlus,
            s if s >= 90.0 => Grade::A,
            s if s >= 85.0 => Grade::BMinus,
            s if s >= 80.0 => Grade::B,
            s if s >= 70.0 => Grade::C,
            s if s >= 60.0 => Grade::D,
            _ => Grade::F,
        }
    }
}

impl fmt::Display for Grade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Grade::APlus => write!(f, "A+"),
            Grade::A => write!(f, "A"),
            Grade::BMinus => write!(f, "B+"),
            Grade::B => write!(f, "B"),
            Grade::C => write!(f, "C"),
            Grade::D => write!(f, "D"),
            Grade::F => write!(f, "F"),
        }
    }
}

/// Quality metrics from various tools
#[derive(Debug, Clone)]
struct QualityMetrics {
    // Coverage metrics
    line_coverage_pct: f64,         // From cargo-tarpaulin
    branch_coverage_pct: f64,       // From cargo-tarpaulin

    // Mutation testing
    mutation_score_pct: f64,        // From cargo-mutants

    // Code complexity
    avg_cyclomatic_complexity: f64, // From cargo-complexity or pmat
    max_cyclomatic_complexity: u32, // Worst function

    // Code quality
    clippy_warnings: u32,           // From cargo clippy
    clippy_errors: u32,             // From cargo clippy -D warnings

    // Documentation
    doc_coverage_pct: f64,          // Public API documentation
}

impl QualityMetrics {
    /// Calculate TDG score (0-100) from quality metrics
    ///
    /// Formula (matching pmat implementation):
    /// - Coverage: 40% weight (line + branch) / 2
    /// - Mutation: 30% weight
    /// - Complexity: 15% weight (inverted, lower = better)
    /// - Quality: 15% weight (zero warnings = 100%)
    fn calculate_tdg_score(&self) -> f64 {
        // Coverage component (40% weight)
        let coverage_score = (self.line_coverage_pct + self.branch_coverage_pct) / 2.0;
        let coverage_component = coverage_score * 0.40;

        // Mutation component (30% weight)
        let mutation_component = self.mutation_score_pct * 0.30;

        // Complexity component (15% weight, inverted)
        // Lower complexity = better score
        // Max acceptable complexity: 15 (from pmat config)
        let complexity_penalty = (self.avg_cyclomatic_complexity / 15.0).min(1.0);
        let complexity_component = (1.0 - complexity_penalty) * 100.0 * 0.15;

        // Quality component (15% weight)
        // Zero warnings/errors = 100%, any warnings = penalty
        let total_issues = self.clippy_warnings + self.clippy_errors;
        let quality_score = if total_issues == 0 {
            100.0
        } else {
            // Harsh penalty: each issue costs points
            (100.0 - (total_issues as f64 * 2.0)).max(0.0)
        };
        let quality_component = quality_score * 0.15;

        // Total TDG score
        coverage_component + mutation_component + complexity_component + quality_component
    }
}

fn main() -> Result<()> {
    println!("📊 Chapter 5: pmat TDG (Test-Driven Grade) Analysis");
    println!();

    // Example 1: Excellent quality (this book's target)
    println!("📈 Example 1: EXCELLENT quality (target for this book)");
    let excellent = QualityMetrics {
        line_coverage_pct: 95.5,
        branch_coverage_pct: 93.2,
        mutation_score_pct: 82.0,
        avg_cyclomatic_complexity: 8.3,
        max_cyclomatic_complexity: 12,
        clippy_warnings: 0,
        clippy_errors: 0,
        doc_coverage_pct: 98.0,
    };

    print_metrics_analysis(&excellent, "Sovereign AI Stack Book")?;

    // Example 2: Good quality (meets minimum standards)
    println!("📈 Example 2: GOOD quality (meets minimum standards)");
    let good = QualityMetrics {
        line_coverage_pct: 90.0,
        branch_coverage_pct: 85.0,
        mutation_score_pct: 75.0,
        avg_cyclomatic_complexity: 12.0,
        max_cyclomatic_complexity: 18,
        clippy_warnings: 0,
        clippy_errors: 0,
        doc_coverage_pct: 85.0,
    };

    print_metrics_analysis(&good, "Typical Project")?;

    // Example 3: Below standards (needs improvement)
    println!("📈 Example 3: BELOW STANDARDS (needs improvement)");
    let poor = QualityMetrics {
        line_coverage_pct: 75.0,
        branch_coverage_pct: 68.0,
        mutation_score_pct: 60.0,
        avg_cyclomatic_complexity: 18.5,
        max_cyclomatic_complexity: 32,
        clippy_warnings: 5,
        clippy_errors: 0,
        doc_coverage_pct: 45.0,
    };

    print_metrics_analysis(&poor, "Legacy Codebase")?;

    // Key takeaways
    println!("🎯 Key takeaways:");
    println!("   1. TDG converts subjective 'quality' into objective score");
    println!("   2. Weighted formula balances coverage, mutation, complexity, quality");
    println!("   3. Grade threshold (≥A-/90) enforces minimum standards");
    println!("   4. Ratchet effect: TDG should only improve over time");
    println!();

    println!("🛡️  pmat.toml configuration:");
    println!("   min_tdg_grade = 'A-'  # Blocks commits below 90");
    println!("   min_test_coverage = 95.0");
    println!("   max_cyclomatic_complexity = 15");
    println!();

    println!("🇪🇺 Toyota Way - Kaizen (Continuous Improvement):");
    println!("   ✓ Measure current state (TDG baseline)");
    println!("   ✓ Set improvement goals (target grade)");
    println!("   ✓ Track progress (TDG trend over sprints)");
    println!("   ✓ Prevent regression (ratchet effect)");
    println!();

    Ok(())
}

fn print_metrics_analysis(metrics: &QualityMetrics, project_name: &str) -> Result<()> {
    println!("   Project: {}", project_name);
    println!();

    // Show raw metrics
    println!("   📊 Raw metrics:");
    println!("      Line coverage:     {:.1}%", metrics.line_coverage_pct);
    println!("      Branch coverage:   {:.1}%", metrics.branch_coverage_pct);
    println!("      Mutation score:    {:.1}%", metrics.mutation_score_pct);
    println!("      Avg complexity:    {:.1}", metrics.avg_cyclomatic_complexity);
    println!("      Max complexity:    {}", metrics.max_cyclomatic_complexity);
    println!("      Clippy warnings:   {}", metrics.clippy_warnings);
    println!("      Clippy errors:     {}", metrics.clippy_errors);
    println!();

    // Calculate TDG score
    let tdg_score = metrics.calculate_tdg_score();
    let grade = Grade::from_score(tdg_score);

    println!("   🎯 TDG Score: {:.1} (Grade: {})", tdg_score, grade);
    println!();

    // Component breakdown
    let coverage_avg = (metrics.line_coverage_pct + metrics.branch_coverage_pct) / 2.0;
    println!("   📈 Component breakdown:");
    println!("      Coverage (40%):   {:.1}% → {:.1} points", coverage_avg, coverage_avg * 0.40);
    println!("      Mutation (30%):   {:.1}% → {:.1} points", metrics.mutation_score_pct, metrics.mutation_score_pct * 0.30);

    let complexity_penalty = (metrics.avg_cyclomatic_complexity / 15.0).min(1.0);
    let complexity_score = (1.0 - complexity_penalty) * 100.0;
    println!("      Complexity (15%): {:.1} → {:.1} points", complexity_score, complexity_score * 0.15);

    let total_issues = metrics.clippy_warnings + metrics.clippy_errors;
    let quality_score = if total_issues == 0 {
        100.0
    } else {
        (100.0 - (total_issues as f64 * 2.0)).max(0.0)
    };
    println!("      Quality (15%):    {:.1} → {:.1} points", quality_score, quality_score * 0.15);
    println!();

    // Pass/fail analysis
    let min_grade = 90.0; // A- threshold from pmat config
    if tdg_score >= min_grade {
        println!("   ✅ PASS: TDG {:.1} ≥ {:.1} (meets A- standard)", tdg_score, min_grade);
    } else {
        println!("   ❌ FAIL: TDG {:.1} < {:.1} (below A- standard)", tdg_score, min_grade);
        println!("      Needs {:.1} points to reach A-", min_grade - tdg_score);
    }
    println!();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grade_from_score() {
        assert_eq!(Grade::from_score(100.0), Grade::APlus);
        assert_eq!(Grade::from_score(95.0), Grade::APlus);
        assert_eq!(Grade::from_score(92.0), Grade::A);
        assert_eq!(Grade::from_score(90.0), Grade::A);
        assert_eq!(Grade::from_score(87.0), Grade::BMinus);
        assert_eq!(Grade::from_score(82.0), Grade::B);
        assert_eq!(Grade::from_score(75.0), Grade::C);
        assert_eq!(Grade::from_score(65.0), Grade::D);
        assert_eq!(Grade::from_score(50.0), Grade::F);
    }

    #[test]
    fn test_perfect_score() {
        let perfect = QualityMetrics {
            line_coverage_pct: 100.0,
            branch_coverage_pct: 100.0,
            mutation_score_pct: 100.0,
            avg_cyclomatic_complexity: 1.0,
            max_cyclomatic_complexity: 3,
            clippy_warnings: 0,
            clippy_errors: 0,
            doc_coverage_pct: 100.0,
        };

        let score = perfect.calculate_tdg_score();
        assert!(score >= 95.0, "Perfect score should be ≥95 (A+), got {}", score);
    }

    #[test]
    fn test_minimum_acceptable_score() {
        // Metrics that should produce a reasonable TDG score
        let acceptable = QualityMetrics {
            line_coverage_pct: 90.0,
            branch_coverage_pct: 85.0,
            mutation_score_pct: 80.0,
            avg_cyclomatic_complexity: 10.0,
            max_cyclomatic_complexity: 15,
            clippy_warnings: 0,
            clippy_errors: 0,
            doc_coverage_pct: 90.0,
        };

        let score = acceptable.calculate_tdg_score();
        // With these metrics:
        // - Coverage (40%): 87.5% * 0.40 = 35.0
        // - Mutation (30%): 80.0% * 0.30 = 24.0
        // - Complexity (15%): (1 - 10/15) * 100 * 0.15 = 5.0
        // - Quality (15%): 100.0 * 0.15 = 15.0
        // Total: 35.0 + 24.0 + 5.0 + 15.0 = 79.0
        assert!(score >= 75.0, "Acceptable metrics should score ≥75, got {}", score);
        assert!(score < 85.0, "These metrics shouldn't reach 85, got {}", score);
    }

    #[test]
    fn test_warnings_penalty() {
        let no_warnings = QualityMetrics {
            line_coverage_pct: 95.0,
            branch_coverage_pct: 90.0,
            mutation_score_pct: 85.0,
            avg_cyclomatic_complexity: 8.0,
            max_cyclomatic_complexity: 12,
            clippy_warnings: 0,
            clippy_errors: 0,
            doc_coverage_pct: 95.0,
        };

        let with_warnings = QualityMetrics {
            clippy_warnings: 5,
            ..no_warnings.clone()
        };

        let score_clean = no_warnings.calculate_tdg_score();
        let score_warnings = with_warnings.calculate_tdg_score();

        assert!(score_clean > score_warnings, "Warnings should lower score");
    }
}
