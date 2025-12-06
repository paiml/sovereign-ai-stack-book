/// Chapter 13: realizar - Inference Engine
///
/// **CLAIM:** realizar provides deterministic ML inference:
/// - Reproducible predictions
/// - Batched inference support
/// - Type-safe model loading
///
/// **VALIDATION:** `make run-ch13`
use anyhow::Result;

/// Inference configuration
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct InferenceConfig {
    batch_size: usize,
    num_threads: usize,
    precision: Precision,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
enum Precision {
    F32,
    F64,
}

impl Default for InferenceConfig {
    fn default() -> Self {
        Self {
            batch_size: 32,
            num_threads: 4,
            precision: Precision::F64,
        }
    }
}

/// A trained model ready for inference
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Model {
    weights: Vec<f64>,
    bias: f64,
    config: InferenceConfig,
}

impl Model {
    /// Load model with given weights
    fn new(weights: Vec<f64>, bias: f64) -> Self {
        Self {
            weights,
            bias,
            config: InferenceConfig::default(),
        }
    }

    /// Configure inference settings
    #[allow(dead_code)]
    fn with_config(mut self, config: InferenceConfig) -> Self {
        self.config = config;
        self
    }

    /// Single prediction
    fn predict(&self, x: &[f64]) -> f64 {
        let sum: f64 = self
            .weights
            .iter()
            .zip(x.iter())
            .map(|(w, xi)| w * xi)
            .sum();
        sum + self.bias
    }

    /// Batch prediction for efficiency
    fn predict_batch(&self, batch: &[Vec<f64>]) -> Vec<f64> {
        batch.iter().map(|x| self.predict(x)).collect()
    }

    /// Prediction with confidence bounds
    fn predict_with_bounds(&self, x: &[f64], uncertainty: f64) -> PredictionResult {
        let prediction = self.predict(x);
        PredictionResult {
            value: prediction,
            lower_bound: prediction - uncertainty,
            upper_bound: prediction + uncertainty,
        }
    }
}

/// Prediction result with uncertainty bounds
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct PredictionResult {
    value: f64,
    lower_bound: f64,
    upper_bound: f64,
}

impl PredictionResult {
    fn contains(&self, target: f64) -> bool {
        target >= self.lower_bound && target <= self.upper_bound
    }
}

/// Inference engine with model management
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

    fn get_model(&self, name: &str) -> Option<&Model> {
        self.models.iter().find(|(n, _)| n == name).map(|(_, m)| m)
    }

    fn predict(&self, model_name: &str, x: &[f64]) -> Option<f64> {
        self.get_model(model_name).map(|m| m.predict(x))
    }

    fn model_count(&self) -> usize {
        self.models.len()
    }
}

/// Demonstrate basic inference
fn basic_inference_demo() {
    println!("🔮 Basic Inference");
    println!();

    // Pre-trained model: y = 2x + 1
    let model = Model::new(vec![2.0], 1.0);

    println!("   Model: y = 2x + 1");
    println!("   Weights: {:?}, Bias: {}", model.weights, model.bias);
    println!();

    let test_inputs: Vec<f64> = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
    println!("   {:>6} │ {:>10}", "x", "y_pred");
    println!("   ───────┼───────────");

    for x in &test_inputs {
        let pred = model.predict(&[*x]);
        println!("   {:>6.1} │ {:>10.4}", x, pred);
    }
    println!();
}

/// Demonstrate batch inference
fn batch_inference_demo() {
    println!("📦 Batch Inference");
    println!();

    let model = Model::new(vec![2.0, 3.0], 1.0);
    println!("   Model: y = 2*x1 + 3*x2 + 1");
    println!();

    let batch: Vec<Vec<f64>> = vec![vec![1.0, 1.0], vec![2.0, 2.0], vec![3.0, 3.0]];

    let predictions = model.predict_batch(&batch);

    println!("   {:>8} │ {:>10}", "Input", "Prediction");
    println!("   ─────────┼───────────");

    for (input, pred) in batch.iter().zip(predictions.iter()) {
        println!("   [{:.1}, {:.1}] │ {:>10.4}", input[0], input[1], pred);
    }
    println!();
}

/// Demonstrate determinism
fn determinism_demo() {
    println!("🔁 Inference Determinism");
    println!();

    let model = Model::new(vec![2.5, 1.5, 0.5], 0.75);
    let input = vec![1.0, 2.0, 3.0];

    let mut results = Vec::new();

    for run in 1..=5 {
        let pred = model.predict(&input);
        results.push(pred);
        println!("   Run {}: prediction = {:.10}", run, pred);
    }

    let first = results[0];
    let all_identical = results.iter().all(|&r| (r - first).abs() < 1e-15);

    println!();
    if all_identical {
        println!("   ✅ DETERMINISTIC: All inference runs identical");
    } else {
        println!("   ❌ Non-deterministic: Results varied!");
    }
    println!();
}

/// Demonstrate prediction with uncertainty
fn uncertainty_demo() {
    println!("📊 Predictions with Uncertainty");
    println!();

    let model = Model::new(vec![2.0], 1.0);
    let uncertainty = 0.5;

    println!("   Model: y = 2x + 1 (uncertainty: ±{:.1})", uncertainty);
    println!();

    let test_cases = vec![
        (1.0, 3.0),  // Expected: 3, actual target: 3
        (2.0, 5.0),  // Expected: 5, actual target: 5
        (3.0, 6.5),  // Expected: 7, actual target: 6.5 (within bounds)
        (4.0, 10.0), // Expected: 9, actual target: 10 (outside bounds)
    ];

    println!(
        "   {:>4} │ {:>8} │ {:>12} │ {:>6}",
        "x", "Target", "Bounds", "Hit?"
    );
    println!("   ─────┼──────────┼──────────────┼───────");

    for (x, target) in test_cases {
        let result = model.predict_with_bounds(&[x], uncertainty);
        let in_bounds = if result.contains(target) {
            "✅"
        } else {
            "❌"
        };
        println!(
            "   {:>4.1} │ {:>8.2} │ [{:.2}, {:.2}] │ {}",
            x, target, result.lower_bound, result.upper_bound, in_bounds
        );
    }
    println!();
}

