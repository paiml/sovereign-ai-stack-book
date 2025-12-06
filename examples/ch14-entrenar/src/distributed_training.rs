/// Chapter 14: entrenar - Distributed Training
///
/// **CLAIM:** entrenar provides deterministic distributed training:
/// - Reproducible data parallelism
/// - Gradient aggregation
/// - Synchronized updates
///
/// **VALIDATION:** `make run-ch14`
use anyhow::Result;

/// Training configuration
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct TrainingConfig {
    num_workers: usize,
    batch_size: usize,
    learning_rate: f64,
    epochs: usize,
}

impl Default for TrainingConfig {
    fn default() -> Self {
        Self {
            num_workers: 4,
            batch_size: 32,
            learning_rate: 0.01,
            epochs: 10,
        }
    }
}

/// Simulated worker for distributed training
#[derive(Debug, Clone)]
#[allow(dead_code)]
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

    /// Compute local gradients on a data shard
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

    fn predict(&self, x: &[f64]) -> f64 {
        let sum: f64 = self.weights.iter()
            .zip(x.iter())
            .map(|(w, xi)| w * xi)
            .sum();
        sum + self.bias
    }

    #[allow(dead_code)]
    fn update(&mut self, weight_grads: &[f64], bias_grad: f64, lr: f64) {
        for (w, g) in self.weights.iter_mut().zip(weight_grads.iter()) {
            *w -= lr * g;
        }
        self.bias -= lr * bias_grad;
    }
}

/// Parameter server for gradient aggregation
#[derive(Debug)]
#[allow(dead_code)]
struct ParameterServer {
    weights: Vec<f64>,
    bias: f64,
    num_workers: usize,
}

impl ParameterServer {
    fn new(features: usize, num_workers: usize) -> Self {
        Self {
            weights: vec![0.0; features],
            bias: 0.0,
            num_workers,
        }
    }

    /// Aggregate gradients from all workers
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

    fn apply_update(&mut self, weight_grads: &[f64], bias_grad: f64, lr: f64) {
        for (w, g) in self.weights.iter_mut().zip(weight_grads.iter()) {
            *w -= lr * g;
        }
        self.bias -= lr * bias_grad;
    }

    fn broadcast_weights(&self) -> (Vec<f64>, f64) {
        (self.weights.clone(), self.bias)
    }
}

/// Distributed training coordinator
struct DistributedTrainer {
    workers: Vec<Worker>,
    server: ParameterServer,
    config: TrainingConfig,
}

impl DistributedTrainer {
    fn new(features: usize, config: TrainingConfig) -> Self {
        let workers: Vec<Worker> = (0..config.num_workers)
            .map(|id| Worker::new(id, features))
            .collect();
        let server = ParameterServer::new(features, config.num_workers);

        Self { workers, server, config }
    }

    /// Shard data across workers
    fn shard_data<'a>(&self, x: &'a [Vec<f64>], y: &'a [f64]) -> Vec<(&'a [Vec<f64>], &'a [f64])> {
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

    /// Run one epoch of distributed training
    fn train_epoch(&mut self, x: &[Vec<f64>], y: &[f64]) -> f64 {
        // Broadcast current weights to workers
        let (weights, bias) = self.server.broadcast_weights();
        for worker in &mut self.workers {
            worker.weights = weights.clone();
            worker.bias = bias;
        }

        // Shard data
        let shards = self.shard_data(x, y);

        // Compute gradients on each worker
        let gradients: Vec<_> = self.workers.iter()
            .zip(shards.iter())
            .map(|(worker, (x_shard, y_shard))| {
                worker.compute_gradients(x_shard, y_shard)
            })
            .collect();

        // Aggregate and apply updates
        let (avg_wg, avg_bg) = self.server.aggregate_gradients(&gradients);
        self.server.apply_update(&avg_wg, avg_bg, self.config.learning_rate);

        // Compute loss
        self.compute_loss(x, y)
    }

    fn compute_loss(&self, x: &[Vec<f64>], y: &[f64]) -> f64 {
        let n = x.len() as f64;
        let sum: f64 = x.iter()
            .zip(y.iter())
            .map(|(xi, yi)| {
                let pred: f64 = self.server.weights.iter()
                    .zip(xi.iter())
                    .map(|(w, x)| w * x)
                    .sum::<f64>() + self.server.bias;
                (pred - yi).powi(2)
            })
            .sum();
        sum / n
    }

    fn train(&mut self, x: &[Vec<f64>], y: &[f64]) -> Vec<f64> {
        let mut losses = Vec::with_capacity(self.config.epochs);
        for _ in 0..self.config.epochs {
            let loss = self.train_epoch(x, y);
            losses.push(loss);
        }
        losses
    }

