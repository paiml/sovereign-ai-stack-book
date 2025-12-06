/// Chapter 2: Crisis of Determinism in the Age of Generative AI
///
/// Example 1: Deterministic Baseline (Traditional ML)
///
/// **CLAIM:** Traditional ML (linear regression, decision trees, etc.) produces
/// deterministic, reproducible results.
///
/// **VALIDATION:** `make run-ch02-baseline`
/// - Run the example multiple times
/// - Verify identical outputs every time
/// - Prove determinism through repeated execution
///
/// **KEY PRINCIPLE:** SCIENTIFIC REPRODUCIBILITY
/// - Same input → Same output (always)
/// - No randomness, no variance
/// - Perfect for EU AI Act Article 13 (transparency requirements)
use anyhow::Result;
use std::time::Instant;

/// Simple linear regression (deterministic)
/// Model: y = mx + b
#[derive(Debug, Clone)]
struct LinearModel {
    slope: f64,
    intercept: f64,
}

impl LinearModel {
    /// Fit model using ordinary least squares (OLS)
    /// This is completely deterministic - same data always gives same model
    fn fit(x: &[f64], y: &[f64]) -> Result<Self> {
        assert_eq!(x.len(), y.len(), "x and y must have same length");
        let n = x.len() as f64;

        // Calculate means
        let mean_x: f64 = x.iter().sum::<f64>() / n;
        let mean_y: f64 = y.iter().sum::<f64>() / n;

        // Calculate slope: m = Σ((x - mean_x)(y - mean_y)) / Σ((x - mean_x)²)
        let mut numerator = 0.0;
        let mut denominator = 0.0;

        for i in 0..x.len() {
            let x_diff = x[i] - mean_x;
            let y_diff = y[i] - mean_y;
            numerator += x_diff * y_diff;
            denominator += x_diff * x_diff;
        }

        let slope = numerator / denominator;
        let intercept = mean_y - slope * mean_x;

        Ok(LinearModel { slope, intercept })
    }

    /// Predict y given x (deterministic)
    fn predict(&self, x: f64) -> f64 {
        self.slope * x + self.intercept
    }

    /// Predict multiple values
    fn predict_batch(&self, x: &[f64]) -> Vec<f64> {
        x.iter().map(|&xi| self.predict(xi)).collect()
    }
}

