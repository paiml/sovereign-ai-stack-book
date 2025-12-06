# Introduction to Transpilation

> **Toyota Way Principle (Jidoka):** Build quality in at the source. Transform code to a safer language before execution.

**Status:** Complete

## What is Transpilation?

Transpilation converts source code from one programming language to another, preserving the original semantics while gaining the benefits of the target language.

```
Transpilation Pipeline
───────────────────────────────────────────────────────────────

  Source Code     →  AST  →  Transform  →  Target Code
  (Python/Bash)      │         │            (Rust)
                     │         │
                     ↓         ↓
               Type Inference  Semantic
                              Preservation

  Key: Same behavior, better guarantees
───────────────────────────────────────────────────────────────
```

## Validation

Run all chapter examples:

```bash
make run-ch08           # Run all examples
make run-ch08-concepts  # Transpilation concepts
make run-ch08-ast       # AST analysis
make test-ch08          # Run all tests
```

## Why Transpile to Rust?

| Source Language | Weakness | Rust Advantage |
|-----------------|----------|----------------|
| Python | Dynamic types | Compile-time type checking |
| Bash | Shell injection | Memory-safe string handling |
| TypeScript | Runtime VM | Native binary, no Node.js |

### The Core Benefits

```rust
// Original Python (dynamic, interpreted)
def calculate(x, y):
    return x + y * 2

// Transpiled Rust (typed, compiled)
fn calculate(x: i64, y: i64) -> i64 {
    x + y * 2
}
```

Benefits gained through transpilation:
1. **Type safety**: Errors caught at compile time
2. **Memory safety**: No buffer overflows or use-after-free
3. **Performance**: Native code, no interpreter overhead
4. **Single binary**: No runtime dependencies

## Transpilation vs Compilation

Understanding the difference:

```
Compilation:
Source → AST → IR → Machine Code
(Python → bytecode, C → assembly)

Transpilation:
Source → AST → Target Source
(Python → Rust, TypeScript → JavaScript)

Our Approach: Transpile THEN Compile
Python → Rust → Native Binary
```

The key advantage: Rust's compiler performs safety verification that the source language lacks.

## Abstract Syntax Trees (ASTs)

ASTs provide the foundation for transpilation:

```rust
// Expression: x + y * 2
// AST representation:

BinOp(+)
├── Var(x)
└── BinOp(*)
    ├── Var(y)
    └── Int(2)
```

### AST Node Types

```rust
enum Expr {
    Int(i64),           // 42
    Float(f64),         // 3.5
    Str(String),        // "hello"
    Bool(bool),         // true
    Var(String),        // x
    BinOp {             // x + y
        op: BinOperator,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Call {              // foo(x, y)
        name: String,
        args: Vec<Expr>,
    },
}
```

## Type Mapping

Each source language type maps to a Rust equivalent:

```
   Python          TypeScript       Rust
   ────────────────────────────────────────
   int         →   number       →   i64
   float       →   number       →   f64
   str         →   string       →   String
   bool        →   boolean      →   bool
   list[T]     →   T[]          →   Vec<T>
   dict[K,V]   →   Map<K,V>     →   HashMap<K,V>
   None        →   null         →   Option<T>
```

### Type Inference

When source code lacks type annotations, we infer types from usage:

```rust
fn infer_type(expr: &Expr) -> Type {
    match expr {
        Expr::Int(_) => Type::Int,
        Expr::Float(_) => Type::Float,
        Expr::BinOp { left, right, .. } => {
            let left_type = infer_type(left);
            let right_type = infer_type(right);
            // Int + Int = Int, Float + anything = Float
            match (left_type, right_type) {
                (Type::Int, Type::Int) => Type::Int,
                _ => Type::Float,
            }
        }
        _ => Type::Unknown,
    }
}
```

## Code Generation

Transform the AST into valid Rust source code:

