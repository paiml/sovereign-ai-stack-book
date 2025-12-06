/// Chapter 8: AST Analysis for Transpilation
///
/// **CLAIM:** Abstract Syntax Trees (ASTs) enable:
/// - Structured code representation
/// - Language-agnostic transformations
/// - Type inference and checking
///
/// **VALIDATION:** `make run-ch08-ast`
use anyhow::Result;
use std::fmt;

/// Simple AST node types for demonstration
#[derive(Debug, Clone, PartialEq)]
enum Expr {
    /// Integer literal: 42
    Int(i64),
    /// Floating point: 3.14
    Float(f64),
    /// String literal: "hello"
    Str(String),
    /// Boolean: true/false
    Bool(bool),
    /// Variable reference: x
    Var(String),
    /// Binary operation: x + y
    BinOp {
        op: BinOperator,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    /// Function call: foo(x, y)
    Call { name: String, args: Vec<Expr> },
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
enum BinOperator {
    Add,
    Sub,
    Mul,
    Div,
}

impl fmt::Display for BinOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinOperator::Add => write!(f, "+"),
            BinOperator::Sub => write!(f, "-"),
            BinOperator::Mul => write!(f, "*"),
            BinOperator::Div => write!(f, "/"),
        }
    }
}

/// Inferred type from AST analysis
#[derive(Debug, Clone, PartialEq)]
enum Type {
    Int,
    Float,
    Str,
    Bool,
    Unknown,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Int => write!(f, "i64"),
            Type::Float => write!(f, "f64"),
            Type::Str => write!(f, "String"),
            Type::Bool => write!(f, "bool"),
            Type::Unknown => write!(f, "?"),
        }
    }
}

/// Build an AST for: x + y * 2
fn build_example_ast() -> Expr {
    // y * 2
    let mul = Expr::BinOp {
        op: BinOperator::Mul,
        left: Box::new(Expr::Var("y".to_string())),
        right: Box::new(Expr::Int(2)),
    };

    // x + (y * 2)
    Expr::BinOp {
        op: BinOperator::Add,
        left: Box::new(Expr::Var("x".to_string())),
        right: Box::new(mul),
    }
}

/// Print AST structure
fn print_ast(expr: &Expr, indent: usize) {
    let prefix = "  ".repeat(indent);

    match expr {
        Expr::Int(n) => println!("{}Int({})", prefix, n),
        Expr::Float(f) => println!("{}Float({})", prefix, f),
        Expr::Str(s) => println!("{}Str(\"{}\")", prefix, s),
        Expr::Bool(b) => println!("{}Bool({})", prefix, b),
        Expr::Var(name) => println!("{}Var({})", prefix, name),
        Expr::BinOp { op, left, right } => {
            println!("{}BinOp({})", prefix, op);
            print_ast(left, indent + 1);
            print_ast(right, indent + 1);
        }
        Expr::Call { name, args } => {
            println!("{}Call({})", prefix, name);
            for arg in args {
                print_ast(arg, indent + 1);
            }
        }
    }
}

/// Infer type of expression
fn infer_type(expr: &Expr) -> Type {
    match expr {
        Expr::Int(_) => Type::Int,
        Expr::Float(_) => Type::Float,
        Expr::Str(_) => Type::Str,
        Expr::Bool(_) => Type::Bool,
        Expr::Var(_) => Type::Unknown, // Would need context
        Expr::BinOp { op, left, right } => {
            let left_type = infer_type(left);
            let right_type = infer_type(right);

            match (left_type, right_type, op) {
                // Int op Int -> Int (except division)
                (Type::Int, Type::Int, BinOperator::Add) => Type::Int,
                (Type::Int, Type::Int, BinOperator::Sub) => Type::Int,
                (Type::Int, Type::Int, BinOperator::Mul) => Type::Int,
                (Type::Int, Type::Int, BinOperator::Div) => Type::Float, // Division promotes to float

                // Float in either position -> Float
                (Type::Float, _, _) => Type::Float,
                (_, Type::Float, _) => Type::Float,

                // Unknown propagates
                (Type::Unknown, _, _) => Type::Unknown,
                (_, Type::Unknown, _) => Type::Unknown,

                // Invalid combinations
                _ => Type::Unknown,
            }
        }
        Expr::Call { .. } => Type::Unknown, // Would need function signature
    }
}