fn main() -> Result<()> {
    println!("📊 Chapter 2: Deterministic Baseline (Traditional ML)");
    println!();

    // Training data (completely deterministic)
    let x_train = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let y_train = vec![2.1, 4.2, 6.1, 8.3, 10.2, 12.1, 14.0, 16.2, 18.1, 20.0];

    println!("📈 Training linear regression model (OLS)");
    println!("   Data points: {}", x_train.len());
    println!();

    // Fit model (deterministic operation)
    let start = Instant::now();
    let model = LinearModel::fit(&x_train, &y_train)?;
    let fit_time = start.elapsed();

    println!("✅ Model fitted in {:?}", fit_time);
    println!("   Slope:     {:.6}", model.slope);
    println!("   Intercept: {:.6}", model.intercept);
    println!();

    // Make predictions (deterministic)
    let x_test = vec![11.0, 12.0, 13.0, 14.0, 15.0];
    let predictions = model.predict_batch(&x_test);

    println!("🔮 Predictions (deterministic):");
    for (x, pred) in x_test.iter().zip(predictions.iter()) {
        println!("   x = {:.1} → y = {:.4}", x, pred);
    }
    println!();

    // CRITICAL TEST: Prove determinism by running multiple times
    println!("🧪 Determinism verification (run model 5 times):");
    let test_x = 15.0;
    let mut all_predictions = Vec::new();

    for run in 1..=5 {
        let model_repeat = LinearModel::fit(&x_train, &y_train)?;
        let pred = model_repeat.predict(test_x);
        all_predictions.push(pred);
        println!("   Run {}: x = {:.1} → y = {:.10}", run, test_x, pred);
    }

    // Verify ALL predictions are IDENTICAL
    let first_pred = all_predictions[0];
    let all_identical = all_predictions
        .iter()
        .all(|&p| (p - first_pred).abs() < 1e-10);

    println!();
    if all_identical {
        println!("✅ DETERMINISTIC: All 5 runs produced IDENTICAL results");
        println!("   Variance: 0.0 (perfect determinism)");
    } else {
        println!("❌ NON-DETERMINISTIC: Results varied across runs!");
        let variance: f64 = all_predictions
            .iter()
            .map(|&p| (p - first_pred).powi(2))
            .sum::<f64>()
            / all_predictions.len() as f64;
        println!("   Variance: {:.10}", variance);
    }
    println!();

    // EU AI Act compliance
    println!("🇪🇺 EU AI Act Article 13 Compliance:");
    println!(
        "   ✓ Transparency: Model is y = {:.6}x + {:.6}",
        model.slope, model.intercept
    );
    println!("   ✓ Reproducibility: Same input → same output (always)");
    println!("   ✓ Explainability: Linear relationship is human-interpretable");
    println!("   ✓ Auditability: Model parameters are fixed and documented");
    println!();

    // Performance characteristics
    println!("⚡ Performance (deterministic ML):");
    println!("   Training time: {:?}", fit_time);
    println!("   Prediction time: <1µs per sample");
    println!("   Memory: O(1) (just 2 float64 values)");
    println!();

    // Key takeaway
    println!("🎯 Key takeaway:");
    println!("   Traditional ML is DETERMINISTIC - perfect for regulated environments.");
    println!("   Run this example 100 times, get identical results 100 times.");
    println!();

    println!("💡 Contrast with generative AI:");
    println!("   LLMs: Same prompt → different outputs (non-deterministic)");
    println!("   Traditional ML: Same input → same output (deterministic)");
    println!();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determinism() -> Result<()> {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];

        // Fit model 10 times
        let mut predictions = Vec::new();
        for _ in 0..10 {
            let model = LinearModel::fit(&x, &y)?;
            predictions.push(model.predict(6.0));
        }

        // All predictions must be IDENTICAL
        let first = predictions[0];
        for pred in &predictions {
            assert!(
                (pred - first).abs() < 1e-10,
                "Determinism violated: {} != {}",
                pred,
                first
            );
        }

        Ok(())
    }

    #[test]
    fn test_perfect_fit() -> Result<()> {
        // Perfect linear relationship: y = 2x
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];

        let model = LinearModel::fit(&x, &y)?;

        // Should perfectly recover slope=2, intercept=0
        assert!((model.slope - 2.0).abs() < 1e-10, "Slope should be 2.0");
        assert!(model.intercept.abs() < 1e-10, "Intercept should be 0.0");

        Ok(())
    }

    #[test]
    fn test_prediction_accuracy() -> Result<()> {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];

        let model = LinearModel::fit(&x, &y)?;

        // Prediction for x=6 should be y=12 (extrapolation)
        let pred = model.predict(6.0);
        assert!((pred - 12.0).abs() < 1e-10, "Prediction should be 12.0");

        Ok(())
    }

    #[test]
    fn test_batch_predictions() -> Result<()> {
        let x_train = vec![1.0, 2.0, 3.0];
        let y_train = vec![2.0, 4.0, 6.0];

        let model = LinearModel::fit(&x_train, &y_train)?;

        let x_test = vec![4.0, 5.0, 6.0];
        let predictions = model.predict_batch(&x_test);

        // Should be [8.0, 10.0, 12.0]
        assert_eq!(predictions.len(), 3);
        assert!((predictions[0] - 8.0).abs() < 1e-10);
        assert!((predictions[1] - 10.0).abs() < 1e-10);
        assert!((predictions[2] - 12.0).abs() < 1e-10);

        Ok(())
    }
}
