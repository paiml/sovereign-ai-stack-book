# bashrs: Bash to Rust Transpilation

> **Toyota Way Principle (Poka-Yoke):** Error-proof the process. Eliminate shell injection at the source.

**Status:** Complete

## The Problem: Shell Script Vulnerabilities

Bash scripts are powerful but dangerous:

```bash
# VULNERABLE: Command injection
user_input="file.txt; rm -rf /"
cat $user_input  # Executes rm -rf /!

# VULNERABLE: Path traversal
filename="../../../etc/passwd"
cat /data/$filename  # Reads /etc/passwd!
```

## bashrs Solution: Safe by Construction

bashrs transpiles Bash to Rust, eliminating entire categories of vulnerabilities:

```
┌─────────────────────────────────────────────────────────┐
│                    bashrs Pipeline                       │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Bash Script → Parser → AST → Rust Code → Binary       │
│       │                         │                       │
│       ↓                         ↓                       │
│  Shell injection          Type-safe commands           │
│  Path traversal           Validated paths              │
│  Env var attacks          Explicit configuration       │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

## Validation

Run all chapter examples:

```bash
make run-ch09           # Run all examples
make run-ch09-transpilation  # Bash transpilation
make run-ch09-safety    # Shell safety demo
make test-ch09          # Run all tests
```

## Bash to Rust Mapping

| Bash Command | Rust Equivalent |
|--------------|-----------------|
| `echo "text"` | `println!("text");` |
| `cd /path` | `std::env::set_current_dir(path)?;` |
| `cat file` | `std::fs::read_to_string(path)?` |
| `VAR=value` | `let var = String::from("value");` |
| `$VAR` | `&var` |

### Example Transpilation

```bash
# Bash
NAME="Alice"
echo "Hello, $NAME"
cd /home/user
ls -la
```

```rust
// Transpiled Rust
let name = String::from("Alice");
println!("Hello, {}", name);
std::env::set_current_dir(PathBuf::from("/home/user"))?;
list_directory(PathBuf::from("."), &["-la"]);
```

## Security: Command Injection Prevention

### The Vulnerability

```bash
# Bash (VULNERABLE)
user_input="file.txt; rm -rf /"
cat $user_input  # The semicolon executes rm!
```

### The Safe Alternative

```rust
// Rust via bashrs (SAFE)
let user_input = "file.txt; rm -rf /";
SafeCommand::new("cat")
    .arg(user_input)  // Argument is escaped
    .execute()?;

// Result: cat "file.txt; rm -rf /"
// The semicolon is a STRING, not a command separator!
```

### SafeCommand Implementation

```rust
struct SafeCommand {
    program: String,
    args: Vec<String>,
}

impl SafeCommand {
    fn new(program: &str) -> Result<Self> {
        // Reject dangerous characters in program name
        if program.chars().any(|c| ";|&".contains(c)) {
            bail!("Invalid program name");
        }
        Ok(Self { program: program.to_string(), args: vec![] })
    }

    fn arg(mut self, arg: &str) -> Self {
        // Arguments are stored as strings, not interpreted
        self.args.push(arg.to_string());
        self
    }
}
```

## Security: Path Traversal Prevention

### The Vulnerability

```bash
# Bash (VULNERABLE)
filename="../../../etc/passwd"
cat /data/$filename  # Reads /etc/passwd!
```

### The Safe Alternative

```rust
// Rust via bashrs (SAFE)
let base = Path::new("/data");
let filename = "../../../etc/passwd";

let safe_path = SafePath::new(base, filename)?;
// Error: Path traversal detected!
```

### SafePath Implementation

```rust
struct SafePath {
    base: PathBuf,
    relative: PathBuf,
}

impl SafePath {
    fn new(base: &Path, relative: &str) -> Result<Self> {
        let relative_path = PathBuf::from(relative);

        // Check each path component
        for component in relative_path.components() {
            match component {
                Component::ParentDir => {
                    bail!("Path traversal detected: {}", relative);
                }
                Component::RootDir => {
                    bail!("Absolute path not allowed");
                }
                _ => {}
            }
        }

        Ok(Self {
            base: base.to_path_buf(),
            relative: relative_path,
        })
    }
}
```

## Security: Environment Variable Safety

### The Vulnerability

```bash
# Attacker sets: PATH="/malicious/bin:$PATH"
ls  # Executes /malicious/bin/ls instead of /usr/bin/ls!
```

### The Safe Alternative

```rust
// Rust via bashrs uses absolute paths
Command::new("/usr/bin/ls")
    .args(&["-la", "/home"])
    .spawn()?;

// PATH cannot redirect execution!
```

## Cross-Platform Execution

Bash scripts require:
- Bash interpreter installed
- Unix-like environment
- Platform-specific paths

Transpiled Rust provides:
- Single native binary
- Works on Windows, macOS, Linux
- No runtime dependencies

```rust
// Same code runs everywhere
#[cfg(windows)]
const LS_CMD: &str = "dir";

#[cfg(unix)]
const LS_CMD: &str = "ls";
```

## Type Safety

### Bash (Untyped)

```bash
count=5
result=$((count + "hello"))  # Silent failure or cryptic error
```

### Rust (Typed)

```rust
let count: i32 = 5;
let result = count + "hello";
// error: cannot add `&str` to `i32`
// Caught at compile time!
```

## EU AI Act Compliance

### Article 10: Data Governance

```rust
// All inputs validated at construction time
let cmd = SafeCommand::new("process")?
    .arg(&validated_input);
// No shell expansion of untrusted data
```

### Article 13: Transparency

- Source-to-source mapping preserved
- Every Bash command has Rust equivalent
- Behavior fully auditable

### Article 15: Robustness

- Memory-safe execution
- No shell injection possible
- Cross-platform reliability

## Testing (Poka-Yoke)

```rust
#[test]
fn test_safe_command_rejects_injection() {
    assert!(SafeCommand::new("ls; rm").is_err());
    assert!(SafeCommand::new("cat | grep").is_err());
    assert!(SafeCommand::new("cmd && evil").is_err());
}

#[test]
fn test_safe_path_rejects_traversal() {
    let base = Path::new("/data");
    assert!(SafePath::new(base, "../etc/passwd").is_err());
    assert!(SafePath::new(base, "subdir/../../etc").is_err());
}
```

## Performance Comparison

| Metric | Bash | bashrs (Rust) |
|--------|------|---------------|
| Startup time | ~10ms (interpreter) | ~1ms (native) |
| Execution | Interpreted | Compiled |
| Memory safety | None | Guaranteed |
| Type checking | None | Compile-time |

## Key Takeaways

1. **Command injection eliminated**: Arguments are escaped, not interpreted
2. **Path traversal blocked**: Components validated at construction
3. **Type safety**: Errors caught at compile time
4. **Cross-platform**: Single binary runs everywhere
5. **EU compliant**: Full auditability and transparency

## Next Steps

- **Chapter 10**: depyler - Python to Rust transpilation
- **Chapter 11**: decy - TypeScript to Rust transpilation

## Source Code

Full implementation: `examples/ch09-bashrs/`

```bash
# Verify all claims
make test-ch09

# Run examples
make run-ch09
```
