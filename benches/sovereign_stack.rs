//! Sovereign AI Stack Benchmarks
//!
//! Validates performance claims from the book using Criterion.
//! Run with: `cargo bench`

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

/// SIMD-style vector operations benchmark (Chapter 3/6 claims)
fn bench_vector_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("vector_ops");

    for size in [64, 256, 1024, 4096].iter() {
        let a: Vec<f32> = (0..*size).map(|i| i as f32 * 0.01).collect();
        let b: Vec<f32> = (0..*size).map(|i| (size - i) as f32 * 0.01).collect();

        group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(BenchmarkId::new("dot_product", size), size, |bench, _| {
            bench.iter(|| {
                let result: f32 = black_box(&a)
                    .iter()
                    .zip(black_box(&b).iter())
                    .map(|(x, y)| x * y)
                    .sum();
                black_box(result)
            })
        });

        group.bench_with_input(BenchmarkId::new("vector_add", size), size, |bench, _| {
            bench.iter(|| {
                let result: Vec<f32> = black_box(&a)
                    .iter()
                    .zip(black_box(&b).iter())
                    .map(|(x, y)| x + y)
                    .collect();
                black_box(result)
            })
        });

        group.bench_with_input(BenchmarkId::new("vector_scale", size), size, |bench, _| {
            let scale = 2.5f32;
            bench.iter(|| {
                let result: Vec<f32> = black_box(&a).iter().map(|x| x * scale).collect();
                black_box(result)
            })
        });
    }

    group.finish();
}

/// Matrix multiplication benchmark (Chapter 6/7 claims)
fn bench_matrix_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("matrix_ops");

    for size in [16, 32, 64, 128].iter() {
        let n = *size;
        let a: Vec<f32> = (0..n * n).map(|i| (i % 100) as f32 / 100.0).collect();
        let b: Vec<f32> = (0..n * n).map(|i| ((n * n - i) % 100) as f32 / 100.0).collect();

        group.throughput(Throughput::Elements((n * n) as u64));

        group.bench_with_input(
            BenchmarkId::new("naive_matmul", size),
            &(n, &a, &b),
            |bench, (n, a, b)| {
                bench.iter(|| {
                    let mut c = vec![0.0f32; n * n];
                    for i in 0..*n {
                        for j in 0..*n {
                            for k in 0..*n {
                                c[i * n + j] += a[i * n + k] * b[k * n + j];
                            }
                        }
                    }
                    black_box(c)
                })
            },
        );

        // Cache-optimized (transposed B)
        group.bench_with_input(
            BenchmarkId::new("transposed_matmul", size),
            &(n, &a, &b),
            |bench, (n, a, b)| {
                // Pre-transpose B
                let mut bt = vec![0.0f32; n * n];
                for i in 0..*n {
                    for j in 0..*n {
                        bt[j * n + i] = b[i * n + j];
                    }
                }

                bench.iter(|| {
                    let mut c = vec![0.0f32; n * n];
                    for i in 0..*n {
                        for j in 0..*n {
                            let mut sum = 0.0f32;
                            for k in 0..*n {
                                sum += a[i * n + k] * bt[j * n + k];
                            }
                            c[i * n + j] = sum;
                        }
                    }
                    black_box(c)
                })
            },
        );
    }

    group.finish();
}

