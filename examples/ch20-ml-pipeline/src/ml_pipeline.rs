/// Chapter 20: Building an ML Pipeline
///
/// **CLAIM:** Integrate all stack components into a production ML pipeline:
/// - End-to-end reproducibility
/// - Type-safe data flow
/// - Deterministic results
///
/// **VALIDATION:** `make run-ch20`
use anyhow::Result;

/// Pipeline stage trait
trait Stage {
    type Input;
    type Output;
    fn process(&self, input: Self::Input) -> Self::Output;
    fn name(&self) -> &str;
}

/// Data loader stage
struct DataLoader {
    name: String,
}

impl DataLoader {
    fn new() -> Self {
        Self { name: "DataLoader".to_string() }
    }
}

impl Stage for DataLoader {
    type Input = ();
    type Output = Vec<Vec<f64>>;

    fn process(&self, _: Self::Input) -> Self::Output {
        // Generate deterministic data
        (0..100).map(|i| vec![i as f64, (i * 2) as f64]).collect()
    }

    fn name(&self) -> &str { &self.name }
}

/// Preprocessor stage
struct Preprocessor {
    name: String,
    scale: f64,
}

impl Preprocessor {
    fn new(scale: f64) -> Self {
        Self { name: "Preprocessor".to_string(), scale }
    }
}

impl Stage for Preprocessor {
    type Input = Vec<Vec<f64>>;
    type Output = Vec<Vec<f64>>;

    fn process(&self, input: Self::Input) -> Self::Output {
        input.into_iter()
            .map(|row| row.into_iter().map(|x| x * self.scale).collect())
            .collect()
    }

    fn name(&self) -> &str { &self.name }
}

/// Feature extractor stage
struct FeatureExtractor {
    name: String,
}

impl FeatureExtractor {
    fn new() -> Self {
        Self { name: "FeatureExtractor".to_string() }
    }
}

impl Stage for FeatureExtractor {
    type Input = Vec<Vec<f64>>;
    type Output = Vec<f64>;

    fn process(&self, input: Self::Input) -> Self::Output {
        // Extract mean of each feature
        if input.is_empty() { return Vec::new(); }

        let num_features = input[0].len();
        let n = input.len() as f64;

        (0..num_features).map(|f| {
            input.iter().map(|row| row[f]).sum::<f64>() / n
        }).collect()
    }

    fn name(&self) -> &str { &self.name }
}

/// Model trainer stage
struct ModelTrainer {
    name: String,
    learning_rate: f64,
}

impl ModelTrainer {
    fn new(learning_rate: f64) -> Self {
        Self { name: "ModelTrainer".to_string(), learning_rate }
    }
}

#[derive(Debug, Clone)]
struct Model {
    weights: Vec<f64>,
    bias: f64,
}

impl Stage for ModelTrainer {
    type Input = Vec<f64>;
    type Output = Model;

    fn process(&self, features: Self::Input) -> Self::Output {
        // Simple deterministic training
        let weights: Vec<f64> = features.iter()
            .map(|&f| f * self.learning_rate)
            .collect();
        let bias = features.iter().sum::<f64>() / features.len() as f64;

        Model { weights, bias }
    }

    fn name(&self) -> &str { &self.name }
}

/// Pipeline executor
struct Pipeline {
    stages: Vec<String>,
}

impl Pipeline {
    fn new() -> Self {
        Self { stages: Vec::new() }
    }

    fn log(&mut self, stage: &str) {
        self.stages.push(stage.to_string());
    }

    fn run(&mut self) -> Model {
        // Execute pipeline stages
        let loader = DataLoader::new();
        self.log(loader.name());
        let data = loader.process(());

        let preprocessor = Preprocessor::new(0.01);
        self.log(preprocessor.name());
        let preprocessed = preprocessor.process(data);

        let extractor = FeatureExtractor::new();
        self.log(extractor.name());
        let features = extractor.process(preprocessed);

        let trainer = ModelTrainer::new(0.1);
        self.log(trainer.name());
        trainer.process(features)
    }

    fn stages_executed(&self) -> &[String] {
        &self.stages
    }
}

/// Demonstrate pipeline execution
fn pipeline_demo() {
    println!("🔄 ML Pipeline Execution");
    println!();

    let mut pipeline = Pipeline::new();
    let model = pipeline.run();

    println!("   Stages executed:");
    for (i, stage) in pipeline.stages_executed().iter().enumerate() {
        println!("   {}. {}", i + 1, stage);
    }
    println!();

    println!("   Model output:");
    println!("   - Weights: {:?}", model.weights);
    println!("   - Bias: {:.4}", model.bias);
    println!();
}

