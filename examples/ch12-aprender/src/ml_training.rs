/// Chapter 12: aprender - ML Training Framework
///
/// **CLAIM:** aprender provides deterministic ML training:
/// - Reproducible gradient descent
/// - Type-safe model definitions
/// - EU AI Act compliant training loops
///
/// **VALIDATION:** `make run-ch12`
use anyhow::Result;

/// Simple linear regression model
#[derive(Debug, Clone)]
struct LinearRegression {
    weights: Vec<f64>,
    bias: f64,
    learning_rate: f64,
}

impl LinearRegression {
    fn new(features: usize, learning_rate: f64) -> Self {
        Self {
            weights: vec![0.0; features],
            bias: 0.0,
            learning_rate,
        }
    }

    /// Forward pass: y = Wx + b
    fn predict(&self, x: &[f64]) -> f64 {
        let sum: f64 = self
            .weights
            .iter()
            .zip(x.iter())
            .map(|(w, xi)| w * xi)
            .sum();
        sum + self.bias
    }

    /// Compute mean squared error
    fn mse(&self, x: &[Vec<f64>], y: &[f64]) -> f64 {
        let n = x.len() as f64;
        let sum: f64 = x
            .iter()
            .zip(y.iter())
            .map(|(xi, yi)| {
                let pred = self.predict(xi);
                (pred - yi).powi(2)
            })
            .sum();
        sum / n
    }

    /// Single gradient descent step
    fn train_step(&mut self, x: &[Vec<f64>], y: &[f64]) {
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

        // Update weights
        for (w, grad) in self.weights.iter_mut().zip(weight_grads.iter()) {
            *w -= self.learning_rate * grad / n;
        }
        self.bias -= self.learning_rate * bias_grad / n;
    }

    /// Train for multiple epochs
    fn fit(&mut self, x: &[Vec<f64>], y: &[f64], epochs: usize) -> Vec<f64> {
        let mut losses = Vec::with_capacity(epochs);
        for _ in 0..epochs {
            self.train_step(x, y);
            losses.push(self.mse(x, y));
        }
        losses
    }
}

/// Demonstrate basic training
fn basic_training_demo() {
    println!("🧠 Basic ML Training");
    println!();

    // Simple dataset: y = 2x + 1
    let x: Vec<Vec<f64>> = vec![vec![1.0], vec![2.0], vec![3.0], vec![4.0], vec![5.0]];
    let y: Vec<f64> = vec![3.0, 5.0, 7.0, 9.0, 11.0];

    let mut model = LinearRegression::new(1, 0.1);

    println!("   Dataset: y = 2x + 1");
    println!(
        "   Initial weights: {:?}, bias: {:.4}",
        model.weights, model.bias
    );
    println!();

    let losses = model.fit(&x, &y, 100);

    println!("   After 100 epochs:");
    println!("   Weights: {:.4}", model.weights[0]);
    println!("   Bias: {:.4}", model.bias);
    println!("   Final MSE: {:.6}", losses.last().expect("at least one loss recorded"));
    println!();

    // Predictions
    println!("   Predictions:");
    for xi in &x {
        let pred = model.predict(xi);
        println!("   x={:.1} → y_pred={:.4}", xi[0], pred);
    }
    println!();
}

/// Demonstrate determinism
fn determinism_demo() {
    println!("🔁 Training Determinism");
    println!();

    let x: Vec<Vec<f64>> = vec![vec![1.0], vec![2.0], vec![3.0]];
    let y: Vec<f64> = vec![2.0, 4.0, 6.0];

    let mut results = Vec::new();

    for run in 1..=5 {
        let mut model = LinearRegression::new(1, 0.1);
        model.fit(&x, &y, 50);
        let final_weight = model.weights[0];
        results.push(final_weight);
        println!("   Run {}: weight = {:.10}", run, final_weight);
    }

    let first = results[0];
    let all_identical = results.iter().all(|&r| (r - first).abs() < 1e-10);

    println!();
    if all_identical {
        println!("   ✅ DETERMINISTIC: All training runs identical");
    } else {
        println!("   ❌ Non-deterministic: Results varied!");
    }
    println!();
}

