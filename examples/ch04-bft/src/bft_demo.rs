/// Chapter 4: Byzantine Fault Tolerance for Multi-Agent Systems
///
/// Example 1: Byzantine Fault Tolerance Demonstration
///
/// **CLAIM:** With n=3f+1 nodes, the system tolerates f Byzantine (faulty) nodes.
/// For f=1 faulty node, we need n=4 nodes minimum.
///
/// **VALIDATION:** `make run-ch04-bft`
/// - Demonstrates consensus with honest majority
/// - Shows system failure when Byzantine nodes exceed threshold
/// - Proves 3f+1 formula empirically
///
/// **KEY PRINCIPLE:** Byzantine Generals Problem
/// - Nodes may fail or lie (like LLMs hallucinating)
/// - Consensus requires honest majority
/// - Practical BFT: Use multiple models, vote on results
use anyhow::Result;
use std::collections::HashMap;

/// Simulated node that can be honest or Byzantine (faulty)
#[derive(Debug, Clone)]
struct Node {
    #[allow(dead_code)]
    id: usize,
    is_byzantine: bool,
}

impl Node {
    fn new(id: usize, is_byzantine: bool) -> Self {
        Self { id, is_byzantine }
    }

    /// Node processes input and returns result
    /// Byzantine nodes may return incorrect results
    fn process(&self, input: i32) -> i32 {
        if self.is_byzantine {
            // Byzantine node returns wrong answer (simulates LLM hallucination)
            input * 2 + 999 // Clearly wrong
        } else {
            // Honest node returns correct answer
            input * 2
        }
    }
}

/// Byzantine Fault Tolerant consensus system
#[derive(Debug)]
struct BftConsensus {
    nodes: Vec<Node>,
    fault_tolerance: usize, // f in the 3f+1 formula
}

impl BftConsensus {
    /// Create BFT system with given fault tolerance
    /// Requires n = 3f + 1 nodes
    fn new(fault_tolerance: usize) -> Self {
        let num_nodes = 3 * fault_tolerance + 1;
        let nodes: Vec<Node> = (0..num_nodes)
            .map(|id| Node::new(id, false))
            .collect();

        Self {
            nodes,
            fault_tolerance,
        }
    }

    /// Set specific nodes as Byzantine
    fn set_byzantine(&mut self, node_ids: &[usize]) {
        for &id in node_ids {
            if id < self.nodes.len() {
                self.nodes[id].is_byzantine = true;
            }
        }
    }

    /// Get consensus result using majority voting
    fn consensus(&self, input: i32) -> Option<i32> {
        let mut votes: HashMap<i32, usize> = HashMap::new();

        // Collect votes from all nodes
        for node in &self.nodes {
            let result = node.process(input);
            *votes.entry(result).or_insert(0) += 1;
        }

        // Find majority (need > 2f + 1 votes for safety)
        let threshold = 2 * self.fault_tolerance + 1;

        for (result, count) in &votes {
            if *count >= threshold {
                return Some(*result);
            }
        }

        None // No consensus reached
    }

    /// Display node status
    fn status(&self) {
        let byzantine_count = self.nodes.iter().filter(|n| n.is_byzantine).count();
        let honest_count = self.nodes.len() - byzantine_count;

        println!("   Nodes: {} total ({} honest, {} Byzantine)",
            self.nodes.len(), honest_count, byzantine_count);
        println!("   Fault tolerance: f={}", self.fault_tolerance);
        println!("   Threshold for consensus: {} votes", 2 * self.fault_tolerance + 1);
    }
}

