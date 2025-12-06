/// Chapter 11: decy - C to Rust Transpilation
///
/// **CLAIM:** decy converts C code to safe Rust:
/// - Eliminates memory unsafety (buffer overflows, use-after-free)
/// - Preserves performance characteristics
/// - Adds Rust's ownership guarantees
///
/// **VALIDATION:** `make run-ch11`
use anyhow::Result;

/// C type to Rust type mapping
fn type_mapping_demo() {
    println!("📋 C to Rust Type Mapping");
    println!();

    let mappings = vec![
        ("int", "i32"),
        ("long", "i64"),
        ("unsigned int", "u32"),
        ("float", "f32"),
        ("double", "f64"),
        ("char", "i8 or u8"),
        ("char*", "String or &str"),
        ("void*", "Box<dyn Any> or *mut c_void"),
        ("int[]", "Vec<i32> or [i32; N]"),
        ("struct T", "struct T"),
        ("T*", "&T or &mut T or Box<T>"),
        ("NULL", "None (Option<T>)"),
    ];

    println!("   {:>20} │ {:>25}", "C Type", "Rust Type");
    println!("   ─────────────────────┼──────────────────────────");

    for (c_type, rust_type) in mappings {
        println!("   {:>20} │ {:>25}", c_type, rust_type);
    }
    println!();
}

/// Demonstrate pointer to reference transpilation
fn pointer_transpilation() {
    println!("🔄 Pointer to Reference Transpilation");
    println!();

    let c_code = r#"
void process(int* data, int len) {
    for (int i = 0; i < len; i++) {
        data[i] *= 2;
    }
}
"#;

    let rust_code = r#"
fn process(data: &mut [i32]) {
    for item in data.iter_mut() {
        *item *= 2;
    }
}
"#;

    println!("   C:");
    for line in c_code.lines() {
        if !line.is_empty() {
            println!("   {}", line);
        }
    }
    println!();

    println!("   Rust:");
    for line in rust_code.lines() {
        if !line.is_empty() {
            println!("   {}", line);
        }
    }
    println!();
}

/// Memory safety improvements
fn memory_safety() {
    println!("🛡️  Memory Safety Improvements");
    println!();

    println!("   C (VULNERABLE):");
    println!("   ┌────────────────────────────────────────────");
    println!("   │ char* get_name() {{");
    println!("   │     char buffer[32];");
    println!("   │     strcpy(buffer, \"Alice\");");
    println!("   │     return buffer;  // DANGLING POINTER!");
    println!("   │ }}");
    println!("   └────────────────────────────────────────────");
    println!();

    println!("   Rust via decy (SAFE):");
    println!("   ┌────────────────────────────────────────────");
    println!("   │ fn get_name() -> String {{");
    println!("   │     let buffer = String::from(\"Alice\");");
    println!("   │     buffer  // Ownership transferred, no dangle!");
    println!("   │ }}");
    println!("   │ // Compiler prevents returning references to locals");
    println!("   └────────────────────────────────────────────");
    println!();
}

/// Buffer overflow prevention
fn buffer_overflow_prevention() {
    println!("🛡️  Buffer Overflow Prevention");
    println!();

    println!("   C (VULNERABLE):");
    println!("   ┌────────────────────────────────────────────");
    println!("   │ void copy_data(char* dest, char* src) {{");
    println!("   │     strcpy(dest, src);  // No bounds checking!");
    println!("   │ }}");
    println!("   │ // Buffer overflow if src > dest capacity");
    println!("   └────────────────────────────────────────────");
    println!();

    println!("   Rust via decy (SAFE):");
    println!("   ┌────────────────────────────────────────────");
    println!("   │ fn copy_data(dest: &mut String, src: &str) {{");
    println!("   │     dest.clear();");
    println!("   │     dest.push_str(src);  // Automatic resizing!");
    println!("   │ }}");
    println!("   │ // Or use slices with bounds checking");
    println!("   └────────────────────────────────────────────");
    println!();
}

