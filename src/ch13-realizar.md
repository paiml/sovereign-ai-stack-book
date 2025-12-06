# realizar: Inference Engine

> **Toyota Way Principle (Heijunka):** Level the workload. Batch inference for consistent throughput and predictable latency.

**Status:** Complete

## The Problem: Unpredictable Inference

Traditional inference systems suffer from:

```python
# PyTorch inference - hidden non-determinism
model.eval()
with torch.no_grad():
    pred1 = model(x)
    pred2 = model(x)  # May differ due to dropout state!
```

## realizar Solution: Deterministic Inference

```
┌─────────────────────────────────────────────────────────┐
│                  realizar Pipeline                       │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Input → Validate → Batch → Predict → Verify → Output  │
│    │         │        │        │        │        │     │
│    ↓         ↓        ↓        ↓        ↓        ↓     │
│  Typed   Bounds   Efficient  Exact   Tracked  Logged   │
│  Data    Check    Batches   Results  Bounds   Response │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

## Validation

Run all chapter examples:

```bash
make run-ch13      # Run inference example
make test-ch13     # Run all tests
```

## Model Definition

```rust
#[derive(Debug, Clone)]
struct Model {
    weights: Vec<f64>,
    bias: f64,
    config: InferenceConfig,
}

impl Model {
    fn new(weights: Vec<f64>, bias: f64) -> Self {
        Self {
            weights,
            bias,
            config: InferenceConfig::default(),
        }
    }
}
```

## Single Prediction

```rust
fn predict(&self, x: &[f64]) -> f64 {
    let sum: f64 = self.weights.iter()
        .zip(x.iter())
        .map(|(w, xi)| w * xi)
        .sum();
    sum + self.bias
}
```

## Batch Inference

For efficiency, process multiple inputs at once:

```rust
fn predict_batch(&self, batch: &[Vec<f64>]) -> Vec<f64> {
    batch.iter().map(|x| self.predict(x)).collect()
}
```

### Example Output

```
   Input   │ Prediction
─────────┼───────────
[1.0, 1.0] │     6.0000
[2.0, 2.0] │    11.0000
[3.0, 3.0] │    16.0000
```

## Uncertainty Quantification

Provide confidence bounds with predictions:

```rust
struct PredictionResult {
    value: f64,
    lower_bound: f64,
    upper_bound: f64,
}

fn predict_with_bounds(&self, x: &[f64], uncertainty: f64) -> PredictionResult {
    let prediction = self.predict(x);
    PredictionResult {
        value: prediction,
        lower_bound: prediction - uncertainty,
        upper_bound: prediction + uncertainty,
    }
}
```

### Validation Against Targets

```
   x │   Target │       Bounds │ Hit?
─────┼──────────┼──────────────┼───────
 1.0 │     3.00 │ [2.50, 3.50] │ ✅
 2.0 │     5.00 │ [4.50, 5.50] │ ✅
 3.0 │     6.50 │ [6.50, 7.50] │ ✅
 4.0 │    10.00 │ [8.50, 9.50] │ ❌
```

## Inference Engine

Manage multiple models:

```rust
struct InferenceEngine {
    models: Vec<(String, Model)>,
}

impl InferenceEngine {
    fn new() -> Self {
        Self { models: Vec::new() }
    }

    fn register_model(&mut self, name: &str, model: Model) {
        self.models.push((name.to_string(), model));
    }

    fn predict(&self, model_name: &str, x: &[f64]) -> Option<f64> {
        self.get_model(model_name).map(|m| m.predict(x))
    }
}
```

## Determinism Guarantee

```rust
#[test]
fn test_inference_determinism() {
    let model = Model::new(vec![1.5, 2.5], 0.5);
    let input = vec![1.0, 2.0];

    let mut results = Vec::new();
    for _ in 0..10 {
        results.push(model.predict(&input));
    }

    let first = results[0];
    assert!(results.iter().all(|&r| (r - first).abs() < 1e-15),
        "Inference must be deterministic");
}
```

**Result:** All 10 runs produce identical results to 15 decimal places.

## Configuration

```rust
#[derive(Debug, Clone)]
struct InferenceConfig {
    batch_size: usize,
    num_threads: usize,
    precision: Precision,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Precision {
    F32,
    F64,
}
```

## EU AI Act Compliance

### Article 10: Data Governance

- Model weights fully specified
- No external model loading
- Inference data stays local

### Article 13: Transparency

- Predictions fully explainable
- Uncertainty bounds provided
- Model architecture visible

### Article 15: Robustness

- Deterministic predictions
- Type-safe operations
- Batch processing reliable

## Comparison: realizar vs TensorFlow Serving

| Aspect | TensorFlow Serving | realizar |
|--------|-------------------|----------|
| Model format | SavedModel (opaque) | Rust struct (transparent) |
| Determinism | Approximate | Exact |
| Batching | Automatic | Explicit |
| Uncertainty | Not built-in | First-class support |
| Memory safety | C++ runtime | Rust ownership |

## Testing

```rust
#[test]
fn test_single_prediction() {
    let model = Model::new(vec![2.0], 1.0);
    let pred = model.predict(&[3.0]);
    // y = 2*3 + 1 = 7
    assert!((pred - 7.0).abs() < 1e-10);
}

#[test]
fn test_batch_prediction() {
    let model = Model::new(vec![2.0], 0.0);
    let batch = vec![vec![1.0], vec![2.0], vec![3.0]];
    let preds = model.predict_batch(&batch);

    assert_eq!(preds.len(), 3);
    assert!((preds[0] - 2.0).abs() < 1e-10);
    assert!((preds[1] - 4.0).abs() < 1e-10);
    assert!((preds[2] - 6.0).abs() < 1e-10);
}

#[test]
fn test_prediction_bounds() {
    let model = Model::new(vec![1.0], 0.0);
    let result = model.predict_with_bounds(&[5.0], 1.0);

    assert!(result.contains(5.0));
    assert!(result.contains(4.5));
    assert!(!result.contains(3.0));
}
```

## Key Takeaways

1. **Deterministic Inference:** Same input always produces same output
2. **Batch Processing:** Efficient handling of multiple inputs
3. **Uncertainty Bounds:** Every prediction has confidence intervals
4. **Model Registry:** Manage multiple models in one engine
5. **Type Safety:** Compile-time guarantees on model operations

## Next Steps

- **Chapter 14:** entrenar - Distributed training
- **Chapter 15:** trueno-db - Vector database

## Source Code

Full implementation: `examples/ch13-realizar/`

```bash
# Verify all claims
make test-ch13

# Run examples
make run-ch13
```