/// Demonstrate inference engine
fn engine_demo() {
    println!("🏭 Inference Engine");
    println!();

    let mut engine = InferenceEngine::new();

    // Register multiple models
    engine.register_model("linear", Model::new(vec![2.0], 1.0));
    engine.register_model("quadratic_approx", Model::new(vec![1.0, 1.0], 0.0));
    engine.register_model("classifier", Model::new(vec![0.5, -0.5], 0.0));

    println!("   Registered {} models", engine.model_count());
    println!();

    let test_input = vec![2.0];
    let test_input2 = vec![1.0, 2.0];

    println!("   Model predictions for x=2.0:");
    if let Some(pred) = engine.predict("linear", &test_input) {
        println!("   - linear: {:.4}", pred);
    }
    if let Some(pred) = engine.predict("quadratic_approx", &test_input2) {
        println!("   - quadratic_approx: {:.4}", pred);
    }
    if let Some(pred) = engine.predict("classifier", &test_input2) {
        println!("   - classifier: {:.4}", pred);
    }
    println!();
}

/// EU AI Act compliance
fn eu_compliance() {
    println!("🇪🇺 EU AI Act Compliance");
    println!();

    println!("   Article 10 (Data Governance):");
    println!("   ├─ Model weights fully specified");
    println!("   ├─ No external model loading");
    println!("   └─ Inference data stays local");
    println!();

    println!("   Article 13 (Transparency):");
    println!("   ├─ Predictions fully explainable");
    println!("   ├─ Uncertainty bounds provided");
    println!("   └─ Model architecture visible");
    println!();

    println!("   Article 15 (Robustness):");
    println!("   ├─ Deterministic predictions");
    println!("   ├─ Type-safe operations");
    println!("   └─ Batch processing reliable");
    println!();
}

fn main() -> Result<()> {
    println!("🎯 Chapter 13: realizar - Inference Engine");
    println!();
    println!("Deterministic, reproducible ML inference.");
    println!();
    println!("{}", "─".repeat(70));
    println!();

    basic_inference_demo();
    println!("{}", "─".repeat(70));
    println!();

    batch_inference_demo();
    println!("{}", "─".repeat(70));
    println!();

    determinism_demo();
    println!("{}", "─".repeat(70));
    println!();

    uncertainty_demo();
    println!("{}", "─".repeat(70));
    println!();

    engine_demo();
    println!("{}", "─".repeat(70));
    println!();

    eu_compliance();
    println!("{}", "─".repeat(70));
    println!();

    println!("🎯 Key Takeaways:");
    println!("   1. Deterministic inference: same input → same output");
    println!("   2. Batch processing for efficiency");
    println!("   3. Uncertainty quantification");
    println!("   4. Model registry for multi-model systems");
    println!();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_creation() {
        let model = Model::new(vec![1.0, 2.0], 0.5);
        assert_eq!(model.weights.len(), 2);
        assert_eq!(model.bias, 0.5);
    }

    #[test]
    fn test_single_prediction() {
        let model = Model::new(vec![2.0], 1.0);
        let pred = model.predict(&[3.0]);
        // y = 2*3 + 1 = 7
        assert!((pred - 7.0).abs() < 1e-10);
    }

    #[test]
    fn test_multi_feature_prediction() {
        let model = Model::new(vec![2.0, 3.0], 1.0);
        let pred = model.predict(&[1.0, 2.0]);
        // y = 2*1 + 3*2 + 1 = 9
        assert!((pred - 9.0).abs() < 1e-10);
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

        assert!((result.value - 5.0).abs() < 1e-10);
        assert!((result.lower_bound - 4.0).abs() < 1e-10);
        assert!((result.upper_bound - 6.0).abs() < 1e-10);
        assert!(result.contains(5.0));
        assert!(result.contains(4.5));
        assert!(!result.contains(3.0));
    }

    #[test]
    fn test_inference_engine() {
        let mut engine = InferenceEngine::new();
        engine.register_model("test", Model::new(vec![2.0], 1.0));

        assert_eq!(engine.model_count(), 1);
        assert!(engine.get_model("test").is_some());
        assert!(engine.get_model("missing").is_none());

        let pred = engine.predict("test", &[3.0]);
        assert!(pred.is_some());
        assert!((pred.unwrap() - 7.0).abs() < 1e-10);
    }

    #[test]
    fn test_inference_determinism() {
        let model = Model::new(vec![1.5, 2.5], 0.5);
        let input = vec![1.0, 2.0];

        let mut results = Vec::new();
        for _ in 0..10 {
            results.push(model.predict(&input));
        }

        let first = results[0];
        assert!(
            results.iter().all(|&r| (r - first).abs() < 1e-15),
            "Inference must be deterministic"
        );
    }

    #[test]
    fn test_config() {
        let config = InferenceConfig {
            batch_size: 64,
            num_threads: 8,
            precision: Precision::F32,
        };

        let model = Model::new(vec![1.0], 0.0).with_config(config);
        assert_eq!(model.config.batch_size, 64);
        assert_eq!(model.config.num_threads, 8);
        assert_eq!(model.config.precision, Precision::F32);
    }
}
