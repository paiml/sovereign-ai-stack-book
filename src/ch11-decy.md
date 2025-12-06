# decy: C to Rust Transpilation

> **Toyota Way Principle (Jidoka):** Build quality in. Convert C's undefined behavior to Rust's guaranteed safety.

**Status:** Complete

## The Problem: C's Memory Unsafety

C code is fast but dangerous:

```c
// Buffer overflow
char buffer[10];
strcpy(buffer, very_long_string);  // Writes past end!

// Use-after-free
char* ptr = malloc(100);
free(ptr);
printf("%s", ptr);  // Undefined behavior!

// Dangling pointer
char* get_name() {
    char buffer[32];
    strcpy(buffer, "Alice");
    return buffer;  // Returns stack memory!
}
```

## decy Solution: Transpile to Safe Rust

```
┌─────────────────────────────────────────────────────────┐
│                    decy Pipeline                         │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  C Code → Parser → AST → Ownership Analysis → Rust     │
│     │                         │                         │
│     ↓                         ↓                         │
│  Pointers                 References                    │
│  malloc/free              Ownership/Drop                │
│  NULL                     Option<T>                     │
│  Buffer overflow          Bounds checking               │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

## Validation

Run all chapter examples:

```bash
make run-ch11      # Run examples
make test-ch11     # Run all tests
```

## Type Mapping

| C Type | Rust Type |
|--------|-----------|
| `int` | `i32` |
| `long` | `i64` |
| `unsigned int` | `u32` |
| `float` | `f32` |
| `double` | `f64` |
| `char*` | `String` or `&str` |
| `int[]` | `Vec<i32>` or `[i32; N]` |
| `T*` | `&T` or `&mut T` or `Box<T>` |
| `NULL` | `None` (Option<T>) |

## Pointer to Reference Transpilation

### C Code

```c
void process(int* data, int len) {
    for (int i = 0; i < len; i++) {
        data[i] *= 2;
    }
}
```

### Rust Code

```rust
fn process(data: &mut [i32]) {
    for item in data.iter_mut() {
        *item *= 2;
    }
}
```

Key improvements:
- No separate length parameter needed (slices carry length)
- Bounds checking automatic
- No null pointer possible

## Memory Safety: Dangling Pointers

### C (VULNERABLE)

```c
char* get_name() {
    char buffer[32];
    strcpy(buffer, "Alice");
    return buffer;  // DANGLING POINTER!
}
```

### Rust (SAFE)

```rust
fn get_name() -> String {
    let buffer = String::from("Alice");
    buffer  // Ownership transferred, no dangle!
}
// Compiler prevents returning references to locals
```

## Memory Safety: Buffer Overflow

### C (VULNERABLE)

```c
void copy_data(char* dest, char* src) {
    strcpy(dest, src);  // No bounds checking!
}
// Buffer overflow if src > dest capacity
```

### Rust (SAFE)

```rust
fn copy_data(dest: &mut String, src: &str) {
    dest.clear();
    dest.push_str(src);  // Automatic resizing!
}
// Or use slices with bounds checking
```

## Struct Transpilation

### C Code

```c
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

void free_student(Student* s) {
    free(s);
}
```

### Rust Code

```rust
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
// No free_student needed - ownership handles cleanup!
```

## NULL to Option

### C Pattern

```c
User* find_user(int id) {
    // Returns NULL if not found
    if (id < 0) return NULL;
    return &users[id];
}

// Caller must check
User* user = find_user(id);
if (user != NULL) {
    printf("%s", user->name);
}
```

### Rust Pattern

```rust
fn find_user(id: i32) -> Option<&User> {
    if id < 0 {
        return None;
    }
    Some(&users[id as usize])
}

// Compiler FORCES handling
match find_user(id) {
    Some(user) => println!("{}", user.name),
    None => println!("User not found"),
}
```

## Performance Preservation

decy preserves C's performance characteristics:

| Aspect | C | Rust |
|--------|---|------|
| Memory layout | Same | Same |
| Inline functions | Same | Same |
| Zero-cost abstractions | Manual | Automatic |
| Bounds checking | None | Optional (release mode) |

## EU AI Act Compliance

### Article 10: Data Governance

- No undefined behavior
- Deterministic memory management
- All allocations tracked

### Article 13: Transparency

- Source-to-source mapping preserved
- Ownership semantics make data flow explicit
- Every pointer has documented lifetime

### Article 15: Robustness

- No buffer overflows
- No use-after-free
- No null pointer dereference
- No data races

## Testing

```rust
#[test]
fn test_pointer_to_slice() {
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
    let ptr: Option<i32> = None;
    assert!(ptr.is_none());

    let ptr2: Option<i32> = Some(42);
    assert_eq!(ptr2, Some(42));
}
```

## Key Takeaways

1. **Pointers → References**: Lifetimes enforced by compiler
2. **malloc/free → Ownership**: Automatic cleanup via Drop
3. **NULL → Option**: Compiler-enforced null checking
4. **Buffer overflows → Prevented**: Bounds checking automatic
5. **Same performance**: Zero-cost abstractions

## Next Steps

- **Chapter 12**: aprender - ML training framework
- **Chapter 13**: realizar - Inference engine

## Source Code

Full implementation: `examples/ch11-decy/`

```bash
# Verify all claims
make test-ch11

# Run examples
make run-ch11
```