/// Generate Rust code from AST
fn generate_rust(expr: &Expr) -> String {
    match expr {
        Expr::Int(n) => format!("{}", n),
        Expr::Float(f) => format!("{:.1}", f),
        Expr::Str(s) => format!("\"{}\"", s),
        Expr::Bool(b) => format!("{}", b),
        Expr::Var(name) => name.clone(),
        Expr::BinOp { op, left, right } => {
            let left_code = generate_rust(left);
            let right_code = generate_rust(right);
            format!("({} {} {})", left_code, op, right_code)
        }
        Expr::Call { name, args } => {
            let args_code: Vec<String> = args.iter().map(generate_rust).collect();
            format!("{}({})", name, args_code.join(", "))
        }
    }
}

/// Demonstrate AST structure
fn ast_structure_demo() {
    println!("🌳 AST Structure");
    println!();

    let expr = build_example_ast();

    println!("   Expression: x + y * 2");
    println!();
    println!("   AST representation:");
    print_ast(&expr, 2);
    println!();
}

/// Demonstrate type inference
fn type_inference_demo() {
    println!("📋 Type Inference");
    println!();

    let examples = vec![
        ("42", Expr::Int(42)),
        ("3.5", Expr::Float(3.5)),
        ("\"hello\"", Expr::Str("hello".to_string())),
        ("true", Expr::Bool(true)),
        (
            "1 + 2",
            Expr::BinOp {
                op: BinOperator::Add,
                left: Box::new(Expr::Int(1)),
                right: Box::new(Expr::Int(2)),
            },
        ),
        (
            "1.0 + 2",
            Expr::BinOp {
                op: BinOperator::Add,
                left: Box::new(Expr::Float(1.0)),
                right: Box::new(Expr::Int(2)),
            },
        ),
    ];

    println!("   {:>15} │ {:>10}", "Expression", "Type");
    println!("   ────────────────┼────────────");

    for (source, expr) in examples {
        let ty = infer_type(&expr);
        println!("   {:>15} │ {:>10}", source, ty);
    }
    println!();
}

/// Demonstrate code generation
fn code_generation_demo() {
    println!("🔧 Code Generation");
    println!();

    let examples = vec![
        Expr::Int(42),
        Expr::BinOp {
            op: BinOperator::Add,
            left: Box::new(Expr::Var("x".to_string())),
            right: Box::new(Expr::Int(1)),
        },
        Expr::BinOp {
            op: BinOperator::Mul,
            left: Box::new(Expr::BinOp {
                op: BinOperator::Add,
                left: Box::new(Expr::Var("a".to_string())),
                right: Box::new(Expr::Var("b".to_string())),
            }),
            right: Box::new(Expr::Int(2)),
        },
        Expr::Call {
            name: "calculate".to_string(),
            args: vec![Expr::Var("x".to_string()), Expr::Int(5)],
        },
    ];

    println!("   Generated Rust code:");
    println!();

    for expr in examples {
        let rust_code = generate_rust(&expr);
        println!("   → {}", rust_code);
    }
    println!();
}

/// Evaluate expression (for verification)
fn evaluate(expr: &Expr, vars: &std::collections::HashMap<String, i64>) -> Option<i64> {
    match expr {
        Expr::Int(n) => Some(*n),
        Expr::Var(name) => vars.get(name).copied(),
        Expr::BinOp { op, left, right } => {
            let l = evaluate(left, vars)?;
            let r = evaluate(right, vars)?;
            match op {
                BinOperator::Add => Some(l + r),
                BinOperator::Sub => Some(l - r),
                BinOperator::Mul => Some(l * r),
                BinOperator::Div => Some(l / r),
            }
        }
        _ => None, // Floats, strings, etc. not supported
    }
}