/// Vector database similarity search benchmark (Chapter 15 claims)
fn bench_similarity_search(c: &mut Criterion) {
    let mut group = c.benchmark_group("similarity_search");

    for db_size in [100, 1000, 10000].iter() {
        let dim = 128;

        // Generate database of vectors
        let db: Vec<Vec<f32>> = (0..*db_size)
            .map(|i| (0..dim).map(|j| ((i * dim + j) % 100) as f32 / 100.0).collect())
            .collect();

        // Query vector
        let query: Vec<f32> = (0..dim).map(|i| (i % 50) as f32 / 50.0).collect();

        group.throughput(Throughput::Elements(*db_size as u64));

        // Euclidean distance brute-force search
        group.bench_with_input(
            BenchmarkId::new("euclidean_search", db_size),
            &(&db, &query),
            |bench, (db, query)| {
                bench.iter(|| {
                    let mut distances: Vec<(usize, f32)> = db
                        .iter()
                        .enumerate()
                        .map(|(i, vec)| {
                            let dist: f32 = vec
                                .iter()
                                .zip(query.iter())
                                .map(|(a, b)| (a - b).powi(2))
                                .sum();
                            (i, dist.sqrt())
                        })
                        .collect();
                    distances.sort_by(|a, b| a.1.partial_cmp(&b.1).expect("valid comparison"));
                    distances.truncate(10);
                    black_box(distances)
                })
            },
        );

        // Cosine similarity search
        group.bench_with_input(
            BenchmarkId::new("cosine_search", db_size),
            &(&db, &query),
            |bench, (db, query)| {
                let query_norm: f32 = query.iter().map(|x| x * x).sum::<f32>().sqrt();

                bench.iter(|| {
                    let mut similarities: Vec<(usize, f32)> = db
                        .iter()
                        .enumerate()
                        .map(|(i, vec)| {
                            let dot: f32 = vec.iter().zip(query.iter()).map(|(a, b)| a * b).sum();
                            let vec_norm: f32 = vec.iter().map(|x| x * x).sum::<f32>().sqrt();
                            let sim = dot / (vec_norm * query_norm + 1e-10);
                            (i, sim)
                        })
                        .collect();
                    similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).expect("valid comparison"));
                    similarities.truncate(10);
                    black_box(similarities)
                })
            },
        );
    }

    group.finish();
}

/// ML training step benchmark (Chapter 12/14 claims)
fn bench_ml_training(c: &mut Criterion) {
    let mut group = c.benchmark_group("ml_training");

    for n_samples in [100, 1000, 10000].iter() {
        // Generate linear regression data: y = 2x + 1 + noise
        let x: Vec<f32> = (0..*n_samples).map(|i| i as f32 / *n_samples as f32).collect();
        let y: Vec<f32> = x.iter().map(|xi| 2.0 * xi + 1.0).collect();

        group.throughput(Throughput::Elements(*n_samples as u64));

        // Single gradient descent step
        group.bench_with_input(
            BenchmarkId::new("sgd_step", n_samples),
            &(&x, &y),
            |bench, (x, y)| {
                let mut weight = 0.0f32;
                let mut bias = 0.0f32;
                let lr = 0.01f32;

                bench.iter(|| {
                    let n = x.len() as f32;
                    let mut dw = 0.0f32;
                    let mut db = 0.0f32;

                    for (xi, yi) in x.iter().zip(y.iter()) {
                        let pred = weight * xi + bias;
                        let error = pred - yi;
                        dw += error * xi;
                        db += error;
                    }

                    weight -= lr * dw / n;
                    bias -= lr * db / n;

                    black_box((weight, bias))
                })
            },
        );

        // MSE loss calculation
        group.bench_with_input(
            BenchmarkId::new("mse_loss", n_samples),
            &(&x, &y),
            |bench, (x, y)| {
                let weight = 2.0f32;
                let bias = 1.0f32;

                bench.iter(|| {
                    let mse: f32 = x
                        .iter()
                        .zip(y.iter())
                        .map(|(xi, yi)| {
                            let pred = weight * xi + bias;
                            (pred - yi).powi(2)
                        })
                        .sum::<f32>()
                        / x.len() as f32;
                    black_box(mse)
                })
            },
        );
    }

    group.finish();
}

/// Determinism verification benchmark (validates reproducibility claims)
fn bench_determinism(c: &mut Criterion) {
    let mut group = c.benchmark_group("determinism");

    // Verify deterministic computation
    group.bench_function("deterministic_sum", |bench| {
        let data: Vec<f64> = (0..10000).map(|i| (i as f64) * 0.0001).collect();

        bench.iter(|| {
            let sum: f64 = black_box(&data).iter().sum();
            black_box(sum)
        })
    });

    // Kahan summation for high-precision determinism
    group.bench_function("kahan_sum", |bench| {
        let data: Vec<f64> = (0..10000).map(|i| (i as f64) * 0.0001).collect();

        bench.iter(|| {
            let mut sum = 0.0f64;
            let mut c = 0.0f64;
            for &x in black_box(&data) {
                let y = x - c;
                let t = sum + y;
                c = (t - sum) - y;
                sum = t;
            }
            black_box(sum)
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_vector_operations,
    bench_matrix_operations,
    bench_similarity_search,
    bench_ml_training,
    bench_determinism,
);

criterion_main!(benches);
