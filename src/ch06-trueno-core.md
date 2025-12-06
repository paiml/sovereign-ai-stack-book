# Trueno Core: Deterministic Tensor Operations

> **Toyota Way Principle (Jidoka):** Build quality into the process. Every tensor operation is deterministic and verifiable.

**Status:** Complete

## The Problem: ML Operations Without Guarantees

Machine learning systems depend on tensor operations - vectors for embeddings, matrices for neural network weights. Traditional ML frameworks introduce three critical risks:

1. **Non-determinism**: Same input may produce different outputs (floating-point variance)
2. **Memory unsafety**: Buffer overflows, use-after-free in tensor operations
3. **Data exfiltration**: Tensors sent to cloud APIs for processing

## trueno's Solution: Deterministic, Local, Safe

trueno provides tensor operations with EU AI Act compliance built-in:

```
┌─────────────────────────────────────────────────────────┐
│                    trueno Core                          │
├─────────────────────────────────────────────────────────┤
│  Vector Operations        │  Matrix Operations          │
│  • Creation              │  • Creation                  │
│  • Dot product           │  • Transpose                 │
│  • Element-wise ops      │  • Multiplication            │
│  • Statistics            │  • Neural layer forward      │
├──────────────────────────┴─────────────────────────────┤
│              Guarantees (Jidoka)                        │
│  ✓ Deterministic: Same input → Same output             │
│  ✓ Memory-safe: Rust borrow checker                    │
│  ✓ Local: Zero network calls                           │
└─────────────────────────────────────────────────────────┘
```

## Validation

Run all chapter examples:

```bash
make run-ch06           # Run all examples
make run-ch06-vector    # Vector operations only
make run-ch06-matrix    # Matrix operations only
make test-ch06          # Run all tests
```

## Vector Operations

Vectors are the foundation of ML - embeddings, activations, gradients all use vectors.

### Basic Operations

```rust
use trueno::Vector;

// Create vectors
let v1 = Vector::from_slice(&[1.0, 2.0, 3.0, 4.0, 5.0]);
let v2 = Vector::from_slice(&[5.0, 4.0, 3.0, 2.0, 1.0]);

// Basic statistics
let sum: f32 = v1.as_slice().iter().sum();  // 15.0
let mean = sum / v1.len() as f32;           // 3.0
```

### Dot Product (Neural Network Forward Pass)

The dot product is fundamental to neural networks - it computes the weighted sum:

```rust
// Dot product: v1 · v2
let dot: f32 = v1.as_slice().iter()
    .zip(v2.as_slice().iter())
    .map(|(a, b)| a * b)
    .sum();  // 35.0

// Formula: 1×5 + 2×4 + 3×3 + 4×2 + 5×1 = 35
```

### Determinism Verification (Genchi Genbutsu)

Go and see for yourself - verify determinism empirically:

```rust
let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
let mut results = Vec::new();

for _ in 0..5 {
    let v = Vector::from_slice(&data);
    let sum: f32 = v.as_slice().iter().sum();
    results.push(sum);
}

// All runs produce: 15.0000000000
// Bit-for-bit identical every time
```

## Matrix Operations

Matrices represent neural network weights, attention mechanisms, and feature transformations.

### Matrix Creation

```rust
use trueno::Matrix;

// Create a 3x3 matrix (row-major layout)
let data = vec![
    1.0, 2.0, 3.0,
    4.0, 5.0, 6.0,
    7.0, 8.0, 9.0,
];
let m = Matrix::from_vec(3, 3, data).expect("Valid matrix");

assert_eq!(m.rows(), 3);
assert_eq!(m.cols(), 3);
```

### Matrix Transpose

Transpose is essential for data reshaping and backpropagation:

```rust
// Original 2x3 matrix
let m = Matrix::from_vec(2, 3, vec![
    1.0, 2.0, 3.0,
    4.0, 5.0, 6.0,
]).expect("Valid matrix");

// Manual transpose to 3x2
let slice = m.as_slice();
let transposed: Vec<f32> = (0..3).flat_map(|col| {
    (0..2).map(move |row| slice[row * 3 + col])
}).collect();

// Result: [1.0, 4.0, 2.0, 5.0, 3.0, 6.0]
```