/// Struct transpilation
fn struct_transpilation() {
    println!("📦 Struct Transpilation");
    println!();

    let c_code = r#"
typedef struct {
    int id;
    char name[64];
    float score;
} Student;

Student* create_student(int id, const char* name) {
    Student* s = malloc(sizeof(Student));
    s->id = id;
    strncpy(s->name, name, 63);
    s->score = 0.0f;
    return s;
}
"#;

    let rust_code = r#"
#[derive(Debug, Clone)]
struct Student {
    id: i32,
    name: String,
    score: f32,
}

fn create_student(id: i32, name: &str) -> Student {
    Student {
        id,
        name: name.to_string(),
        score: 0.0,
    }
}
// No malloc/free needed - ownership handles cleanup!
"#;

    println!("   C:");
    for line in c_code.lines() {
        if !line.is_empty() {
            println!("   {}", line);
        }
    }
    println!();

    println!("   Rust:");
    for line in rust_code.lines() {
        if !line.is_empty() {
            println!("   {}", line);
        }
    }
    println!();
}

/// EU AI Act compliance
fn eu_compliance() {
    println!("🇪🇺 EU AI Act Compliance");
    println!();

    println!("   Article 10 (Data Governance):");
    println!("   ├─ No undefined behavior");
    println!("   └─ Deterministic memory management");
    println!();

    println!("   Article 13 (Transparency):");
    println!("   ├─ Source-to-source mapping preserved");
    println!("   └─ Ownership semantics make data flow explicit");
    println!();

    println!("   Article 15 (Robustness):");
    println!("   ├─ No buffer overflows");
    println!("   ├─ No use-after-free");
    println!("   ├─ No null pointer dereference");
    println!("   └─ No data races");
    println!();
}

fn main() -> Result<()> {
    println!("🔧 Chapter 11: decy - C to Rust Transpilation");
    println!();
    println!("Convert C code to memory-safe Rust.");
    println!();
    println!("{}", "─".repeat(70));
    println!();

    type_mapping_demo();
    println!("{}", "─".repeat(70));
    println!();

    pointer_transpilation();
    println!("{}", "─".repeat(70));
    println!();

    memory_safety();
    println!("{}", "─".repeat(70));
    println!();

    buffer_overflow_prevention();
    println!("{}", "─".repeat(70));
    println!();

    struct_transpilation();
    println!("{}", "─".repeat(70));
    println!();

    eu_compliance();
    println!("{}", "─".repeat(70));
    println!();

    println!("🎯 Key Takeaways:");
    println!("   1. Pointers → References with lifetimes");
    println!("   2. malloc/free → Ownership/Drop");
    println!("   3. Buffer overflows → Compile-time prevented");
    println!("   4. Same performance, guaranteed safety");
    println!();

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_int_to_i32() {
        // C: int x = 42;
        // Rust: let x: i32 = 42;
        let x: i32 = 42;
        assert_eq!(x, 42);
    }

    #[test]
    fn test_array_to_vec() {
        // C: int arr[] = {1, 2, 3};
        // Rust: let arr = vec![1, 2, 3];
        let arr: Vec<i32> = vec![1, 2, 3];
        assert_eq!(arr.len(), 3);
        assert_eq!(arr[0], 1);
    }

    #[test]
    fn test_pointer_to_slice() {
        // C: void process(int* data, int len)
        // Rust: fn process(data: &mut [i32])
        fn process(data: &mut [i32]) {
            for item in data.iter_mut() {
                *item *= 2;
            }
        }

        let mut data = vec![1, 2, 3];
        process(&mut data);
        assert_eq!(data, vec![2, 4, 6]);
    }

    #[test]
    fn test_null_to_option() {
        // C: int* ptr = NULL;
        // Rust: let ptr: Option<i32> = None;
        let ptr: Option<i32> = None;
        assert!(ptr.is_none());

        let ptr2: Option<i32> = Some(42);
        assert_eq!(ptr2, Some(42));
    }

    #[test]
    fn test_struct_ownership() {
        #[derive(Debug, PartialEq)]
        struct Point { x: i32, y: i32 }

        // C: Point* p = malloc(sizeof(Point));
        // Rust: let p = Box::new(Point { x: 1, y: 2 });
        let p = Box::new(Point { x: 1, y: 2 });
        assert_eq!(p.x, 1);
        assert_eq!(p.y, 2);
        // Automatically freed when p goes out of scope
    }
}
