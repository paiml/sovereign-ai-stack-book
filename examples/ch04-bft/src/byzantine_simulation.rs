/// Chapter 4: Byzantine Fault Tolerance - Monte Carlo Failure Simulation
///
/// This example simulates Byzantine failure modes in multi-agent AI systems
/// using Monte Carlo methods to demonstrate statistical reliability improvements.
///
/// KEY INSIGHT: Byzantine faults are non-deterministic. Monte Carlo simulation
/// quantifies the probability distribution of failure modes.
///
/// This is CODE-FIRST: Real statistical simulation with reproducible results.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FailureMode {
    /// Model produces syntactically valid but semantically wrong output
    Hallucination,
    /// Model fails to produce any output
    Crash,
    /// Model produces corrupted or malformed output
    Corruption,
    /// Model succeeds (not a failure)
    Success,
}

#[derive(Debug, Clone)]
pub struct Agent {
    pub id: usize,
    pub failure_rate: f64,
}

impl Agent {
    pub fn new(id: usize, failure_rate: f64) -> Self {
        Self { id, failure_rate }
    }

    /// Simulate agent execution with deterministic pseudo-random behavior
    /// Returns (success, failure_mode)
    pub fn execute(&self, task_id: usize, seed: usize) -> (bool, FailureMode) {
        // Deterministic hash for reproducibility
        let hash = ((task_id * 31 + self.id * 17 + seed * 7) % 10000) as f64 / 10000.0;

        if hash < self.failure_rate {
            // Failure - determine which mode
            let mode_hash = ((task_id * 13 + self.id * 19) % 100) as f64 / 100.0;
            let failure_mode = if mode_hash < 0.60 {
                FailureMode::Hallucination // 60% of failures are hallucinations
            } else if mode_hash < 0.85 {
                FailureMode::Corruption // 25% are corruptions
            } else {
                FailureMode::Crash // 15% are crashes
            };
            (false, failure_mode)
        } else {
            (true, FailureMode::Success)
        }
    }
}

/// Byzantine consensus with N agents, tolerates up to f failures
/// Requires 2f + 1 agents for BFT (need f+1 honest agents for majority)
pub fn byzantine_consensus(agents: &[Agent], task_id: usize, seed: usize) -> (bool, HashMap<FailureMode, usize>) {
    let mut votes = HashMap::new();
    let mut failures = HashMap::new();

    for agent in agents {
        let (success, mode) = agent.execute(task_id, seed);

        if success {
            *votes.entry(FailureMode::Success).or_insert(0) += 1;
        } else {
            *failures.entry(mode.clone()).or_insert(0) += 1;
        }
    }

    // Byzantine consensus: majority wins
    let successes = *votes.get(&FailureMode::Success).unwrap_or(&0);
    let consensus_reached = successes > agents.len() / 2;

    (consensus_reached, failures)
}

/// Monte Carlo simulation: run N trials and collect statistics
pub struct MonteCarloSimulation {
    pub trials: usize,
    pub tasks_per_trial: usize,
}

impl MonteCarloSimulation {
    pub fn new(trials: usize, tasks_per_trial: usize) -> Self {
        Self {
            trials,
            tasks_per_trial,
        }
    }

    /// Run simulation for single-agent system
    pub fn simulate_single_agent(&self, failure_rate: f64) -> SimulationResult {
        let agent = Agent::new(0, failure_rate);
        let mut total_successes = 0;
        let mut total_tasks = 0;
        let mut failure_counts: HashMap<FailureMode, usize> = HashMap::new();

        for trial in 0..self.trials {
            for task_id in 0..self.tasks_per_trial {
                total_tasks += 1;
                let (success, mode) = agent.execute(task_id, trial);

                if success {
                    total_successes += 1;
                } else {
                    *failure_counts.entry(mode).or_insert(0) += 1;
                }
            }
        }

        SimulationResult {
            total_tasks,
            successes: total_successes,
            failures: failure_counts,
        }
    }