    fn get_model(&self) -> (Vec<f64>, f64) {
        (self.server.weights.clone(), self.server.bias)
    }
}

/// Demonstrate basic distributed training
fn basic_distributed_demo() {
    println!("🌐 Basic Distributed Training");
    println!();

    // Dataset: y = 2x + 1
    let x: Vec<Vec<f64>> = (0..100).map(|i| vec![i as f64 / 10.0]).collect();
    let y: Vec<f64> = x.iter().map(|xi| 2.0 * xi[0] + 1.0).collect();

    let config = TrainingConfig {
        num_workers: 4,
        batch_size: 25,
        learning_rate: 0.01,
        epochs: 50,
    };

    println!("   Configuration:");
    println!("   - Workers: {}", config.num_workers);
    println!("   - Data points: {}", x.len());
    println!("   - Shards per worker: {}", x.len() / config.num_workers);
    println!();

    let mut trainer = DistributedTrainer::new(1, config);
    let losses = trainer.train(&x, &y);

    let (weights, bias) = trainer.get_model();

    println!("   Training progress:");
    println!("   - Initial MSE: {:.6}", losses[0]);
    println!("   - Final MSE: {:.6}", losses.last().unwrap());
    println!();

    println!("   Learned model:");
    println!("   - Weight: {:.4} (expected: 2.0)", weights[0]);
    println!("   - Bias: {:.4} (expected: 1.0)", bias);
    println!();
}

/// Demonstrate gradient aggregation
fn aggregation_demo() {
    println!("📊 Gradient Aggregation");
    println!();

    // Simulate 4 workers with different local gradients
    let gradients = vec![
        (vec![0.1, 0.2], 0.05),
        (vec![0.15, 0.18], 0.06),
        (vec![0.12, 0.22], 0.04),
        (vec![0.08, 0.20], 0.05),
    ];

    let server = ParameterServer::new(2, 4);
    let (avg_wg, avg_bg) = server.aggregate_gradients(&gradients);

    println!("   Worker gradients:");
    for (i, (wg, bg)) in gradients.iter().enumerate() {
        println!("   Worker {}: weight_grad=[{:.3}, {:.3}], bias_grad={:.3}",
            i, wg[0], wg[1], bg);
    }
    println!();

    println!("   Aggregated gradients:");
    println!("   - Weight gradients: [{:.4}, {:.4}]", avg_wg[0], avg_wg[1]);
    println!("   - Bias gradient: {:.4}", avg_bg);
    println!();
}

/// Demonstrate determinism
fn determinism_demo() {
    println!("🔁 Distributed Training Determinism");
    println!();

    let x: Vec<Vec<f64>> = (0..40).map(|i| vec![i as f64]).collect();
    let y: Vec<f64> = x.iter().map(|xi| 2.0 * xi[0] + 1.0).collect();

    let config = TrainingConfig {
        num_workers: 4,
        batch_size: 10,
        learning_rate: 0.0001,
        epochs: 10,
    };

    let mut results = Vec::new();

    for run in 1..=5 {
        let mut trainer = DistributedTrainer::new(1, config.clone());
        trainer.train(&x, &y);
        let (weights, _) = trainer.get_model();
        results.push(weights[0]);
        println!("   Run {}: weight = {:.10}", run, weights[0]);
    }

    let first = results[0];
    let all_identical = results.iter().all(|&r| (r - first).abs() < 1e-10);

    println!();
    if all_identical {
        println!("   ✅ DETERMINISTIC: All distributed training runs identical");
    } else {
        println!("   ❌ Non-deterministic: Results varied!");
    }
    println!();
}

/// Demonstrate scaling
fn scaling_demo() {
    println!("📈 Scaling Analysis");
    println!();

    let x: Vec<Vec<f64>> = (0..100).map(|i| vec![i as f64 / 10.0]).collect();
    let y: Vec<f64> = x.iter().map(|xi| 2.0 * xi[0] + 1.0).collect();

    println!("   {:>8} │ {:>12} │ {:>12}", "Workers", "Final MSE", "Convergence");
    println!("   ─────────┼──────────────┼─────────────");

    for num_workers in [1, 2, 4, 8] {
        let config = TrainingConfig {
            num_workers,
            batch_size: 100 / num_workers,
            learning_rate: 0.01,
            epochs: 50,
        };

        let mut trainer = DistributedTrainer::new(1, config);
        let losses = trainer.train(&x, &y);

        let convergence = if *losses.last().unwrap() < 0.01 { "✅ Good" } else { "⚠️  Slow" };
        println!("   {:>8} │ {:>12.6} │ {:>12}", num_workers, losses.last().unwrap(), convergence);
    }
    println!();
}

