/// Chapter 15: trueno-db - Vector Database
///
/// **CLAIM:** trueno-db provides deterministic vector storage:
/// - Exact nearest neighbor search
/// - Reproducible similarity queries
/// - Type-safe embeddings
///
/// **VALIDATION:** `make run-ch15`
use anyhow::Result;
use std::collections::HashMap;

/// Vector embedding with metadata
#[derive(Debug, Clone)]
struct Embedding {
    id: String,
    vector: Vec<f64>,
    metadata: HashMap<String, String>,
}

impl Embedding {
    fn new(id: &str, vector: Vec<f64>) -> Self {
        Self {
            id: id.to_string(),
            vector,
            metadata: HashMap::new(),
        }
    }

    fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }

    fn dimension(&self) -> usize {
        self.vector.len()
    }
}

/// Distance metrics for similarity search
#[derive(Debug, Clone, Copy, PartialEq)]
enum DistanceMetric {
    Euclidean,
    Cosine,
    DotProduct,
}

/// Compute distance between two vectors
fn compute_distance(a: &[f64], b: &[f64], metric: DistanceMetric) -> f64 {
    match metric {
        DistanceMetric::Euclidean => a
            .iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).powi(2))
            .sum::<f64>()
            .sqrt(),
        DistanceMetric::Cosine => {
            let dot: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
            let norm_a: f64 = a.iter().map(|x| x.powi(2)).sum::<f64>().sqrt();
            let norm_b: f64 = b.iter().map(|x| x.powi(2)).sum::<f64>().sqrt();
            1.0 - (dot / (norm_a * norm_b))
        }
        DistanceMetric::DotProduct => -a.iter().zip(b.iter()).map(|(x, y)| x * y).sum::<f64>(),
    }
}

/// Search result with distance
#[derive(Debug, Clone)]
struct SearchResult {
    id: String,
    distance: f64,
    #[allow(dead_code)]
    embedding: Embedding,
}

/// Vector database with exact search
struct VectorDB {
    embeddings: Vec<Embedding>,
    dimension: usize,
    metric: DistanceMetric,
}

impl VectorDB {
    fn new(dimension: usize, metric: DistanceMetric) -> Self {
        Self {
            embeddings: Vec::new(),
            dimension,
            metric,
        }
    }

    fn insert(&mut self, embedding: Embedding) -> Result<(), String> {
        if embedding.dimension() != self.dimension {
            return Err(format!(
                "Dimension mismatch: expected {}, got {}",
                self.dimension,
                embedding.dimension()
            ));
        }
        self.embeddings.push(embedding);
        Ok(())
    }

    fn search(&self, query: &[f64], k: usize) -> Vec<SearchResult> {
        let mut results: Vec<_> = self
            .embeddings
            .iter()
            .map(|e| SearchResult {
                id: e.id.clone(),
                distance: compute_distance(query, &e.vector, self.metric),
                embedding: e.clone(),
            })
            .collect();

        results.sort_by(|a, b| {
            a.distance
                .partial_cmp(&b.distance)
                .expect("valid distance comparison")
        });
        results.truncate(k);
        results
    }

    fn get(&self, id: &str) -> Option<&Embedding> {
        self.embeddings.iter().find(|e| e.id == id)
    }

    fn len(&self) -> usize {
        self.embeddings.len()
    }

    fn delete(&mut self, id: &str) -> bool {
        let initial_len = self.embeddings.len();
        self.embeddings.retain(|e| e.id != id);
        self.embeddings.len() < initial_len
    }
}