    /// Run simulation for Byzantine fault-tolerant multi-agent system
    pub fn simulate_bft_system(&self, num_agents: usize, failure_rate: f64) -> SimulationResult {
        let agents: Vec<Agent> = (0..num_agents)
            .map(|id| Agent::new(id, failure_rate))
            .collect();

        let mut total_successes = 0;
        let mut total_tasks = 0;
        let mut all_failures: HashMap<FailureMode, usize> = HashMap::new();

        for trial in 0..self.trials {
            for task_id in 0..self.tasks_per_trial {
                total_tasks += 1;
                let (success, failures) = byzantine_consensus(&agents, task_id, trial);

                if success {
                    total_successes += 1;
                } else {
                    for (mode, count) in failures {
                        *all_failures.entry(mode).or_insert(0) += count;
                    }
                }
            }
        }

        SimulationResult {
            total_tasks,
            successes: total_successes,
            failures: all_failures,
        }
    }
}

#[derive(Debug)]
pub struct SimulationResult {
    pub total_tasks: usize,
    pub successes: usize,
    pub failures: HashMap<FailureMode, usize>,
}

impl SimulationResult {
    pub fn success_rate(&self) -> f64 {
        self.successes as f64 / self.total_tasks as f64
    }

    pub fn failure_rate(&self) -> f64 {
        1.0 - self.success_rate()
    }

    pub fn total_failures(&self) -> usize {
        self.failures.values().sum()
    }

    pub fn print_summary(&self, _label: &str) {
        println!("   Total tasks: {}", self.total_tasks);
        println!("   Successes: {} ({:.2}%)", self.successes, self.success_rate() * 100.0);
        println!("   Failures: {} ({:.2}%)", self.total_failures(), self.failure_rate() * 100.0);

        if !self.failures.is_empty() {
            println!("\n   Failure modes:");
            for (mode, count) in &self.failures {
                let pct = (*count as f64 / self.total_failures() as f64) * 100.0;
                println!("      {:?}: {} ({:.1}%)", mode, count, pct);
            }
        }
    }
}