/// Demonstrate semantic preservation through evaluation
fn semantic_preservation_demo() {
    println!("✅ Semantic Preservation");
    println!();

    let expr = build_example_ast(); // x + y * 2

    let mut vars = std::collections::HashMap::new();

    let test_cases = vec![
        (5, 3, 11),  // 5 + 3 * 2 = 11
        (0, 10, 20), // 0 + 10 * 2 = 20
        (10, -2, 6), // 10 + (-2) * 2 = 6
    ];

    println!("   Testing: x + y * 2");
    println!();
    println!(
        "   {:>5} {:>5} │ {:>8} │ {:>8}",
        "x", "y", "Expected", "Actual"
    );
    println!("   ────────────┼──────────┼──────────");

    for (x, y, expected) in test_cases {
        vars.insert("x".to_string(), x);
        vars.insert("y".to_string(), y);

        let actual = evaluate(&expr, &vars).unwrap();
        let status = if actual == expected { "✅" } else { "❌" };

        println!(
            "   {:>5} {:>5} │ {:>8} │ {:>8} {}",
            x, y, expected, actual, status
        );
    }
    println!();
}

fn main() -> Result<()> {
    println!("🌳 Chapter 8: AST Analysis for Transpilation");
    println!();
    println!("ASTs enable structured, language-agnostic code transformation.");
    println!();
    println!("{}", "─".repeat(70));
    println!();

    ast_structure_demo();
    println!("{}", "─".repeat(70));
    println!();

    type_inference_demo();
    println!("{}", "─".repeat(70));
    println!();

    code_generation_demo();
    println!("{}", "─".repeat(70));
    println!();

    semantic_preservation_demo();
    println!("{}", "─".repeat(70));
    println!();

    println!("🎯 Key Takeaways:");
    println!("   1. ASTs provide structured code representation");
    println!("   2. Type inference enables Rust type annotations");
    println!("   3. Code generation produces valid Rust syntax");
    println!("   4. Semantic preservation ensures correct behavior");
    println!();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ast_construction() {
        let expr = build_example_ast();
        match expr {
            Expr::BinOp {
                op: BinOperator::Add,
                ..
            } => (),
            _ => panic!("Expected BinOp(Add) at root"),
        }
    }

    #[test]
    fn test_type_inference_int() {
        let expr = Expr::Int(42);
        assert_eq!(infer_type(&expr), Type::Int);
    }

    #[test]
    fn test_type_inference_binop() {
        let expr = Expr::BinOp {
            op: BinOperator::Add,
            left: Box::new(Expr::Int(1)),
            right: Box::new(Expr::Int(2)),
        };
        assert_eq!(infer_type(&expr), Type::Int);
    }

    #[test]
    fn test_code_generation() {
        let expr = Expr::BinOp {
            op: BinOperator::Add,
            left: Box::new(Expr::Var("x".to_string())),
            right: Box::new(Expr::Int(1)),
        };
        let code = generate_rust(&expr);
        assert_eq!(code, "(x + 1)");
    }

    #[test]
    fn test_evaluate() {
        let expr = build_example_ast(); // x + y * 2
        let mut vars = std::collections::HashMap::new();
        vars.insert("x".to_string(), 5);
        vars.insert("y".to_string(), 3);

        let result = evaluate(&expr, &vars);
        assert_eq!(result, Some(11)); // 5 + 3 * 2 = 11
    }

    #[test]
    fn test_semantic_preservation() {
        let expr = build_example_ast();
        let mut vars = std::collections::HashMap::new();

        // Test multiple inputs
        for (x, y, expected) in [(1, 2, 5), (0, 0, 0), (10, -5, 0)] {
            vars.insert("x".to_string(), x);
            vars.insert("y".to_string(), y);
            let result = evaluate(&expr, &vars).unwrap();
            assert_eq!(result, expected, "x={}, y={}", x, y);
        }
    }
}