/// Demonstrate basic operations
fn basic_demo() {
    println!("📊 Basic Vector Database Operations");
    println!();

    let mut db = VectorDB::new(3, DistanceMetric::Euclidean);

    // Insert embeddings
    let embeddings = vec![
        Embedding::new("doc1", vec![1.0, 0.0, 0.0]).with_metadata("title", "Document 1"),
        Embedding::new("doc2", vec![0.0, 1.0, 0.0]).with_metadata("title", "Document 2"),
        Embedding::new("doc3", vec![0.0, 0.0, 1.0]).with_metadata("title", "Document 3"),
        Embedding::new("doc4", vec![0.5, 0.5, 0.0]).with_metadata("title", "Document 4"),
    ];

    for emb in embeddings {
        db.insert(emb).expect("embedding insertion should succeed");
    }

    println!("   Inserted {} embeddings", db.len());
    println!("   Dimension: {}", 3);
    println!("   Metric: Euclidean");
    println!();

    // Search
    let query = vec![0.6, 0.4, 0.0];
    let results = db.search(&query, 3);

    println!(
        "   Query: [{:.1}, {:.1}, {:.1}]",
        query[0], query[1], query[2]
    );
    println!();
    println!("   {:>6} │ {:>10}", "ID", "Distance");
    println!("   ───────┼───────────");
    for r in &results {
        println!("   {:>6} │ {:>10.4}", r.id, r.distance);
    }
    println!();
}

/// Demonstrate distance metrics
fn distance_metrics_demo() {
    println!("📏 Distance Metrics");
    println!();

    let a = vec![1.0, 2.0, 3.0];
    let b = vec![4.0, 5.0, 6.0];

    println!("   Vector A: [{:.1}, {:.1}, {:.1}]", a[0], a[1], a[2]);
    println!("   Vector B: [{:.1}, {:.1}, {:.1}]", b[0], b[1], b[2]);
    println!();

    let euclidean = compute_distance(&a, &b, DistanceMetric::Euclidean);
    let cosine = compute_distance(&a, &b, DistanceMetric::Cosine);
    let dot = compute_distance(&a, &b, DistanceMetric::DotProduct);

    println!("   {:>12} │ {:>10}", "Metric", "Distance");
    println!("   ─────────────┼───────────");
    println!("   {:>12} │ {:>10.4}", "Euclidean", euclidean);
    println!("   {:>12} │ {:>10.4}", "Cosine", cosine);
    println!("   {:>12} │ {:>10.4}", "DotProduct", dot);
    println!();
}

/// Demonstrate determinism
fn determinism_demo() {
    println!("🔁 Search Determinism");
    println!();

    let mut db = VectorDB::new(4, DistanceMetric::Cosine);

    for i in 0..10 {
        let v: Vec<f64> = (0..4).map(|j| (i * j) as f64 / 10.0).collect();
        db.insert(Embedding::new(&format!("v{}", i), v))
            .expect("embedding insertion should succeed");
    }

    let query = vec![0.5, 0.3, 0.2, 0.1];
    let mut results_history = Vec::new();

    for run in 1..=5 {
        let results = db.search(&query, 3);
        let ids: Vec<_> = results.iter().map(|r| r.id.clone()).collect();
        println!("   Run {}: {:?}", run, ids);
        results_history.push(ids);
    }

    let first = &results_history[0];
    let all_identical = results_history.iter().all(|r| r == first);

    println!();
    if all_identical {
        println!("   ✅ DETERMINISTIC: All search results identical");
    } else {
        println!("   ❌ Non-deterministic: Results varied!");
    }
    println!();
}

/// Demonstrate CRUD operations
fn crud_demo() {
    println!("🔧 CRUD Operations");
    println!();

    let mut db = VectorDB::new(2, DistanceMetric::Euclidean);

    // Create
    db.insert(Embedding::new("item1", vec![1.0, 2.0]))
        .expect("insert item1");
    db.insert(Embedding::new("item2", vec![3.0, 4.0]))
        .expect("insert item2");
    println!("   CREATE: Inserted 2 items");

    // Read
    if let Some(emb) = db.get("item1") {
        println!("   READ: item1 = {:?}", emb.vector);
    }

    // Update (delete + insert)
    db.delete("item1");
    db.insert(Embedding::new("item1", vec![5.0, 6.0]))
        .expect("upsert item1");
    if let Some(emb) = db.get("item1") {
        println!("   UPDATE: item1 = {:?}", emb.vector);
    }

    // Delete
    let deleted = db.delete("item2");
    println!("   DELETE: item2 removed = {}", deleted);
    println!("   Remaining items: {}", db.len());
    println!();
}

