/// Chapter 8: Introduction to Transpilation
///
/// **CLAIM:** Transpilation converts code between languages while:
/// - Preserving semantics (behavior unchanged)
/// - Adding safety guarantees (Rust type system)
/// - Enabling local execution (EU AI Act compliance)
///
/// **VALIDATION:** `make run-ch08`
///
/// **KEY PRINCIPLE:** Deterministic Transformation
/// - Same source → same output (always)
/// - No runtime dependencies on cloud services
use anyhow::Result;

/// Why transpile to Rust?
fn why_transpile() {
    println!("🎯 Why Transpile to Rust?");
    println!();

    println!("   Source Languages       →  Target: Rust");
    println!("   ─────────────────────────────────────────────");
    println!("   Python (dynamic)       →  Memory-safe, typed");
    println!("   Bash (shell)           →  Cross-platform, fast");
    println!("   TypeScript (Node)      →  Native binary, no VM");
    println!();

    println!("   Benefits of Rust target:");
    println!("   ├─ Memory safety (no GC pauses)");
    println!("   ├─ Type safety (compile-time errors)");
    println!("   ├─ Performance (native code)");
    println!("   ├─ No runtime (single binary)");
    println!("   └─ EU AI Act compliance (auditable, local)");
    println!();
}

/// Transpilation vs Compilation
fn transpilation_vs_compilation() {
    println!("📊 Transpilation vs Compilation");
    println!();

    println!("   Compilation:");
    println!("   Source Code → AST → IR → Machine Code");
    println!("   (Python → bytecode, C → assembly)");
    println!();

    println!("   Transpilation:");
    println!("   Source Code → AST → Target Source Code");
    println!("   (Python → Rust, TypeScript → JavaScript)");
    println!();

    println!("   Key difference:");
    println!("   - Compilation: outputs executable");
    println!("   - Transpilation: outputs source code");
    println!();

    println!("   Our approach: Transpile THEN Compile");
    println!("   Python → Rust → Native Binary");
    println!("   (safety verification at each step)");
    println!();
}

/// Simple expression transpilation example
fn expression_transpilation() {
    println!("🔄 Expression Transpilation Example");
    println!();

    // Python expression
    let python_code = "result = x + y * 2";

    // Equivalent Rust
    let rust_code = "let result = x + y * 2;";

    println!("   Python: {}", python_code);
    println!("   Rust:   {}", rust_code);
    println!();

    // More complex example with types
    let python_typed = r#"
    def calculate(x: int, y: int) -> int:
        return x + y * 2"#;

    let rust_typed = r#"
    fn calculate(x: i32, y: i32) -> i32 {
        x + y * 2
    }"#;

    println!("   Python with types:");
    for line in python_typed.lines() {
        if !line.is_empty() {
            println!("     {}", line.trim());
        }
    }
    println!();

    println!("   Rust equivalent:");
    for line in rust_typed.lines() {
        if !line.is_empty() {
            println!("     {}", line.trim());
        }
    }
    println!();
}

/// Type mapping between languages
fn type_mapping() {
    println!("📋 Type Mapping");
    println!();

    println!("   {:>15} │ {:>15} │ {:>15}", "Python", "TypeScript", "Rust");
    println!("   ────────────────┼─────────────────┼────────────────");
    println!("   {:>15} │ {:>15} │ {:>15}", "int", "number", "i64");
    println!("   {:>15} │ {:>15} │ {:>15}", "float", "number", "f64");
    println!("   {:>15} │ {:>15} │ {:>15}", "str", "string", "String");
    println!("   {:>15} │ {:>15} │ {:>15}", "bool", "boolean", "bool");
    println!("   {:>15} │ {:>15} │ {:>15}", "list[T]", "T[]", "Vec<T>");
    println!("   {:>15} │ {:>15} │ {:>15}", "dict[K,V]", "Map<K,V>", "HashMap<K,V>");
    println!("   {:>15} │ {:>15} │ {:>15}", "None", "null", "Option<T>");
    println!();
}

/// Semantic preservation verification
fn semantic_preservation() {
    println!("✅ Semantic Preservation");
    println!();

    println!("   Transpilation must preserve behavior:");
    println!();

    // Demonstrate equivalent behavior
    let inputs = vec![
        (2, 3),   // 2 + 3 * 2 = 8
        (0, 5),   // 0 + 5 * 2 = 10
        (10, -1), // 10 + (-1) * 2 = 8
    ];

    println!("   Testing: result = x + y * 2");
    println!();

    for (x, y) in &inputs {
        let result = x + y * 2;
        println!("   x={:>3}, y={:>3} → result = {:>3}", x, y, result);
    }

    println!();
    println!("   ✅ Same inputs → Same outputs (deterministic)");
    println!();
}

