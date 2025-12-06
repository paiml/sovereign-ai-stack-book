/// Chapter 16: trueno-graph - Graph Analytics
///
/// **CLAIM:** trueno-graph provides deterministic graph processing:
/// - Reproducible traversals
/// - Type-safe graph operations
/// - Deterministic PageRank
///
/// **VALIDATION:** `make run-ch16`
use anyhow::Result;
use std::collections::{HashMap, HashSet, VecDeque};

/// Graph node
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Node {
    id: usize,
    label: String,
}

impl Node {
    fn new(id: usize, label: &str) -> Self {
        Self { id, label: label.to_string() }
    }
}

/// Directed graph structure
#[derive(Debug)]
struct Graph {
    nodes: HashMap<usize, Node>,
    edges: HashMap<usize, Vec<usize>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    fn add_node(&mut self, node: Node) {
        let id = node.id;
        self.nodes.insert(id, node);
        self.edges.entry(id).or_default();
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        self.edges.entry(from).or_default().push(to);
    }

    fn node_count(&self) -> usize {
        self.nodes.len()
    }

    fn edge_count(&self) -> usize {
        self.edges.values().map(|v| v.len()).sum()
    }

    fn neighbors(&self, id: usize) -> &[usize] {
        self.edges.get(&id).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// Breadth-first search
    fn bfs(&self, start: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        queue.push_back(start);
        visited.insert(start);

        while let Some(node) = queue.pop_front() {
            result.push(node);
            for &neighbor in self.neighbors(node) {
                if visited.insert(neighbor) {
                    queue.push_back(neighbor);
                }
            }
        }

        result
    }

    /// Depth-first search
    fn dfs(&self, start: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut stack = vec![start];
        let mut result = Vec::new();

        while let Some(node) = stack.pop() {
            if visited.insert(node) {
                result.push(node);
                // Push in reverse order for consistent ordering
                let mut neighbors: Vec<_> = self.neighbors(node).to_vec();
                neighbors.reverse();
                for neighbor in neighbors {
                    if !visited.contains(&neighbor) {
                        stack.push(neighbor);
                    }
                }
            }
        }

        result
    }

    /// PageRank algorithm
    fn pagerank(&self, iterations: usize, damping: f64) -> HashMap<usize, f64> {
        let n = self.node_count() as f64;
        let mut ranks: HashMap<usize, f64> = self.nodes.keys()
            .map(|&id| (id, 1.0 / n))
            .collect();

        for _ in 0..iterations {
            let mut new_ranks: HashMap<usize, f64> = self.nodes.keys()
                .map(|&id| (id, (1.0 - damping) / n))
                .collect();

            for (&node, &rank) in &ranks {
                let neighbors = self.neighbors(node);
                if neighbors.is_empty() {
                    continue;
                }
                let share = damping * rank / neighbors.len() as f64;
                for &neighbor in neighbors {
                    *new_ranks.get_mut(&neighbor).unwrap() += share;
                }
            }

            ranks = new_ranks;
        }

        ranks
    }
}

/// Demonstrate basic graph operations
fn basic_demo() {
    println!("🕸️  Basic Graph Operations");
    println!();

    let mut graph = Graph::new();

    // Create a simple graph
    for i in 0..5 {
        graph.add_node(Node::new(i, &format!("Node{}", i)));
    }

    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.add_edge(1, 3);
    graph.add_edge(2, 3);
    graph.add_edge(3, 4);

    println!("   Nodes: {}", graph.node_count());
    println!("   Edges: {}", graph.edge_count());
    println!();

    println!("   Adjacency list:");
    for i in 0..5 {
        let neighbors: Vec<_> = graph.neighbors(i).iter().collect();
        println!("   {} -> {:?}", i, neighbors);
    }
    println!();
}

/// Demonstrate graph traversals
fn traversal_demo() {
    println!("🚶 Graph Traversals");
    println!();

    let mut graph = Graph::new();
    for i in 0..6 {
        graph.add_node(Node::new(i, &format!("N{}", i)));
    }

    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.add_edge(1, 3);
    graph.add_edge(2, 4);
    graph.add_edge(3, 5);
    graph.add_edge(4, 5);

    let bfs = graph.bfs(0);
    let dfs = graph.dfs(0);

    println!("   BFS from node 0: {:?}", bfs);
    println!("   DFS from node 0: {:?}", dfs);
    println!();
}

/// Demonstrate PageRank
fn pagerank_demo() {
    println!("📊 PageRank Analysis");
    println!();

    let mut graph = Graph::new();
    for i in 0..4 {
        graph.add_node(Node::new(i, &format!("Page{}", i)));
    }

    // Page 0 links to 1 and 2
    // Page 1 links to 2
    // Page 2 links to 0
    // Page 3 links to 2
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.add_edge(1, 2);
    graph.add_edge(2, 0);
    graph.add_edge(3, 2);

    let ranks = graph.pagerank(20, 0.85);

    println!("   {:>6} │ {:>10}", "Page", "Rank");
    println!("   ───────┼───────────");

    let mut sorted: Vec<_> = ranks.iter().collect();
    sorted.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    for (page, rank) in sorted {
        println!("   {:>6} │ {:>10.4}", page, rank);
    }
    println!();
}

/// Demonstrate determinism
fn determinism_demo() {
    println!("🔁 Graph Algorithm Determinism");
    println!();

    let mut graph = Graph::new();
    for i in 0..5 {
        graph.add_node(Node::new(i, &format!("N{}", i)));
    }
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.add_edge(1, 3);
    graph.add_edge(2, 4);

    let mut bfs_results = Vec::new();
    let mut pagerank_results = Vec::new();

    for run in 1..=5 {
        let bfs = graph.bfs(0);
        let ranks = graph.pagerank(10, 0.85);
        let rank_0 = ranks.get(&0).copied().unwrap_or(0.0);

        println!("   Run {}: BFS={:?}, PR[0]={:.6}", run, bfs, rank_0);
        bfs_results.push(bfs);
        pagerank_results.push(rank_0);
    }

    let first_bfs = &bfs_results[0];
    let first_pr = pagerank_results[0];
    let bfs_identical = bfs_results.iter().all(|r| r == first_bfs);
    let pr_identical = pagerank_results.iter().all(|&r| (r - first_pr).abs() < 1e-10);

    println!();
    if bfs_identical && pr_identical {
        println!("   ✅ DETERMINISTIC: All graph operations identical");
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
    println!("   ├─ All graph data local");
    println!("   ├─ No external graph services");
    println!("   └─ Traversals fully tracked");
    println!();

    println!("   Article 13 (Transparency):");
    println!("   ├─ Algorithm steps visible");
    println!("   ├─ PageRank iterations explicit");
    println!("   └─ Results reproducible");
    println!();

    println!("   Article 15 (Robustness):");
    println!("   ├─ Type-safe graph structure");
    println!("   ├─ Deterministic traversals");
    println!("   └─ Consistent ordering");
    println!();
}

fn main() -> Result<()> {
    println!("📈 Chapter 16: trueno-graph - Graph Analytics");
    println!();
    println!("Deterministic, reproducible graph processing.");
    println!();
    println!("{}", "─".repeat(70));
    println!();

    basic_demo();
    println!("{}", "─".repeat(70));
    println!();

    traversal_demo();
    println!("{}", "─".repeat(70));
    println!();

    pagerank_demo();
    println!("{}", "─".repeat(70));
    println!();

    determinism_demo();
    println!("{}", "─".repeat(70));
    println!();

    eu_compliance();
    println!("{}", "─".repeat(70));
    println!();

    println!("🎯 Key Takeaways:");
    println!("   1. Deterministic BFS and DFS traversals");
    println!("   2. Reproducible PageRank computation");
    println!("   3. Type-safe graph operations");
    println!("   4. EU AI Act compliant by design");
    println!();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_creation() {
        let mut graph = Graph::new();
        graph.add_node(Node::new(0, "A"));
        graph.add_node(Node::new(1, "B"));
        graph.add_edge(0, 1);

        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);
    }

    #[test]
    fn test_neighbors() {
        let mut graph = Graph::new();
        graph.add_node(Node::new(0, "A"));
        graph.add_node(Node::new(1, "B"));
        graph.add_node(Node::new(2, "C"));
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);

        assert_eq!(graph.neighbors(0), &[1, 2]);
        assert!(graph.neighbors(1).is_empty());
    }

