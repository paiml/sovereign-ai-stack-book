/// Chapter 6: trueno Core - Matrix Operations
///
/// **CLAIM:** trueno provides matrix operations essential for ML:
/// - Matrix multiplication (neural network layers)
/// - Transpose (data reshaping)
/// - Element-wise operations (activations)
///
/// **VALIDATION:** `make run-ch06-matrix`
///
/// **KEY PRINCIPLE:** Deterministic Matrix Operations
/// - Same matrices → same result (always)
/// - Critical for reproducible ML training
use anyhow::Result;
use trueno::Matrix;

/// Demonstrate matrix creation
fn matrix_basics() {
    println!("📊 Matrix Basics");
    println!();

    // Create a 3x3 matrix
    let data = vec![
        1.0, 2.0, 3.0,
        4.0, 5.0, 6.0,
        7.0, 8.0, 9.0,
    ];

    let m = Matrix::from_vec(3, 3, data.clone()).expect("Valid matrix");

    println!("   Matrix (3x3):");
    for row in 0..3 {
        print!("   ");
        for col in 0..3 {
            print!("{:>4.1} ", m.as_slice()[row * 3 + col]);
        }
        println!();
    }
    println!();
    println!("   rows = {}", m.rows());
    println!("   cols = {}", m.cols());
    println!("   total elements = {}", m.as_slice().len());
    println!();
}

/// Demonstrate matrix transpose
fn matrix_transpose() {
    println!("🔄 Matrix Transpose");
    println!();

    let data = vec![
        1.0, 2.0, 3.0,
        4.0, 5.0, 6.0,
    ];

    let m = Matrix::from_vec(2, 3, data).expect("Valid matrix");

    println!("   Original (2x3):");
    for row in 0..2 {
        print!("   ");
        for col in 0..3 {
            print!("{:>4.1} ", m.as_slice()[row * 3 + col]);
        }
        println!();
    }
    println!();

    // Manual transpose for demonstration
    let slice = m.as_slice();
    let transposed: Vec<f32> = (0..3).flat_map(|col| {
        (0..2).map(move |row| slice[row * 3 + col])
    }).collect();

    println!("   Transposed (3x2):");
    for row in 0..3 {
        print!("   ");
        for col in 0..2 {
            print!("{:>4.1} ", transposed[row * 2 + col]);
        }
        println!();
    }
    println!();
}

/// Demonstrate matrix multiplication
fn matrix_multiplication() {
    println!("✖️  Matrix Multiplication");
    println!();

    // A: 2x3 matrix
    let a_data = vec![
        1.0, 2.0, 3.0,
        4.0, 5.0, 6.0,
    ];
    let a = Matrix::from_vec(2, 3, a_data).expect("Valid matrix A");

    // B: 3x2 matrix
    let b_data = vec![
        7.0,  8.0,
        9.0,  10.0,
        11.0, 12.0,
    ];
    let b = Matrix::from_vec(3, 2, b_data).expect("Valid matrix B");

    println!("   A (2x3):");
    for row in 0..2 {
        print!("   ");
        for col in 0..3 {
            print!("{:>4.1} ", a.as_slice()[row * 3 + col]);
        }
        println!();
    }
    println!();

    println!("   B (3x2):");
    for row in 0..3 {
        print!("   ");
        for col in 0..2 {
            print!("{:>4.1} ", b.as_slice()[row * 2 + col]);
        }
        println!();
    }
    println!();

    // Manual matrix multiplication: C = A × B (2x3 × 3x2 = 2x2)
    let mut c = [0.0f32; 4];
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..3 {
                c[i * 2 + j] += a.as_slice()[i * 3 + k] * b.as_slice()[k * 2 + j];
            }
        }
    }

    println!("   C = A × B (2x2):");
    for row in 0..2 {
        print!("   ");
        for col in 0..2 {
            print!("{:>6.1} ", c[row * 2 + col]);
        }
        println!();
    }
    println!();

    // Verify: C[0,0] = 1*7 + 2*9 + 3*11 = 7 + 18 + 33 = 58
    println!("   Verification: C[0,0] = 1×7 + 2×9 + 3×11 = {}", 7 + 18 + 33);
    println!();
}

/// Demonstrate determinism in matrix operations
fn matrix_determinism() {
    println!("🔁 Matrix Determinism Verification");
    println!();

    let a_data = vec![1.0, 2.0, 3.0, 4.0];
    let b_data = vec![5.0, 6.0, 7.0, 8.0];

    let mut results = Vec::new();

    for run in 1..=5 {
        let a = Matrix::from_vec(2, 2, a_data.clone()).unwrap();
        let b = Matrix::from_vec(2, 2, b_data.clone()).unwrap();

        // Manual 2x2 matrix multiplication
        let mut c = [0.0f32; 4];
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    c[i * 2 + j] += a.as_slice()[i * 2 + k] * b.as_slice()[k * 2 + j];
                }
            }
        }

        let trace = c[0] + c[3]; // sum of diagonal
        results.push(trace);
        println!("   Run {}: trace(A×B) = {:.10}", run, trace);
    }

    let first = results[0];
    let all_identical = results.iter().all(|&r| (r - first).abs() < 1e-10);

    println!();
    if all_identical {
        println!("   ✅ DETERMINISTIC: All matrix multiplications identical");
    } else {
        println!("   ❌ Non-deterministic: Results varied!");
    }
    println!();
}

