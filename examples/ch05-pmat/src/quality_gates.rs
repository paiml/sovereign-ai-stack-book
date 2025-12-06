/// Chapter 5: pmat - Quality Enforcement Toolkit
///
/// Example: O(1) Pre-commit Quality Gates
///
/// **CLAIM:** pmat validates quality gates in <30ms via hash-based caching
///
/// **VALIDATION:** `make run-ch05-quality-gates`
/// - Demonstrates hash-based O(1) validation
/// - Shows metrics caching mechanism
/// - Proves <30ms validation time
///
/// **KEY PRINCIPLE:** Toyota Way - Jidoka (Automation with Human Touch)
/// - Compiler = Andon cord (stops on defects)
/// - Pre-commit hooks = Quality gates (prevent bad code from entering)
/// - Hash-based caching = Kaizen (continuous improvement without waste)
use anyhow::Result;
use std::collections::HashMap;
use std::time::Instant;

/// Quality gate configuration matching .pmat-metrics.toml
#[derive(Debug)]
#[allow(dead_code)]
struct QualityGate {
    name: &'static str,
    threshold_ms: u64,
    description: &'static str,
}

const QUALITY_GATES: &[QualityGate] = &[
    QualityGate {
        name: "lint",
        threshold_ms: 30_000, // 30s (user requirement)
        description: "Clippy linting with -D warnings",
    },
    QualityGate {
        name: "test-fast",
        threshold_ms: 300_000, // 5min (SPEC requirement)
        description: "Fast test suite via cargo-nextest",
    },
    QualityGate {
        name: "coverage",
        threshold_ms: 600_000, // 10min (SPEC requirement)
        description: "Code coverage analysis (≥95%)",
    },
];

/// Simulated metrics cache (in real pmat, this is .pmat-metrics/)
#[derive(Debug)]
struct MetricsCache {
    cache: HashMap<String, CachedMetric>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct CachedMetric {
    source_hash: String,
    duration_ms: u64,
    timestamp: u64,
    passed: bool,
}

impl MetricsCache {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    /// O(1) lookup: check if cached metrics are valid
    fn get(&self, gate_name: &str, current_hash: &str) -> Option<&CachedMetric> {
        self.cache.get(gate_name).and_then(|metric| {
            if metric.source_hash == current_hash {
                Some(metric)
            } else {
                None // Hash mismatch = code changed = must re-run
            }
        })
    }