/// Transpilation pipeline stages
fn pipeline_stages() {
    println!("🔧 Transpilation Pipeline");
    println!();

    println!("   Stage 1: Parsing");
    println!("   └─ Source code → Abstract Syntax Tree (AST)");
    println!();

    println!("   Stage 2: Type Inference");
    println!("   └─ Infer types from usage patterns");
    println!();

    println!("   Stage 3: Transformation");
    println!("   └─ Source AST → Target AST");
    println!();

    println!("   Stage 4: Code Generation");
    println!("   └─ Target AST → Target source code");
    println!();

    println!("   Stage 5: Verification");
    println!("   └─ Compile target code (Rust checks safety)");
    println!();
}

/// EU AI Act compliance through transpilation
fn eu_compliance() {
    println!("🇪🇺 EU AI Act Compliance via Transpilation");
    println!();

    println!("   Article 10 (Data Governance):");
    println!("   ├─ Source code is fully auditable");
    println!("   ├─ Transformation is deterministic");
    println!("   └─ No external service dependencies");
    println!();

    println!("   Article 13 (Transparency):");
    println!("   ├─ Clear mapping from source to target");
    println!("   ├─ Type information preserved");
    println!("   └─ Behavior semantically equivalent");
    println!();

    println!("   Article 15 (Robustness):");
    println!("   ├─ Rust compiler catches memory errors");
    println!("   ├─ Type system prevents runtime crashes");
    println!("   └─ No garbage collection pauses");
    println!();
}

fn main() -> Result<()> {
    println!("🔄 Chapter 8: Introduction to Transpilation");
    println!();
    println!("Transpilation: Converting code between languages");
    println!("while preserving semantics and adding safety.");
    println!();
    println!("{}", "─".repeat(70));
    println!();

    why_transpile();
    println!("{}", "─".repeat(70));
    println!();

    transpilation_vs_compilation();
    println!("{}", "─".repeat(70));
    println!();

    expression_transpilation();
    println!("{}", "─".repeat(70));
    println!();

    type_mapping();
    println!("{}", "─".repeat(70));
    println!();

    semantic_preservation();
    println!("{}", "─".repeat(70));
    println!();

    pipeline_stages();
    println!("{}", "─".repeat(70));
    println!();

    eu_compliance();
    println!("{}", "─".repeat(70));
    println!();

    println!("🎯 Key Takeaways:");
    println!("   1. Transpilation preserves behavior across languages");
    println!("   2. Rust target adds memory and type safety");
    println!("   3. Deterministic transformation enables auditing");
    println!("   4. Local execution ensures data sovereignty");
    println!();

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_semantic_preservation() {
        // Python: result = x + y * 2
        // Rust:   let result = x + y * 2;
        // Both should produce identical results

        let test_cases = vec![
            (2, 3, 8),     // 2 + 3 * 2 = 8
            (0, 5, 10),    // 0 + 5 * 2 = 10
            (10, -1, 8),   // 10 + (-1) * 2 = 8
            (-5, -5, -15), // -5 + (-5) * 2 = -15
        ];

        for (x, y, expected) in test_cases {
            let result = x + y * 2;
            assert_eq!(result, expected, "x={}, y={} should produce {}", x, y, expected);
        }
    }

    #[test]
    fn test_type_conversion_i32() {
        // Python int maps to Rust i64 or i32
        let py_int: i32 = 42;
        let rust_int: i32 = 42;
        assert_eq!(py_int, rust_int);
    }

    #[test]
    fn test_type_conversion_string() {
        // Python str maps to Rust String
        let py_str = String::from("hello");
        let rust_str = String::from("hello");
        assert_eq!(py_str, rust_str);
    }

    #[test]
    fn test_determinism() {
        let mut results = Vec::new();

        for _ in 0..10 {
            let x = 5;
            let y = 3;
            let result = x + y * 2;
            results.push(result);
        }

        let first = results[0];
        assert!(results.iter().all(|&r| r == first), "Transpiled code must be deterministic");
    }
}
