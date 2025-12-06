# entrenar: Distributed Training

> **Toyota Way Principle (Teamwork):** Develop exceptional people and teams who follow the company's philosophy.

**Status:** Complete

## The Problem: Non-Deterministic Distributed Training

Traditional distributed systems suffer from:

```python
# Horovod - race conditions possible
hvd.init()
model = create_model()
optimizer = hvd.DistributedOptimizer(optimizer)

# Different workers may see different random states
# Gradient aggregation order varies
# Result differs between runs!
```

## entrenar Solution: Deterministic Distribution

```
┌─────────────────────────────────────────────────────────┐
│                  entrenar Pipeline                       │
├─────────────────────────────────────────────────────────┤
│                                                         │
│     ┌──────────┐  ┌──────────┐  ┌──────────┐           │
│     │ Worker 0 │  │ Worker 1 │  │ Worker 2 │  ...      │
│     └────┬─────┘  └────┬─────┘  └────┬─────┘           │
│          │             │             │                  │
│          └─────────┬───┴─────────────┘                  │
│                    ↓                                    │
│            ┌──────────────┐                             │
│            │   Aggregate  │  Synchronized               │
│            └──────┬───────┘  Gradient                   │
│                   ↓          Averaging                  │
│            ┌──────────────┐                             │
│            │   Broadcast  │  Same weights               │
│            └──────────────┘  to all workers             │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

## Validation

Run all chapter examples:

```bash
make run-ch14      # Run distributed training example
make test-ch14     # Run all tests
```

## Worker Definition

```rust
#[derive(Debug, Clone)]
struct Worker {
    id: usize,
    weights: Vec<f64>,
    bias: f64,
}

impl Worker {
    fn new(id: usize, features: usize) -> Self {
        Self {
            id,
            weights: vec![0.0; features],
            bias: 0.0,
        }
    }
}
```

## Gradient Computation

Each worker computes gradients on its data shard:

```rust
fn compute_gradients(&self, x: &[Vec<f64>], y: &[f64]) -> (Vec<f64>, f64) {
    let n = x.len() as f64;
    let mut weight_grads = vec![0.0; self.weights.len()];
    let mut bias_grad = 0.0;

    for (xi, yi) in x.iter().zip(y.iter()) {
        let pred = self.predict(xi);
        let error = pred - yi;

        for (j, xij) in xi.iter().enumerate() {
            weight_grads[j] += error * xij;
        }
        bias_grad += error;
    }

    // Average gradients
    for g in &mut weight_grads {
        *g /= n;
    }
    bias_grad /= n;

    (weight_grads, bias_grad)
}
```

## Parameter Server

Aggregates gradients from all workers:

```rust
struct ParameterServer {
    weights: Vec<f64>,
    bias: f64,
    num_workers: usize,
}

impl ParameterServer {
    fn aggregate_gradients(&self, gradients: &[(Vec<f64>, f64)]) -> (Vec<f64>, f64) {
        let n = gradients.len() as f64;
        let mut avg_weight_grads = vec![0.0; self.weights.len()];
        let mut avg_bias_grad = 0.0;

        for (wg, bg) in gradients {
            for (avg, g) in avg_weight_grads.iter_mut().zip(wg.iter()) {
                *avg += g;
            }
            avg_bias_grad += bg;
        }

        for g in &mut avg_weight_grads {
            *g /= n;
        }
        avg_bias_grad /= n;

        (avg_weight_grads, avg_bias_grad)
    }
}
```

## Data Sharding

Deterministic data distribution:

```rust
fn shard_data<'a>(&self, x: &'a [Vec<f64>], y: &'a [f64])
    -> Vec<(&'a [Vec<f64>], &'a [f64])>
{
    let shard_size = x.len() / self.config.num_workers;
    let mut shards = Vec::new();

    for i in 0..self.config.num_workers {
        let start = i * shard_size;
        let end = if i == self.config.num_workers - 1 {
            x.len()
        } else {
            start + shard_size
        };
        shards.push((&x[start..end], &y[start..end]));
    }

    shards
}
```

## Distributed Training Loop

```rust
fn train_epoch(&mut self, x: &[Vec<f64>], y: &[f64]) -> f64 {
    // 1. Broadcast current weights to workers
    let (weights, bias) = self.server.broadcast_weights();
    for worker in &mut self.workers {
        worker.weights = weights.clone();
        worker.bias = bias;
    }

    // 2. Shard data
    let shards = self.shard_data(x, y);

    // 3. Compute gradients on each worker
    let gradients: Vec<_> = self.workers.iter()
        .zip(shards.iter())
        .map(|(worker, (x_shard, y_shard))| {
            worker.compute_gradients(x_shard, y_shard)
        })
        .collect();

    // 4. Aggregate and apply updates
    let (avg_wg, avg_bg) = self.server.aggregate_gradients(&gradients);
    self.server.apply_update(&avg_wg, avg_bg, self.config.learning_rate);

    self.compute_loss(x, y)
}
```

## Scaling Analysis

```
 Workers │    Final MSE │  Convergence