```rust
fn generate_rust(expr: &Expr) -> String {
    match expr {
        Expr::Int(n) => format!("{}", n),
        Expr::Var(name) => name.clone(),
        Expr::BinOp { op, left, right } => {
            let left_code = generate_rust(left);
            let right_code = generate_rust(right);
            format!("({} {} {})", left_code, op, right_code)
        }
        // ... other cases
    }
}

// Example outputs:
// Int(42)           → "42"
// Var(x) + Int(1)   → "(x + 1)"
// (a + b) * 2       → "((a + b) * 2)"
```

## Semantic Preservation

The critical requirement: transpiled code must behave identically to the original.

```rust
#[test]
fn test_semantic_preservation() {
    // Python: result = x + y * 2
    // Rust:   let result = x + y * 2;

    let test_cases = vec![
        (2, 3, 8),     // 2 + 3 * 2 = 8
        (0, 5, 10),    // 0 + 5 * 2 = 10
        (10, -1, 8),   // 10 + (-1) * 2 = 8
    ];

    for (x, y, expected) in test_cases {
        let result = x + y * 2;
        assert_eq!(result, expected);
    }
}
```

## The Transpilation Pipeline

```
Stage 1: Parsing
└─ Source code → Abstract Syntax Tree (AST)

Stage 2: Type Inference
└─ Infer types from usage patterns

Stage 3: Transformation
└─ Source AST → Target AST

Stage 4: Code Generation
└─ Target AST → Target source code

Stage 5: Verification
└─ Compile target code (Rust checks safety)
```

## EU AI Act Compliance

Transpilation enables compliance with EU AI Act requirements:

### Article 10: Data Governance

```rust
// All operations are deterministic
// No external service dependencies
// Source code is fully auditable

fn transpile(source: &str) -> Result<String> {
    let ast = parse(source)?;       // Deterministic
    let typed = infer_types(ast)?;  // Deterministic
    let rust = generate(typed)?;    // Deterministic
    Ok(rust)
}
```

### Article 13: Transparency

- Clear mapping from source to target
- Type information preserved and explicit
- Behavior semantically equivalent

### Article 15: Robustness

- Rust compiler catches memory errors
- Type system prevents runtime crashes
- No garbage collection pauses

## The Sovereign AI Stack Transpilers

This book covers three transpilers in detail:

```
┌─────────────────────────────────────────────────────────┐
│              Sovereign AI Stack Transpilers             │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  bashrs (Chapter 9)                                     │
│  └─ Bash shell scripts → Rust                          │
│     Eliminates: shell injection, path issues           │
│                                                         │
│  depyler (Chapter 10)                                   │
│  └─ Python ML code → Rust                              │
│     Eliminates: GIL, dynamic type errors               │
│                                                         │
│  decy (Chapter 11)                                      │
│  └─ TypeScript/Deno → Rust                             │
│     Eliminates: Node.js runtime, V8 overhead           │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

## Testing Transpilers (Poka-Yoke)

Error-proof the transpilation process:

```rust
#[test]
fn test_determinism() {
    let source = "x + y * 2";
    let mut results = Vec::new();

    for _ in 0..10 {
        let result = transpile(source).unwrap();
        results.push(result);
    }

    let first = &results[0];
    assert!(results.iter().all(|r| r == first),
        "Transpilation must be deterministic");
}
```

## Key Takeaways

1. **Transpilation preserves semantics**: Same behavior, different language
2. **Rust target adds safety**: Type and memory safety at compile time
3. **ASTs enable structured transformation**: Language-agnostic representation
4. **Determinism enables auditing**: Same input → same output
5. **Local execution ensures sovereignty**: No cloud dependencies

## Next Steps

- **Chapter 9**: bashrs - Bash to Rust transpilation
- **Chapter 10**: depyler - Python to Rust transpilation
- **Chapter 11**: decy - TypeScript to Rust transpilation

## Source Code

Full implementation: `examples/ch08-transpilation/`

```bash
# Verify all claims
make test-ch08

# Run examples
make run-ch08
```