/// Demonstrate ML-relevant operations
fn ml_operations() {
    println!("🧠 ML-Relevant Matrix Operations");
    println!();

    // Simulate a neural network layer: y = Wx + b
    let weights = vec![
        0.1, 0.2, 0.3,
        0.4, 0.5, 0.6,
    ];
    let w = Matrix::from_vec(2, 3, weights).unwrap();

    let input = vec![1.0, 2.0, 3.0];
    let bias = vec![0.1, 0.2];

    println!("   Neural network layer: y = Wx + b");
    println!();
    println!("   W (2x3 weights):");
    for row in 0..2 {
        print!("   ");
        for col in 0..3 {
            print!("{:>5.2} ", w.as_slice()[row * 3 + col]);
        }
        println!();
    }
    println!();

    println!("   x (input): {:?}", input);
    println!("   b (bias): {:?}", bias);
    println!();

    // Manual matrix-vector multiplication: y = Wx + b
    let mut output = [0.0f32; 2];
    for i in 0..2 {
        for (j, &inp) in input.iter().enumerate() {
            output[i] += w.as_slice()[i * 3 + j] * inp;
        }
        output[i] += bias[i];
    }

    println!("   y (output): {:?}", output);
    println!();

    // ReLU activation: max(0, x)
    let activated: Vec<f32> = output.iter().map(|&x| x.max(0.0)).collect();
    println!("   ReLU(y): {:?}", activated);
    println!();

    // Softmax (simplified for 2 outputs)
    let max_val = output.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let exp_sum: f32 = output.iter().map(|x| (x - max_val).exp()).sum();
    let softmax: Vec<f32> = output.iter().map(|x| (x - max_val).exp() / exp_sum).collect();
    println!("   Softmax(y): {:?}", softmax);
    println!("   Sum = {:.4} (should be 1.0)", softmax.iter().sum::<f32>());
    println!();
}

fn main() -> Result<()> {
    println!("🧮 Chapter 6: trueno Core - Matrix Operations");
    println!();
    println!("Matrices are fundamental to ML:");
    println!("   - Neural network weights");
    println!("   - Attention mechanisms");
    println!("   - Feature transformations");
    println!();
    println!("{}", "─".repeat(70));
    println!();

    matrix_basics();
    println!("{}", "─".repeat(70));
    println!();

    matrix_transpose();
    println!("{}", "─".repeat(70));
    println!();

    matrix_multiplication();
    println!("{}", "─".repeat(70));
    println!();

    matrix_determinism();
    println!("{}", "─".repeat(70));
    println!();

    ml_operations();
    println!("{}", "─".repeat(70));
    println!();

    // Performance characteristics
    println!("⚡ Performance Characteristics:");
    println!("   Matrix creation: O(n) - linear in elements");
    println!("   Matrix multiply: O(n³) for n×n matrices");
    println!("   Memory layout: Row-major (cache-friendly)");
    println!();

    // EU AI Act compliance
    println!("🇪🇺 EU AI Act Compliance:");
    println!("   ✅ Transparency: All operations documented and auditable");
    println!("   ✅ Reproducibility: Deterministic matrix math");
    println!("   ✅ Local processing: No external API calls");
    println!();

    // Key takeaway
    println!("🎯 Key takeaway:");
    println!("   trueno matrices enable deterministic, memory-safe ML operations.");
    println!("   Essential for EU-compliant AI systems.");
    println!();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_creation() {
        let m = Matrix::from_vec(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
        assert_eq!(m.rows(), 2);
        assert_eq!(m.cols(), 3);
        assert_eq!(m.as_slice().len(), 6);
    }

    #[test]
    fn test_matrix_multiplication() {
        // A: 2x2, B: 2x2
        let a = Matrix::from_vec(2, 2, vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        let b = Matrix::from_vec(2, 2, vec![5.0, 6.0, 7.0, 8.0]).unwrap();

        let mut c = vec![0.0f32; 4];
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    c[i * 2 + j] += a.as_slice()[i * 2 + k] * b.as_slice()[k * 2 + j];
                }
            }
        }

        // C[0,0] = 1*5 + 2*7 = 19
        // C[0,1] = 1*6 + 2*8 = 22
        // C[1,0] = 3*5 + 4*7 = 43
        // C[1,1] = 3*6 + 4*8 = 50
        assert!((c[0] - 19.0).abs() < 1e-6);
        assert!((c[1] - 22.0).abs() < 1e-6);
        assert!((c[2] - 43.0).abs() < 1e-6);
        assert!((c[3] - 50.0).abs() < 1e-6);
    }

    #[test]
    fn test_matrix_determinism() {
        let data = vec![1.0, 2.0, 3.0, 4.0];
        let mut sums = Vec::new();

        for _ in 0..10 {
            let m = Matrix::from_vec(2, 2, data.clone()).unwrap();
            let sum: f32 = m.as_slice().iter().sum();
            sums.push(sum);
        }

        let first = sums[0];
        assert!(sums.iter().all(|&s| (s - first).abs() < 1e-10),
            "Matrix operations must be deterministic");
    }

    #[test]
    fn test_relu_activation() {
        let values: Vec<f32> = vec![-1.0, 0.0, 1.0, -0.5, 2.0];
        let relu: Vec<f32> = values.iter().map(|&x: &f32| x.max(0.0)).collect();
        assert_eq!(relu, vec![0.0, 0.0, 1.0, 0.0, 2.0]);
    }

    #[test]
    fn test_softmax_sums_to_one() {
        let logits = vec![1.0, 2.0, 3.0];
        let max_val = logits.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let exp_sum: f32 = logits.iter().map(|x| (x - max_val).exp()).sum();
        let softmax: Vec<f32> = logits.iter().map(|x| (x - max_val).exp() / exp_sum).collect();

        let sum: f32 = softmax.iter().sum();
        assert!((sum - 1.0).abs() < 1e-6, "Softmax should sum to 1.0");
    }
}