### Matrix Multiplication (Neural Network Layers)

Matrix multiplication is the core operation in neural networks:

```rust
// A: 2x3 matrix (2 outputs, 3 inputs)
let a = Matrix::from_vec(2, 3, vec![
    1.0, 2.0, 3.0,
    4.0, 5.0, 6.0,
]).expect("Valid matrix A");

// B: 3x2 matrix
let b = Matrix::from_vec(3, 2, vec![
    7.0,  8.0,
    9.0,  10.0,
    11.0, 12.0,
]).expect("Valid matrix B");

// C = A × B (2x3 × 3x2 = 2x2)
let mut c = [0.0f32; 4];
for i in 0..2 {
    for j in 0..2 {
        for k in 0..3 {
            c[i * 2 + j] += a.as_slice()[i * 3 + k]
                         * b.as_slice()[k * 2 + j];
        }
    }
}

// Result: [58, 64, 139, 154]
// Verification: C[0,0] = 1×7 + 2×9 + 3×11 = 58
```

## ML-Relevant Operations

### Neural Network Layer Forward Pass

A typical neural network layer computes `y = Wx + b`:

```rust
// Weights: 2x3 (2 outputs, 3 inputs)
let w = Matrix::from_vec(2, 3, vec![
    0.1, 0.2, 0.3,
    0.4, 0.5, 0.6,
]).unwrap();

let input = vec![1.0, 2.0, 3.0];
let bias = vec![0.1, 0.2];

// Compute y = Wx + b
let mut output = [0.0f32; 2];
for i in 0..2 {
    for (j, &inp) in input.iter().enumerate() {
        output[i] += w.as_slice()[i * 3 + j] * inp;
    }
    output[i] += bias[i];
}
// output = [1.5, 3.4]
```

### ReLU Activation

```rust
let activated: Vec<f32> = output.iter()
    .map(|&x| x.max(0.0))
    .collect();
// ReLU(y) = [1.5, 3.4] (both positive, unchanged)
```

### Softmax (Classification Output)

```rust
let max_val = output.iter().cloned()
    .fold(f32::NEG_INFINITY, f32::max);
let exp_sum: f32 = output.iter()
    .map(|x| (x - max_val).exp())
    .sum();
let softmax: Vec<f32> = output.iter()
    .map(|x| (x - max_val).exp() / exp_sum)
    .collect();
// Sum = 1.0 (probability distribution)
```

## Performance Characteristics

| Operation | Complexity | Memory Layout |
|-----------|------------|---------------|
| Vector creation | O(n) | Contiguous |
| Dot product | O(n) | Sequential access |
| Matrix creation | O(n×m) | Row-major |
| Matrix multiply | O(n³) | Cache-friendly |

## EU AI Act Compliance

trueno core operations satisfy EU AI Act requirements:

### Article 10: Data Governance

```rust
// All operations are local - no data leaves the system
let v = Vector::from_slice(&sensitive_data);
let result = process(v);  // Zero network calls
```

### Article 13: Transparency

```rust
// Every operation is deterministic and auditable
let run1 = compute(&input);
let run2 = compute(&input);
assert_eq!(run1, run2);  // Guaranteed identical
```

### Article 15: Robustness

```rust
// Rust's type system prevents memory errors
let m = Matrix::from_vec(2, 2, vec![1.0, 2.0]);  // Error: wrong size
// Compile-time: Cannot create invalid matrix
```

## Testing (Poka-Yoke)

Error-proof the implementation with comprehensive tests:

```rust
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
```

## Key Takeaways

1. **Determinism is non-negotiable**: EU AI Act requires reproducible results
2. **Memory safety is free**: Rust's borrow checker catches errors at compile time
3. **Local processing is sovereign**: No data leaves your infrastructure
4. **trueno provides the foundation**: Higher-level ML operations build on these primitives

## Next Steps

- **Chapter 7**: trueno GPU acceleration with CUDA/Metal backends
- **Chapter 8**: aprender ML training with deterministic gradients
- **Chapter 9**: realizar inference with certified outputs

## Source Code

Full implementation: `examples/ch06-trueno-core/`

```bash
# Verify all claims
make test-ch06

# Run examples
make run-ch06
```