fn main() -> Result<()> {
    println!("🛡️  Chapter 4: Byzantine Fault Tolerance Demonstration");
    println!();
    println!("Byzantine Generals Problem:");
    println!("   How do distributed nodes reach consensus when some nodes may lie?");
    println!("   Solution: With n=3f+1 nodes, we can tolerate f Byzantine (faulty) nodes.");
    println!();
    println!("{}", "─".repeat(70));
    println!();

    // Test 1: No Byzantine nodes (all honest)
    println!("📊 Test 1: No Byzantine nodes (f=0 actual, f=1 tolerance)");
    let mut bft = BftConsensus::new(1); // n=4 nodes, tolerates f=1
    bft.status();

    let input = 21;
    let result = bft.consensus(input);
    println!("   Input: {}", input);
    println!("   Expected: {} (input * 2)", input * 2);
    println!("   Result: {:?}", result);
    println!("   ✅ Consensus reached: {}", result.is_some());
    println!();

    // Test 2: One Byzantine node (within tolerance)
    println!("📊 Test 2: One Byzantine node (f=1 actual, f=1 tolerance)");
    bft.set_byzantine(&[0]); // Node 0 is Byzantine
    bft.status();

    let result = bft.consensus(input);
    println!("   Input: {}", input);
    println!("   Expected: {}", input * 2);
    println!("   Result: {:?}", result);
    println!("   ✅ Consensus reached despite 1 Byzantine node: {}", result.is_some());
    println!();

    // Test 3: Two Byzantine nodes (EXCEEDS tolerance for n=4)
    println!("📊 Test 3: Two Byzantine nodes (f=2 actual, f=1 tolerance) - FAILURE");
    bft.set_byzantine(&[0, 1]); // Nodes 0 and 1 are Byzantine
    bft.status();

    let result = bft.consensus(input);
    println!("   Input: {}", input);
    println!("   Expected: {}", input * 2);
    println!("   Result: {:?}", result);
    if result.is_none() {
        println!("   ❌ No consensus: Byzantine nodes exceed tolerance (f=2 > f=1)");
    }
    println!();

    // Test 4: Higher fault tolerance (f=2, n=7)
    println!("📊 Test 4: Higher fault tolerance (f=2, n=7 nodes)");
    let mut bft_high = BftConsensus::new(2); // n=7 nodes, tolerates f=2
    bft_high.set_byzantine(&[0, 1]); // 2 Byzantine nodes
    bft_high.status();

    let result = bft_high.consensus(input);
    println!("   Input: {}", input);
    println!("   Expected: {}", input * 2);
    println!("   Result: {:?}", result);
    println!("   ✅ Consensus reached with 2 Byzantine nodes (f=2 tolerance): {}", result.is_some());
    println!();

    println!("{}", "─".repeat(70));
    println!();

    // Key formula
    println!("🔢 The 3f+1 Formula:");
    println!();
    println!("   | f (faults) | n (nodes) | Threshold |");
    println!("   |------------|-----------|-----------|");
    println!("   | 1          | 4         | 3 votes   |");
    println!("   | 2          | 7         | 5 votes   |");
    println!("   | 3          | 10        | 7 votes   |");
    println!();

    // Application to LLMs
    println!("🤖 Application to LLM Systems:");
    println!();
    println!("   LLMs can be viewed as Byzantine nodes:");
    println!("   - They may hallucinate (return wrong answers)");
    println!("   - They may be inconsistent (same input → different outputs)");
    println!("   - They may fail silently (no error, just wrong)");
    println!();
    println!("   BFT Solution for LLMs:");
    println!("   - Use multiple LLMs (Claude, GPT-4, Llama)");
    println!("   - Vote on results");
    println!("   - Require consensus before accepting output");
    println!();

    // EU AI Act compliance
    println!("🇪🇺 EU AI Act Compliance:");
    println!("   ✅ Article 15 (Robustness): System tolerates faulty components");
    println!("   ✅ Article 13 (Transparency): Consensus mechanism is auditable");
    println!("   ✅ Reliability: Byzantine tolerance provides formal guarantees");
    println!();

    // Key takeaway
    println!("🎯 Key takeaway:");
    println!("   Byzantine Fault Tolerance provides MATHEMATICAL guarantees");
    println!("   for reliability when components (LLMs) may fail or lie.");
    println!("   Formula: n = 3f + 1 nodes to tolerate f Byzantine failures.");
    println!();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consensus_no_byzantine() {
        let bft = BftConsensus::new(1);
        let result = bft.consensus(10);
        assert_eq!(result, Some(20), "Should reach consensus with no Byzantine nodes");
    }

    #[test]
    fn test_consensus_one_byzantine() {
        let mut bft = BftConsensus::new(1);
        bft.set_byzantine(&[0]);
        let result = bft.consensus(10);
        assert_eq!(result, Some(20), "Should reach consensus with 1 Byzantine node");
    }

    #[test]
    fn test_no_consensus_too_many_byzantine() {
        let mut bft = BftConsensus::new(1);
        bft.set_byzantine(&[0, 1]); // 2 Byzantine > f=1 tolerance
        let result = bft.consensus(10);
        assert_eq!(result, None, "Should NOT reach consensus with 2 Byzantine nodes");
    }

    #[test]
    fn test_higher_fault_tolerance() {
        let mut bft = BftConsensus::new(2); // f=2, n=7
        bft.set_byzantine(&[0, 1]); // 2 Byzantine = f tolerance
        let result = bft.consensus(10);
        assert_eq!(result, Some(20), "Should reach consensus with f=2 tolerance");
    }

    #[test]
    fn test_3f_plus_1_formula() {
        // Verify the formula: n = 3f + 1
        for f in 1..=5 {
            let bft = BftConsensus::new(f);
            assert_eq!(bft.nodes.len(), 3 * f + 1, "Node count should be 3f+1");
        }
    }
}