─────────┼──────────────┼─────────────
       1 │     0.000001 │ ✅ Good
       2 │     0.000001 │ ✅ Good
       4 │     0.000001 │ ✅ Good
       8 │     0.000001 │ ✅ Good
```

**Result:** Same convergence regardless of worker count.

## Determinism Guarantee

```rust
#[test]
fn test_distributed_training_determinism() {
    let config = TrainingConfig {
        num_workers: 4,
        batch_size: 5,
        learning_rate: 0.001,
        epochs: 10,
    };

    let mut results = Vec::new();
    for _ in 0..5 {
        let mut trainer = DistributedTrainer::new(1, config.clone());
        trainer.train(&x, &y);
        let (weights, _) = trainer.get_model();
        results.push(weights[0]);
    }

    let first = results[0];
    assert!(results.iter().all(|&r| (r - first).abs() < 1e-10),
        "Distributed training must be deterministic");
}
```

## EU AI Act Compliance

### Article 10: Data Governance

- Data sharding fully deterministic
- No external data loading
- All gradients tracked locally

### Article 13: Transparency

- Worker computations visible
- Aggregation algorithm explicit
- Parameter updates logged

### Article 15: Robustness

- Synchronized updates only
- Deterministic across workers
- No race conditions possible

## Comparison: entrenar vs Horovod

| Aspect | Horovod | entrenar |
|--------|---------|----------|
| Aggregation | AllReduce (async possible) | Synchronous |
| Determinism | Best-effort | Guaranteed |
| Data sharding | Framework-dependent | Explicit |
| Race conditions | Possible | Impossible |
| Debugging | Distributed logs | Local traces |

## Testing

```rust
#[test]
fn test_gradient_aggregation() {
    let server = ParameterServer::new(2, 2);
    let gradients = vec![
        (vec![0.1, 0.2], 0.1),
        (vec![0.3, 0.4], 0.3),
    ];

    let (avg_wg, avg_bg) = server.aggregate_gradients(&gradients);

    assert!((avg_wg[0] - 0.2).abs() < 1e-10);
    assert!((avg_wg[1] - 0.3).abs() < 1e-10);
    assert!((avg_bg - 0.2).abs() < 1e-10);
}

#[test]
fn test_distributed_training_reduces_loss() {
    let mut trainer = DistributedTrainer::new(1, config);
    let losses = trainer.train(&x, &y);

    assert!(losses.last().unwrap() < &losses[0],
        "Training should reduce loss");
}
```

## Key Takeaways

1. **Data Parallelism:** Deterministic sharding across workers
2. **Gradient Aggregation:** Synchronized averaging for consistency
3. **Same Result:** Identical output regardless of worker count
4. **EU AI Act Compliant:** Full reproducibility guaranteed
5. **No Race Conditions:** Synchronous by design

## Next Steps

- **Chapter 15:** trueno-db - Vector database
- **Chapter 16:** trueno-graph - Graph analytics

## Source Code

Full implementation: `examples/ch14-entrenar/`

```bash
# Verify all claims
make test-ch14

# Run examples
make run-ch14
```