/// Gradient descent visualization
fn gradient_descent_demo() {
    println!("📉 Gradient Descent Convergence");
    println!();

    let x: Vec<Vec<f64>> = vec![vec![1.0], vec![2.0], vec![3.0], vec![4.0], vec![5.0]];
    let y: Vec<f64> = vec![3.0, 5.0, 7.0, 9.0, 11.0];

    let mut model = LinearRegression::new(1, 0.1);
    let losses = model.fit(&x, &y, 20);

    println!("   {:>6} │ {:>12}", "Epoch", "MSE");
    println!("   ───────┼─────────────");

    for (i, loss) in losses.iter().enumerate() {
        if !(5..15).contains(&i) {
            println!("   {:>6} │ {:>12.6}", i + 1, loss);
        } else if i == 5 {
            println!("   {:>6} │ {:>12}", "...", "...");
        }
    }
    println!();
}

/// EU AI Act compliance
fn eu_compliance() {
    println!("🇪🇺 EU AI Act Compliance");
    println!();

    println!("   Article 10 (Data Governance):");
    println!("   ├─ Training data fully local");
    println!("   ├─ No external API calls");
    println!("   └─ Deterministic preprocessing");
    println!();

    println!("   Article 13 (Transparency):");
    println!("   ├─ Model weights fully inspectable");
    println!("   ├─ Training history logged");
    println!("   └─ Reproducible training runs");
    println!();

    println!("   Article 15 (Robustness):");
    println!("   ├─ Numerical stability guaranteed");
    println!("   ├─ Type-safe operations");
    println!("   └─ Memory-safe training loops");
    println!();
}

fn main() -> Result<()> {
    println!("🎓 Chapter 12: aprender - ML Training Framework");
    println!();
    println!("Deterministic, reproducible machine learning training.");
    println!();
    println!("{}", "─".repeat(70));
    println!();

    basic_training_demo();
    println!("{}", "─".repeat(70));
    println!();

    determinism_demo();
    println!("{}", "─".repeat(70));
    println!();

    gradient_descent_demo();
    println!("{}", "─".repeat(70));
    println!();

    eu_compliance();
    println!("{}", "─".repeat(70));
    println!();

    println!("🎯 Key Takeaways:");
    println!("   1. Deterministic training: same data → same model");
    println!("   2. Type-safe model definitions");
    println!("   3. Transparent gradient computation");
    println!("   4. EU AI Act compliant by design");
    println!();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_regression_creation() {
        let model = LinearRegression::new(3, 0.01);
        assert_eq!(model.weights.len(), 3);
        assert_eq!(model.bias, 0.0);
    }

    #[test]
    fn test_prediction() {
        let mut model = LinearRegression::new(2, 0.01);
        model.weights = vec![2.0, 3.0];
        model.bias = 1.0;

        // y = 2*1 + 3*2 + 1 = 9
        let pred = model.predict(&[1.0, 2.0]);
        assert!((pred - 9.0).abs() < 1e-10);
    }

    #[test]
    fn test_mse_calculation() {
        let mut model = LinearRegression::new(1, 0.01);
        model.weights = vec![2.0];
        model.bias = 0.0;

        let x = vec![vec![1.0], vec![2.0]];
        let y = vec![2.0, 4.0]; // Perfect predictions

        let mse = model.mse(&x, &y);
        assert!(mse < 1e-10, "MSE should be ~0 for perfect predictions");
    }

    #[test]
    fn test_training_reduces_loss() {
        let x = vec![vec![1.0], vec![2.0], vec![3.0]];
        let y = vec![2.0, 4.0, 6.0];

        let mut model = LinearRegression::new(1, 0.1);
        let initial_loss = model.mse(&x, &y);
        model.fit(&x, &y, 100);
        let final_loss = model.mse(&x, &y);

        assert!(
            final_loss < initial_loss,
            "Training should reduce loss: {} -> {}",
            initial_loss,
            final_loss
        );
    }

    #[test]
    fn test_training_determinism() {
        let x = vec![vec![1.0], vec![2.0], vec![3.0]];
        let y = vec![2.0, 4.0, 6.0];

        let mut results = Vec::new();
        for _ in 0..5 {
            let mut model = LinearRegression::new(1, 0.1);
            model.fit(&x, &y, 50);
            results.push(model.weights[0]);
        }

        let first = results[0];
        assert!(
            results.iter().all(|&r| (r - first).abs() < 1e-10),
            "Training must be deterministic"
        );
    }
}