/// EU AI Act compliance
fn eu_compliance() {
    println!("🇪🇺 EU AI Act Compliance");
    println!();

    println!("   Article 10 (Data Governance):");
    println!("   ├─ All embeddings stored locally");
    println!("   ├─ No external vector services");
    println!("   └─ Metadata fully tracked");
    println!();

    println!("   Article 13 (Transparency):");
    println!("   ├─ Exact search (no approximation)");
    println!("   ├─ Distance computation visible");
    println!("   └─ Results fully reproducible");
    println!();

    println!("   Article 15 (Robustness):");
    println!("   ├─ Type-safe embeddings");
    println!("   ├─ Dimension validation");
    println!("   └─ Deterministic ordering");
    println!();
}

fn main() -> Result<()> {
    println!("💾 Chapter 15: trueno-db - Vector Database");
    println!();
    println!("Deterministic, reproducible vector storage and search.");
    println!();
    println!("{}", "─".repeat(70));
    println!();

    basic_demo();
    println!("{}", "─".repeat(70));
    println!();

    distance_metrics_demo();
    println!("{}", "─".repeat(70));
    println!();

    determinism_demo();
    println!("{}", "─".repeat(70));
    println!();

    crud_demo();
    println!("{}", "─".repeat(70));
    println!();

    eu_compliance();
    println!("{}", "─".repeat(70));
    println!();

    println!("🎯 Key Takeaways:");
    println!("   1. Exact nearest neighbor search");
    println!("   2. Multiple distance metrics");
    println!("   3. Type-safe embeddings with validation");
    println!("   4. Deterministic query results");
    println!();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedding_creation() {
        let emb = Embedding::new("test", vec![1.0, 2.0, 3.0]);
        assert_eq!(emb.id, "test");
        assert_eq!(emb.dimension(), 3);
    }

    #[test]
    fn test_embedding_metadata() {
        let emb = Embedding::new("test", vec![1.0]).with_metadata("key", "value");
        assert_eq!(emb.metadata.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_euclidean_distance() {
        let a = vec![0.0, 0.0];
        let b = vec![3.0, 4.0];
        let dist = compute_distance(&a, &b, DistanceMetric::Euclidean);
        assert!((dist - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_cosine_distance() {
        let a = vec![1.0, 0.0];
        let b = vec![0.0, 1.0];
        let dist = compute_distance(&a, &b, DistanceMetric::Cosine);
        assert!((dist - 1.0).abs() < 1e-10); // Orthogonal = max distance
    }

    #[test]
    fn test_db_insert_and_search() {
        let mut db = VectorDB::new(2, DistanceMetric::Euclidean);
        db.insert(Embedding::new("a", vec![1.0, 0.0]))
            .expect("insert a");
        db.insert(Embedding::new("b", vec![0.0, 1.0]))
            .expect("insert b");

        let results = db.search(&[0.9, 0.1], 1);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "a");
    }

    #[test]
    fn test_dimension_validation() {
        let mut db = VectorDB::new(3, DistanceMetric::Euclidean);
        let result = db.insert(Embedding::new("bad", vec![1.0, 2.0]));
        assert!(result.is_err());
    }

    #[test]
    fn test_search_determinism() {
        let mut db = VectorDB::new(3, DistanceMetric::Euclidean);
        for i in 0..10 {
            let v: Vec<f64> = (0..3).map(|j| (i + j) as f64).collect();
            db.insert(Embedding::new(&format!("v{}", i), v))
                .expect("embedding insertion should succeed");
        }

        let query = vec![5.0, 5.0, 5.0];
        let mut results_history = Vec::new();
        for _ in 0..5 {
            let results = db.search(&query, 3);
            let ids: Vec<_> = results.iter().map(|r| r.id.clone()).collect();
            results_history.push(ids);
        }

        let first = &results_history[0];
        assert!(
            results_history.iter().all(|r| r == first),
            "Search must be deterministic"
        );
    }

    #[test]
    fn test_delete() {
        let mut db = VectorDB::new(2, DistanceMetric::Euclidean);
        db.insert(Embedding::new("a", vec![1.0, 0.0]))
            .expect("insert a");
        assert_eq!(db.len(), 1);

        let deleted = db.delete("a");
        assert!(deleted);
        assert_eq!(db.len(), 0);
    }
}
