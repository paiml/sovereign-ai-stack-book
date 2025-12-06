/// Chapter 18: renacer - Syscall Profiling
///
/// **CLAIM:** renacer provides deterministic profiling:
/// - Reproducible performance metrics
/// - Type-safe profiling data
/// - Deterministic aggregation
///
/// **VALIDATION:** `make run-ch18`
use anyhow::Result;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Profiling event
#[derive(Debug, Clone)]
struct ProfileEvent {
    name: String,
    duration_ns: u64,
    category: EventCategory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum EventCategory {
    Compute,
    IO,
    Memory,
    Network,
}

impl ProfileEvent {
    fn new(name: &str, duration_ns: u64, category: EventCategory) -> Self {
        Self {
            name: name.to_string(),
            duration_ns,
            category,
        }
    }
}

/// Profiler for collecting metrics
struct Profiler {
    events: Vec<ProfileEvent>,
    active_spans: HashMap<String, Instant>,
}

impl Profiler {
    fn new() -> Self {
        Self {
            events: Vec::new(),
            active_spans: HashMap::new(),
        }
    }

    fn start_span(&mut self, name: &str) {
        self.active_spans.insert(name.to_string(), Instant::now());
    }

    fn end_span(&mut self, name: &str, category: EventCategory) {
        if let Some(start) = self.active_spans.remove(name) {
            let duration = start.elapsed();
            self.events.push(ProfileEvent::new(
                name,
                duration.as_nanos() as u64,
                category,
            ));
        }
    }

    fn record(&mut self, event: ProfileEvent) {
        self.events.push(event);
    }

    fn event_count(&self) -> usize {
        self.events.len()
    }

    /// Aggregate metrics by category
    fn aggregate_by_category(&self) -> HashMap<EventCategory, AggregateStats> {
        let mut stats: HashMap<EventCategory, Vec<u64>> = HashMap::new();

        for event in &self.events {
            stats
                .entry(event.category)
                .or_default()
                .push(event.duration_ns);
        }

        stats
            .into_iter()
            .map(|(cat, durations)| (cat, AggregateStats::from_durations(&durations)))
            .collect()
    }