fn main() {
    println!("🎲 Chapter 4: Byzantine Fault Tolerance - Monte Carlo Simulation\n");

    let simulation = MonteCarloSimulation::new(100, 100); // 100 trials × 100 tasks = 10,000 samples

    println!("📊 Simulation Parameters:");
    println!("   Trials: {}", simulation.trials);
    println!("   Tasks per trial: {}", simulation.tasks_per_trial);
    println!("   Total samples: {}\n", simulation.trials * simulation.tasks_per_trial);

    // Scenario 1: Single agent with 23% failure rate
    println!("🤖 Scenario 1: Single Agent (No BFT)");
    println!("   Agent failure rate: 23%\n");

    let single_result = simulation.simulate_single_agent(0.23);
    single_result.print_summary("Single Agent");

    println!("\n{}\n", "=".repeat(70));

    // Scenario 2: BFT with 3 agents (tolerates 1 Byzantine fault)
    println!("🛡️  Scenario 2: Byzantine Fault Tolerance (3 agents, f=1)");
    println!("   Each agent failure rate: 23%");
    println!("   BFT threshold: 2f+1 = 3 agents (tolerates 1 fault)\n");

    let bft_3_result = simulation.simulate_bft_system(3, 0.23);
    bft_3_result.print_summary("BFT (3 agents)");

    println!("\n{}\n", "=".repeat(70));

    // Scenario 3: BFT with 5 agents (tolerates 2 Byzantine faults)
    println!("🛡️  Scenario 3: Byzantine Fault Tolerance (5 agents, f=2)");
    println!("   Each agent failure rate: 23%");
    println!("   BFT threshold: 2f+1 = 5 agents (tolerates 2 faults)\n");

    let bft_5_result = simulation.simulate_bft_system(5, 0.23);
    bft_5_result.print_summary("BFT (5 agents)");

    println!("\n{}\n", "=".repeat(70));

    // Analysis
    println!("📈 Comparative Analysis:\n");

    let improvement_3 = (bft_3_result.success_rate() - single_result.success_rate()) * 100.0;
    let improvement_5 = (bft_5_result.success_rate() - single_result.success_rate()) * 100.0;

    println!("   Single agent → 3-agent BFT:");
    println!("      Success rate: {:.2}% → {:.2}% (+{:.2} pp)",
             single_result.success_rate() * 100.0,
             bft_3_result.success_rate() * 100.0,
             improvement_3);

    println!("\n   Single agent → 5-agent BFT:");
    println!("      Success rate: {:.2}% → {:.2}% (+{:.2} pp)",
             single_result.success_rate() * 100.0,
             bft_5_result.success_rate() * 100.0,
             improvement_5);

    let failure_reduction = ((single_result.total_failures() - bft_5_result.total_failures()) as f64
        / single_result.total_failures() as f64) * 100.0;

    println!("\n   Failure reduction (5-agent BFT): {:.1}%", failure_reduction);

    println!("\n🎯 Key Insights:");
    println!("   1. Byzantine Fault Tolerance provides statistical guarantees");
    println!("   2. More agents = higher fault tolerance (but diminishing returns)");
    println!("   3. BFT transforms unreliable components into reliable systems");
    println!("   4. Monte Carlo simulation quantifies real-world reliability\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_deterministic() {
        let agent = Agent::new(0, 0.3);
        let (s1, m1) = agent.execute(42, 100);
        let (s2, m2) = agent.execute(42, 100);

        assert_eq!(s1, s2, "Agent should be deterministic");
        assert_eq!(m1, m2, "Failure mode should be deterministic");
    }

    #[test]
    fn test_agent_respects_failure_rate() {
        let agent = Agent::new(0, 0.25);
        let mut successes = 0;

        for task_id in 0..1000 {
            let (success, _) = agent.execute(task_id, 42);
            if success {
                successes += 1;
            }
        }

        let success_rate = successes as f64 / 1000.0;
        // Should be approximately 75% success (25% failure)
        assert!(success_rate > 0.70 && success_rate < 0.80,
                "Success rate should be ~75%, got {:.1}%", success_rate * 100.0);
    }

    #[test]
    fn test_bft_improves_reliability() {
        let simulation = MonteCarloSimulation::new(10, 100);

        let single = simulation.simulate_single_agent(0.23);
        let bft = simulation.simulate_bft_system(3, 0.23);

        assert!(
            bft.success_rate() > single.success_rate(),
            "BFT should improve success rate: {} vs {}",
            bft.success_rate(),
            single.success_rate()
        );
    }

    #[test]
    fn test_more_agents_better_reliability() {
        let simulation = MonteCarloSimulation::new(10, 100);

        let bft_3 = simulation.simulate_bft_system(3, 0.23);
        let bft_5 = simulation.simulate_bft_system(5, 0.23);

        assert!(
            bft_5.success_rate() >= bft_3.success_rate(),
            "More agents should provide better or equal reliability"
        );
    }

    #[test]
    fn test_byzantine_consensus_majority() {
        let agents = vec![
            Agent::new(0, 0.0), // Never fails
            Agent::new(1, 0.0), // Never fails
            Agent::new(2, 1.0), // Always fails
        ];

        let (success, _) = byzantine_consensus(&agents, 0, 42);
        assert!(success, "Majority (2/3) should reach consensus");
    }

    #[test]
    fn test_failure_mode_distribution() {
        let agent = Agent::new(0, 1.0); // Always fails
        let mut modes = HashMap::new();

        for task_id in 0..1000 {
            let (_, mode) = agent.execute(task_id, 42);
            *modes.entry(mode).or_insert(0) += 1;
        }

        // Check that we see all failure modes
        assert!(modes.contains_key(&FailureMode::Hallucination));
        assert!(modes.contains_key(&FailureMode::Corruption));
        assert!(modes.contains_key(&FailureMode::Crash));

        // Hallucination should be most common (~60%)
        let hallucinations = modes.get(&FailureMode::Hallucination).unwrap();
        assert!(*hallucinations > 500, "Hallucinations should be majority failure mode");
    }
}