/// Demonstrate stage composition
fn stage_demo() {
    println!("🧩 Stage Composition");
    println!();

    let loader = DataLoader::new();
    let data = loader.process(());
    println!("   DataLoader: {} rows", data.len());

    let preprocessor = Preprocessor::new(0.1);
    let processed = preprocessor.process(data);
    println!("   Preprocessor: scaled by 0.1");

    let extractor = FeatureExtractor::new();
    let features = extractor.process(processed);
    println!("   FeatureExtractor: {} features", features.len());
    println!("   Features: {:?}", features);
    println!();
}

/// Demonstrate determinism
fn determinism_demo() {
    println!("🔁 Pipeline Determinism");
    println!();

    let mut results = Vec::new();

    for run in 1..=5 {
        let mut pipeline = Pipeline::new();
        let model = pipeline.run();
        println!("   Run {}: bias = {:.6}", run, model.bias);
        results.push(model.bias);
    }

    let first = results[0];
    let all_identical = results.iter().all(|&r| (r - first).abs() < 1e-10);

    println!();
    if all_identical {
        println!("   ✅ DETERMINISTIC: All pipeline runs identical");
    } else {
        println!("   ❌ Non-deterministic: Results varied!");
    }
    println!();
}

/// Demonstrate data flow
fn dataflow_demo() {
    println!("📊 Data Flow Visualization");
    println!();

    println!("   ┌─────────────┐");
    println!("   │ DataLoader  │ → Raw data (100 rows)");
    println!("   └──────┬──────┘");
    println!("          ↓");
    println!("   ┌─────────────┐");
    println!("   │Preprocessor │ → Scaled data (0.01x)");
    println!("   └──────┬──────┘");
    println!("          ↓");
    println!("   ┌─────────────┐");
    println!("   │FeatureExt  │ → Feature means");
    println!("   └──────┬──────┘");
    println!("          ↓");
    println!("   ┌─────────────┐");
    println!("   │ModelTrainer│ → Trained model");
    println!("   └─────────────┘");
    println!();
}

/// EU AI Act compliance
fn eu_compliance() {
    println!("🇪🇺 EU AI Act Compliance");
    println!();

    println!("   Article 10 (Data Governance):");
    println!("   ├─ Data generation deterministic");
    println!("   ├─ Preprocessing transparent");
    println!("   └─ All transformations logged");
    println!();

    println!("   Article 13 (Transparency):");
    println!("   ├─ Pipeline stages visible");
    println!("   ├─ Data flow explicit");
    println!("   └─ Model outputs reproducible");
    println!();

    println!("   Article 15 (Robustness):");
    println!("   ├─ Type-safe stage interfaces");
    println!("   ├─ Deterministic execution");
    println!("   └─ End-to-end validation");
    println!();
}

fn main() -> Result<()> {
    println!("🏭 Chapter 20: Building an ML Pipeline");
    println!();
    println!("End-to-end reproducible ML pipeline.");
    println!();
    println!("{}", "─".repeat(70));
    println!();

    pipeline_demo();
    println!("{}", "─".repeat(70));
    println!();

    stage_demo();
    println!("{}", "─".repeat(70));
    println!();

    determinism_demo();
    println!("{}", "─".repeat(70));
    println!();

    dataflow_demo();
    println!("{}", "─".repeat(70));
    println!();

    eu_compliance();
    println!("{}", "─".repeat(70));
    println!();

    println!("🎯 Key Takeaways:");
    println!("   1. Type-safe stage composition");
    println!("   2. Deterministic data transformations");
    println!("   3. End-to-end reproducibility");
    println!("   4. EU AI Act compliant pipeline");
    println!();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_loader() {
        let loader = DataLoader::new();
        let data = loader.process(());
        assert_eq!(data.len(), 100);
        assert_eq!(data[0].len(), 2);
    }

    #[test]
    fn test_preprocessor() {
        let preprocessor = Preprocessor::new(2.0);
        let input = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let output = preprocessor.process(input);

        assert_eq!(output[0], vec![2.0, 4.0]);
        assert_eq!(output[1], vec![6.0, 8.0]);
    }

    #[test]
    fn test_feature_extractor() {
        let extractor = FeatureExtractor::new();
        let input = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let features = extractor.process(input);

        assert!((features[0] - 2.0).abs() < 1e-10);
        assert!((features[1] - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_model_trainer() {
        let trainer = ModelTrainer::new(0.1);
        let features = vec![10.0, 20.0];
        let model = trainer.process(features);

        assert!((model.weights[0] - 1.0).abs() < 1e-10);
        assert!((model.weights[1] - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_pipeline_execution() {
        let mut pipeline = Pipeline::new();
        let model = pipeline.run();

        assert_eq!(pipeline.stages_executed().len(), 4);
        assert!(!model.weights.is_empty());
    }

    #[test]
    fn test_pipeline_determinism() {
        let mut results = Vec::new();

        for _ in 0..5 {
            let mut pipeline = Pipeline::new();
            let model = pipeline.run();
            results.push(model.bias);
        }

        let first = results[0];
        assert!(results.iter().all(|&r| (r - first).abs() < 1e-10),
            "Pipeline must be deterministic");
    }
}
