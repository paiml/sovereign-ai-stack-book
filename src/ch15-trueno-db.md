# trueno-db: Vector Database

> **Toyota Way Principle (Built-in Quality):** Build quality in at every step. Exact search ensures reproducible results.

**Status:** Complete

## The Problem: Approximate Search

Traditional vector databases use approximate methods:

```python
# FAISS - approximate nearest neighbors
index = faiss.IndexIVFFlat(d, nlist)
index.train(data)
D, I = index.search(query, k)  # Results may vary!
```

## trueno-db Solution: Exact Deterministic Search

```
┌─────────────────────────────────────────────────────────┐
│                  trueno-db Pipeline                      │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Embedding → Validate → Store → Query → Exact Match    │
│      │          │         │       │         │          │
│      ↓          ↓         ↓       ↓         ↓          │
│   Typed    Dimension   Local   Distance  Deterministic │
│   Vector   Check       Storage  Compute  Ranking       │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

## Validation

Run all chapter examples:

```bash
make run-ch15      # Run vector database example
make test-ch15     # Run all tests
```

## Embedding Definition

```rust
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
}
```

## Distance Metrics

```rust
#[derive(Debug, Clone, Copy)]
enum DistanceMetric {
    Euclidean,   // L2 distance
    Cosine,      // Cosine similarity
    DotProduct,  // Inner product
}

fn compute_distance(a: &[f64], b: &[f64], metric: DistanceMetric) -> f64 {
    match metric {
        DistanceMetric::Euclidean => {
            a.iter().zip(b.iter())
                .map(|(x, y)| (x - y).powi(2))
                .sum::<f64>()
                .sqrt()
        }
        DistanceMetric::Cosine => {
            let dot: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
            let norm_a = a.iter().map(|x| x.powi(2)).sum::<f64>().sqrt();
            let norm_b = b.iter().map(|x| x.powi(2)).sum::<f64>().sqrt();
            1.0 - (dot / (norm_a * norm_b))
        }
        DistanceMetric::DotProduct => {
            -a.iter().zip(b.iter()).map(|(x, y)| x * y).sum::<f64>()
        }
    }
}
```

### Distance Comparison

```
   Vector A: [1.0, 2.0, 3.0]
   Vector B: [4.0, 5.0, 6.0]

      Metric │   Distance
─────────────┼───────────
   Euclidean │     5.1962
      Cosine │     0.0254
  DotProduct │   -32.0000
```

## Vector Database

```rust
struct VectorDB {
    embeddings: Vec<Embedding>,
    dimension: usize,
    metric: DistanceMetric,
}

impl VectorDB {
    fn insert(&mut self, embedding: Embedding) -> Result<(), String> {
        if embedding.dimension() != self.dimension {
            return Err("Dimension mismatch".into());
        }
        self.embeddings.push(embedding);
        Ok(())
    }

    fn search(&self, query: &[f64], k: usize) -> Vec<SearchResult> {
        let mut results: Vec<_> = self.embeddings.iter()
            .map(|e| SearchResult {
                id: e.id.clone(),
                distance: compute_distance(query, &e.vector, self.metric),
                embedding: e.clone(),
            })
            .collect();

        results.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
        results.truncate(k);
        results
    }
}
```

## Search Results

```
   Query: [0.6, 0.4, 0.0]

     ID │   Distance
────────┼───────────
   doc4 │     0.1414
   doc1 │     0.5657
   doc2 │     0.7211
```

## CRUD Operations

```rust
// Create
db.insert(Embedding::new("item1", vec![1.0, 2.0])).unwrap();

// Read
let emb = db.get("item1");

// Update (delete + insert)
db.delete("item1");
db.insert(Embedding::new("item1", vec![5.0, 6.0])).unwrap();

// Delete
db.delete("item2");
```

## Determinism Guarantee

```rust
#[test]
fn test_search_determinism() {
    let mut db = VectorDB::new(3, DistanceMetric::Euclidean);
    // ... insert embeddings ...

    let query = vec![5.0, 5.0, 5.0];
    let mut results_history = Vec::new();
    for _ in 0..5 {
        let results = db.search(&query, 3);
        let ids: Vec<_> = results.iter().map(|r| r.id.clone()).collect();
        results_history.push(ids);
    }

    let first = &results_history[0];
    assert!(results_history.iter().all(|r| r == first),
        "Search must be deterministic");
}
```

**Result:** All 5 searches return identical rankings.

## EU AI Act Compliance

### Article 10: Data Governance

- All embeddings stored locally
- No external vector services
- Metadata fully tracked

### Article 13: Transparency

- Exact search (no approximation)
- Distance computation visible
- Results fully reproducible

### Article 15: Robustness

- Type-safe embeddings
- Dimension validation
- Deterministic ordering

## Comparison: trueno-db vs Pinecone

| Aspect | Pinecone | trueno-db |
|--------|----------|-----------|
| Search type | Approximate | Exact |
| Data location | Cloud | Local |
| Determinism | Best-effort | Guaranteed |
| Audit trail | Limited | Full |
| Latency | Variable | Predictable |

## Testing

```rust
#[test]
fn test_euclidean_distance() {
    let a = vec![0.0, 0.0];
    let b = vec![3.0, 4.0];
    let dist = compute_distance(&a, &b, DistanceMetric::Euclidean);
    assert!((dist - 5.0).abs() < 1e-10);  // 3-4-5 triangle
}

#[test]
fn test_dimension_validation() {
    let mut db = VectorDB::new(3, DistanceMetric::Euclidean);
    let result = db.insert(Embedding::new("bad", vec![1.0, 2.0]));
    assert!(result.is_err());  // Wrong dimension rejected
}
```

## Key Takeaways

1. **Exact Search:** No approximation, reproducible results
2. **Multiple Metrics:** Euclidean, Cosine, Dot Product
3. **Type Safety:** Dimension validation at insert time
4. **Deterministic:** Same query always returns same results
5. **Local Storage:** Full control over your data

## Next Steps

- **Chapter 16:** trueno-graph - Graph analytics
- **Chapter 17:** batuta - Workflow orchestration

## Source Code

Full implementation: `examples/ch15-trueno-db/`

```bash
# Verify all claims
make test-ch15

# Run examples
make run-ch15
```