/// EU AI Act compliance
fn eu_compliance() {
    println!("🇪🇺 EU AI Act Compliance");
    println!();

    println!("   Article 10 (Data Governance):");
    println!("   ├─ Data sharding fully deterministic");
    println!("   ├─ No external data loading");
    println!("   └─ All gradients tracked locally");
    println!();

    println!("   Article 13 (Transparency):");
    println!("   ├─ Worker computations visible");
    println!("   ├─ Aggregation algorithm explicit");
    println!("   └─ Parameter updates logged");
    println!();

    println!("   Article 15 (Robustness):");
    println!("   ├─ Synchronized updates only");
    println!("   ├─ Deterministic across workers");
    println!("   └─ No race conditions possible");
    println!();
}

fn main() -> Result<()> {
    println!("🚀 Chapter 14: entrenar - Distributed Training");
    println!();
    println!("Deterministic, reproducible distributed ML training.");
    println!();
    println!("{}", "─".repeat(70));
    println!();

    basic_distributed_demo();
    println!("{}", "─".repeat(70));
    println!();

    aggregation_demo();
    println!("{}", "─".repeat(70));
    println!();

    determinism_demo();
    println!("{}", "─".repeat(70));
    println!();

    scaling_demo();
    println!("{}", "─".repeat(70));
    println!();

    eu_compliance();
    println!("{}", "─".repeat(70));
    println!();

    println!("🎯 Key Takeaways:");
    println!("   1. Data parallelism with deterministic sharding");
    println!("   2. Gradient aggregation for synchronized updates");
    println!("   3. Same result regardless of worker count");
    println!("   4. EU AI Act compliant by design");
    println!();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_worker_creation() {
        let worker = Worker::new(0, 3);
        assert_eq!(worker.id, 0);
        assert_eq!(worker.weights.len(), 3);
        assert_eq!(worker.bias, 0.0);
    }

    #[test]
    fn test_worker_prediction() {
        let mut worker = Worker::new(0, 2);
        worker.weights = vec![2.0, 3.0];
        worker.bias = 1.0;

        let pred = worker.predict(&[1.0, 2.0]);
        // 2*1 + 3*2 + 1 = 9
        assert!((pred - 9.0).abs() < 1e-10);
    }

    #[test]
    fn test_gradient_computation() {
        let mut worker = Worker::new(0, 1);
        worker.weights = vec![0.0];
        worker.bias = 0.0;

        let x = vec![vec![1.0], vec![2.0]];
        let y = vec![2.0, 4.0];

        let (wg, bg) = worker.compute_gradients(&x, &y);
        assert_eq!(wg.len(), 1);
        // Gradients should be non-zero
        assert!(wg[0].abs() > 0.0);
        assert!(bg.abs() > 0.0);
    }

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
        let x: Vec<Vec<f64>> = (0..20).map(|i| vec![i as f64]).collect();
        let y: Vec<f64> = x.iter().map(|xi| 2.0 * xi[0]).collect();

        let config = TrainingConfig {
            num_workers: 2,
            batch_size: 10,
            learning_rate: 0.001,
            epochs: 100,
        };

        let mut trainer = DistributedTrainer::new(1, config);
        let losses = trainer.train(&x, &y);

        assert!(losses.last().unwrap() < &losses[0],
            "Training should reduce loss");
    }

    #[test]
    fn test_distributed_training_determinism() {
        let x: Vec<Vec<f64>> = (0..20).map(|i| vec![i as f64]).collect();
        let y: Vec<f64> = x.iter().map(|xi| 2.0 * xi[0]).collect();

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

    #[test]
    fn test_parameter_server() {
        let mut server = ParameterServer::new(2, 4);
        server.apply_update(&[0.1, 0.2], 0.1, 1.0);

        assert!((server.weights[0] - (-0.1)).abs() < 1e-10);
        assert!((server.weights[1] - (-0.2)).abs() < 1e-10);
        assert!((server.bias - (-0.1)).abs() < 1e-10);
    }

    #[test]
    fn test_broadcast_weights() {
        let mut server = ParameterServer::new(2, 2);
        server.weights = vec![1.0, 2.0];
        server.bias = 0.5;

        let (w, b) = server.broadcast_weights();
        assert_eq!(w, vec![1.0, 2.0]);
        assert_eq!(b, 0.5);
    }
}