    /// Get top N slowest events
    fn top_slowest(&self, n: usize) -> Vec<&ProfileEvent> {
        let mut events: Vec<_> = self.events.iter().collect();
        events.sort_by(|a, b| b.duration_ns.cmp(&a.duration_ns));
        events.truncate(n);
        events
    }
}

/// Aggregate statistics
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct AggregateStats {
    count: usize,
    total_ns: u64,
    mean_ns: f64,
    min_ns: u64,
    max_ns: u64,
}

impl AggregateStats {
    fn from_durations(durations: &[u64]) -> Self {
        let count = durations.len();
        let total_ns: u64 = durations.iter().sum();
        let mean_ns = total_ns as f64 / count as f64;
        let min_ns = *durations.iter().min().unwrap_or(&0);
        let max_ns = *durations.iter().max().unwrap_or(&0);

        Self {
            count,
            total_ns,
            mean_ns,
            min_ns,
            max_ns,
        }
    }
}

/// Demonstrate basic profiling
fn basic_demo() {
    println!("⏱️  Basic Profiling");
    println!();

    let mut profiler = Profiler::new();

    // Record some events
    profiler.record(ProfileEvent::new(
        "matrix_mul",
        1500000,
        EventCategory::Compute,
    ));
    profiler.record(ProfileEvent::new("file_read", 5000000, EventCategory::IO));
    profiler.record(ProfileEvent::new("malloc", 50000, EventCategory::Memory));
    profiler.record(ProfileEvent::new(
        "vector_add",
        200000,
        EventCategory::Compute,
    ));
    profiler.record(ProfileEvent::new("file_write", 3000000, EventCategory::IO));

    println!("   Recorded {} events", profiler.event_count());
    println!();

    let stats = profiler.aggregate_by_category();
    println!(
        "   {:>10} │ {:>6} │ {:>12} │ {:>12}",
        "Category", "Count", "Total (ns)", "Mean (ns)"
    );
    println!("   ───────────┼────────┼──────────────┼─────────────");

    let mut sorted: Vec<_> = stats.iter().collect();
    sorted.sort_by_key(|(cat, _)| format!("{:?}", cat));

    for (cat, stat) in sorted {
        println!(
            "   {:>10?} │ {:>6} │ {:>12} │ {:>12.0}",
            cat, stat.count, stat.total_ns, stat.mean_ns
        );
    }
    println!();
}

/// Demonstrate span-based profiling
fn span_demo() {
    println!("📊 Span-Based Profiling");
    println!();

    let mut profiler = Profiler::new();

    // Simulate profiling with spans
    profiler.start_span("operation_1");
    std::thread::sleep(Duration::from_micros(100));
    profiler.end_span("operation_1", EventCategory::Compute);

    profiler.start_span("operation_2");
    std::thread::sleep(Duration::from_micros(200));
    profiler.end_span("operation_2", EventCategory::IO);

    println!("   Captured {} spans", profiler.event_count());

    for event in &profiler.events {
        println!(
            "   - {}: {} ns ({:?})",
            event.name, event.duration_ns, event.category
        );
    }
    println!();
}

/// Demonstrate top-N analysis
fn top_n_demo() {
    println!("🏆 Top Slowest Operations");
    println!();

    let mut profiler = Profiler::new();

    profiler.record(ProfileEvent::new("op_fast", 100, EventCategory::Compute));
    profiler.record(ProfileEvent::new("op_slow", 10000000, EventCategory::IO));
    profiler.record(ProfileEvent::new(
        "op_medium",
        500000,
        EventCategory::Memory,
    ));
    profiler.record(ProfileEvent::new(
        "op_slower",
        5000000,
        EventCategory::Network,
    ));
    profiler.record(ProfileEvent::new("op_fastest", 50, EventCategory::Compute));

    let top = profiler.top_slowest(3);

    println!(
        "   {:>4} │ {:>15} │ {:>12}",
        "Rank", "Operation", "Duration (ns)"
    );
    println!("   ─────┼─────────────────┼─────────────");

    for (i, event) in top.iter().enumerate() {
        println!(
            "   {:>4} │ {:>15} │ {:>12}",
            i + 1,
            event.name,
            event.duration_ns
        );
    }
    println!();
}

/// Demonstrate determinism
fn determinism_demo() {
    println!("🔁 Aggregation Determinism");
    println!();

    let events = vec![
        ProfileEvent::new("a", 100, EventCategory::Compute),
        ProfileEvent::new("b", 200, EventCategory::Compute),
        ProfileEvent::new("c", 300, EventCategory::IO),
    ];

    let mut results = Vec::new();

    for run in 1..=5 {
        let mut profiler = Profiler::new();
        for event in &events {
            profiler.record(event.clone());
        }

        let stats = profiler.aggregate_by_category();
        let compute_mean = stats
            .get(&EventCategory::Compute)
            .map(|s| s.mean_ns)
            .unwrap_or(0.0);

        println!("   Run {}: Compute mean = {:.2} ns", run, compute_mean);
        results.push(compute_mean);
    }

    let first = results[0];
    let all_identical = results.iter().all(|&r| (r - first).abs() < 1e-10);

    println!();
    if all_identical {
        println!("   ✅ DETERMINISTIC: All aggregations identical");
    } else {
        println!("   ❌ Non-deterministic: Results varied!");
    }
    println!();
}

/// EU AI Act compliance
fn eu_compliance() {
    println!("🇪🇺 EU AI Act Compliance");
    println!();

    println!("   Article 10 (Data Governance):");
    println!("   ├─ All metrics local");
    println!("   ├─ No external profiling services");
    println!("   └─ Events fully tracked");
    println!();

    println!("   Article 13 (Transparency):");
    println!("   ├─ Performance bottlenecks visible");
    println!("   ├─ Aggregation algorithms explicit");
    println!("   └─ Results reproducible");
    println!();

    println!("   Article 15 (Robustness):");
    println!("   ├─ Type-safe event categories");
    println!("   ├─ Deterministic statistics");
    println!("   └─ Consistent ordering");
    println!();
}

fn main() -> Result<()> {
    println!("🔍 Chapter 18: renacer - Syscall Profiling");
    println!();
    println!("Deterministic, reproducible performance profiling.");
    println!();
    println!("{}", "─".repeat(70));
    println!();

    basic_demo();
    println!("{}", "─".repeat(70));
    println!();

    span_demo();
    println!("{}", "─".repeat(70));
    println!();

    top_n_demo();
    println!("{}", "─".repeat(70));
    println!();

    determinism_demo();
    println!("{}", "─".repeat(70));
    println!();

    eu_compliance();
    println!("{}", "─".repeat(70));
    println!();

    println!("🎯 Key Takeaways:");
    println!("   1. Type-safe profiling events");
    println!("   2. Deterministic aggregation");
    println!("   3. Span-based timing");
    println!("   4. EU AI Act compliant metrics");
    println!();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_creation() {
        let event = ProfileEvent::new("test", 1000, EventCategory::Compute);
        assert_eq!(event.name, "test");
        assert_eq!(event.duration_ns, 1000);
    }

    #[test]
    fn test_profiler_record() {
        let mut profiler = Profiler::new();
        profiler.record(ProfileEvent::new("a", 100, EventCategory::IO));
        profiler.record(ProfileEvent::new("b", 200, EventCategory::Memory));
        assert_eq!(profiler.event_count(), 2);
    }

    #[test]
    fn test_aggregate_stats() {
        let durations = vec![100, 200, 300];
        let stats = AggregateStats::from_durations(&durations);

        assert_eq!(stats.count, 3);
        assert_eq!(stats.total_ns, 600);
        assert!((stats.mean_ns - 200.0).abs() < 1e-10);
        assert_eq!(stats.min_ns, 100);
        assert_eq!(stats.max_ns, 300);
    }

    #[test]
    fn test_aggregate_by_category() {
        let mut profiler = Profiler::new();
        profiler.record(ProfileEvent::new("a", 100, EventCategory::Compute));
        profiler.record(ProfileEvent::new("b", 200, EventCategory::Compute));
        profiler.record(ProfileEvent::new("c", 500, EventCategory::IO));

        let stats = profiler.aggregate_by_category();

        let compute = stats
            .get(&EventCategory::Compute)
            .expect("compute stats exist");
        assert_eq!(compute.count, 2);
        assert_eq!(compute.total_ns, 300);

        let io = stats.get(&EventCategory::IO).expect("IO stats exist");
        assert_eq!(io.count, 1);
    }

    #[test]
    fn test_top_slowest() {
        let mut profiler = Profiler::new();
        profiler.record(ProfileEvent::new("fast", 100, EventCategory::Compute));
        profiler.record(ProfileEvent::new("slow", 1000, EventCategory::IO));
        profiler.record(ProfileEvent::new("medium", 500, EventCategory::Memory));

        let top = profiler.top_slowest(2);
        assert_eq!(top.len(), 2);
        assert_eq!(top[0].name, "slow");
        assert_eq!(top[1].name, "medium");
    }

    #[test]
    fn test_aggregation_determinism() {
        let events = vec![
            ProfileEvent::new("a", 100, EventCategory::Compute),
            ProfileEvent::new("b", 200, EventCategory::Compute),
        ];

        let mut results = Vec::new();
        for _ in 0..5 {
            let mut profiler = Profiler::new();
            for event in &events {
                profiler.record(event.clone());
            }
            let stats = profiler.aggregate_by_category();
            results.push(
                stats
                    .get(&EventCategory::Compute)
                    .expect("compute stats")
                    .mean_ns,
            );
        }

        let first = results[0];
        assert!(
            results.iter().all(|&r| (r - first).abs() < 1e-10),
            "Aggregation must be deterministic"
        );
    }
}