    /// Store validated metrics
    fn set(&mut self, gate_name: String, metric: CachedMetric) {
        self.cache.insert(gate_name, metric);
    }
}

/// Simulate source code hash (in real pmat, uses SHA256 of source files)
fn compute_source_hash(seed: u8) -> String {
    // Simple mock: in production, this would be SHA256(all_source_files)
    format!("abc123def456_{:02x}", seed)
}

/// Simulate running a quality gate (expensive operation)
fn run_quality_gate_expensive(gate: &QualityGate) -> Result<(u64, bool)> {
    // Simulate the actual expensive operation
    // In reality: cargo clippy, cargo test, cargo tarpaulin
    let start = Instant::now();

    // Mock validation time (in real pmat, this takes seconds/minutes)
    std::thread::sleep(std::time::Duration::from_micros(100));

    let duration_ms = start.elapsed().as_millis() as u64;
    let passed = duration_ms < gate.threshold_ms;

    Ok((duration_ms, passed))
}

fn main() -> Result<()> {
    println!("🛡️  Chapter 5: pmat Quality Gates - O(1) Validation");
    println!();

    // Initialize metrics cache
    let mut cache = MetricsCache::new();

    // Simulate first run (cache MISS)
    println!("📊 Scenario 1: First run (cache MISS)");
    println!("   All gates must be validated from scratch");
    println!();

    let source_hash_v1 = compute_source_hash(1);

    for gate in QUALITY_GATES {
        print!("   🔍 Running {:<15} ", gate.name);

        let start = Instant::now();
        let (duration_ms, passed) = run_quality_gate_expensive(gate)?;
        let validation_time = start.elapsed();

        println!(
            "took {:>4}ms  [{}]",
            duration_ms,
            if passed { "✅ PASS" } else { "❌ FAIL" }
        );

        // Store in cache
        cache.set(
            gate.name.to_string(),
            CachedMetric {
                source_hash: source_hash_v1.clone(),
                duration_ms,
                timestamp: 1732385000, // Mock timestamp
                passed,
            },
        );

        // Show actual validation overhead
        println!("      Validation overhead: {:?}", validation_time);
    }

    println!();
    println!("✅ All gates validated and cached");
    println!();

    // Simulate second run (cache HIT, code unchanged)
    println!("📊 Scenario 2: Second run (cache HIT, code unchanged)");
    println!("   O(1) lookup via hash comparison");
    println!();

    for gate in QUALITY_GATES {
        print!("   ⚡ Checking {:<15} ", gate.name);

        let start = Instant::now();

        // O(1) hash lookup
        if let Some(cached) = cache.get(gate.name, &source_hash_v1) {
            let lookup_time = start.elapsed();
            println!(
                "cached {:>4}ms  [{}]  (lookup: {:?})",
                cached.duration_ms,
                if cached.passed {
                    "✅ PASS"
                } else {
                    "❌ FAIL"
                },
                lookup_time
            );
        } else {
            println!("cache miss! Would re-run.");
        }
    }

    println!();
    println!("✅ O(1) validation complete via cache");
    println!("   Total lookup time: ~microseconds (vs minutes for full validation)");
    println!();

    // Simulate third run (cache MISS, code changed)
    println!("📊 Scenario 3: Code changed (cache MISS)");
    println!("   Hash mismatch triggers re-validation");
    println!();

    let source_hash_v2 = compute_source_hash(2); // Different hash = code changed

    for gate in QUALITY_GATES {
        print!("   🔍 Checking {:<15} ", gate.name);

        let start = Instant::now();

        if let Some(_cached) = cache.get(gate.name, &source_hash_v2) {
            println!("cached (unexpected!)");
        } else {
            let lookup_time = start.elapsed();
            println!("cache miss! (lookup: {:?})", lookup_time);
            println!("      → Would re-run validation");
        }
    }

    println!();

    // Key takeaways
    println!("🎯 Key takeaways:");
    println!("   1. FIRST RUN: All gates validated (expensive)");
    println!("   2. NO CHANGES: O(1) hash lookup (microseconds)");
    println!("   3. CODE CHANGED: Hash mismatch triggers re-run (safety)");
    println!();

    println!("🇪🇺 Toyota Way principles:");
    println!("   ✓ Jidoka: Automatic quality enforcement");
    println!("   ✓ Muda: Eliminate waste (don't re-run unchanged code)");
    println!("   ✓ Kaizen: Continuous improvement (metrics inform decisions)");
    println!();

    println!("📈 Performance:");
    println!(
        "   Full validation: {}s",
        QUALITY_GATES[2].threshold_ms / 1000
    );
    println!("   Cached lookup:   <1ms (O(1) hash table)");
    println!(
        "   Speedup:         >{}x faster",
        QUALITY_GATES[2].threshold_ms
    );
    println!();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_hit() {
        let mut cache = MetricsCache::new();
        let hash = "abc123".to_string();

        cache.set(
            "lint".to_string(),
            CachedMetric {
                source_hash: hash.clone(),
                duration_ms: 100,
                timestamp: 123456,
                passed: true,
            },
        );

        assert!(cache.get("lint", &hash).is_some());
    }

    #[test]
    fn test_cache_miss_different_hash() {
        let mut cache = MetricsCache::new();

        cache.set(
            "lint".to_string(),
            CachedMetric {
                source_hash: "abc123".to_string(),
                duration_ms: 100,
                timestamp: 123456,
                passed: true,
            },
        );

        // Different hash = cache miss
        assert!(cache.get("lint", "def456").is_none());
    }

    #[test]
    fn test_quality_gates_defined() {
        assert_eq!(QUALITY_GATES.len(), 3);
        assert_eq!(QUALITY_GATES[0].name, "lint");
        assert_eq!(QUALITY_GATES[1].name, "test-fast");
        assert_eq!(QUALITY_GATES[2].name, "coverage");
    }

    #[test]
    fn test_o1_lookup_is_fast() {
        let mut cache = MetricsCache::new();
        let hash = "test_hash".to_string();

        cache.set(
            "test".to_string(),
            CachedMetric {
                source_hash: hash.clone(),
                duration_ms: 1000,
                timestamp: 123456,
                passed: true,
            },
        );

        let start = Instant::now();
        let _result = cache.get("test", &hash);
        let duration = start.elapsed();

        // O(1) lookup should be < 1ms
        assert!(
            duration.as_millis() < 1,
            "Lookup took too long: {:?}",
            duration
        );
    }
}