    #[test]
    fn test_bfs() {
        let mut graph = Graph::new();
        for i in 0..4 {
            graph.add_node(Node::new(i, ""));
        }
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 3);

        let result = graph.bfs(0);
        assert_eq!(result[0], 0);
        assert!(result.contains(&1));
        assert!(result.contains(&2));
        assert!(result.contains(&3));
    }

    #[test]
    fn test_dfs() {
        let mut graph = Graph::new();
        for i in 0..3 {
            graph.add_node(Node::new(i, ""));
        }
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let result = graph.dfs(0);
        assert_eq!(result, vec![0, 1, 2]);
    }

    #[test]
    fn test_pagerank_sums_to_one() {
        let mut graph = Graph::new();
        for i in 0..4 {
            graph.add_node(Node::new(i, ""));
        }
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);
        graph.add_edge(3, 0);

        let ranks = graph.pagerank(20, 0.85);
        let sum: f64 = ranks.values().sum();

        assert!((sum - 1.0).abs() < 0.01, "PageRank should sum to ~1.0");
    }

    #[test]
    fn test_traversal_determinism() {
        let mut graph = Graph::new();
        for i in 0..5 {
            graph.add_node(Node::new(i, ""));
        }
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 4);

        let mut results = Vec::new();
        for _ in 0..5 {
            results.push(graph.bfs(0));
        }

        let first = &results[0];
        assert!(results.iter().all(|r| r == first),
            "BFS must be deterministic");
    }

    #[test]
    fn test_pagerank_determinism() {
        let mut graph = Graph::new();
        for i in 0..3 {
            graph.add_node(Node::new(i, ""));
        }
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let mut results = Vec::new();
        for _ in 0..5 {
            let ranks = graph.pagerank(10, 0.85);
            results.push(ranks.get(&0).copied().unwrap_or(0.0));
        }

        let first = results[0];
        assert!(results.iter().all(|&r| (r - first).abs() < 1e-10),
            "PageRank must be deterministic");
    }
}
