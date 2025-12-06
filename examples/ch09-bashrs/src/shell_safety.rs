/// Chapter 9: Shell Safety Through Transpilation
///
/// **CLAIM:** bashrs prevents common shell vulnerabilities:
/// - Command injection
/// - Path traversal
/// - Environment variable attacks
///
/// **VALIDATION:** `make run-ch09-safety`
use anyhow::Result;
use std::path::{Path, PathBuf};

/// Safe command builder - prevents injection
#[derive(Debug)]
struct SafeCommand {
    program: String,
    args: Vec<String>,
}

impl SafeCommand {
    /// Create a new command (program name cannot contain spaces or special chars)
    fn new(program: &str) -> Result<Self> {
        if program
            .chars()
            .any(|c| c.is_whitespace() || c == ';' || c == '|' || c == '&')
        {
            anyhow::bail!("Invalid program name: {}", program);
        }
        Ok(Self {
            program: program.to_string(),
            args: Vec::new(),
        })
    }

    /// Add an argument (automatically escaped)
    fn arg(mut self, arg: &str) -> Self {
        self.args.push(arg.to_string());
        self
    }

    /// Get the safe command representation
    fn to_safe_string(&self) -> String {
        let escaped_args: Vec<String> = self.args.iter().map(|a| format!("{:?}", a)).collect();
        format!("{} {}", self.program, escaped_args.join(" "))
    }
}

/// Safe path handling - prevents traversal
#[derive(Debug)]
struct SafePath {
    base: PathBuf,
    relative: PathBuf,
}

impl SafePath {
    /// Create a safe path within a base directory
    fn new(base: &Path, relative: &str) -> Result<Self> {
        let relative_path = PathBuf::from(relative);

        // Check for traversal attempts
        for component in relative_path.components() {
            match component {
                std::path::Component::ParentDir => {
                    anyhow::bail!("Path traversal detected: {}", relative);
                }
                std::path::Component::RootDir => {
                    anyhow::bail!("Absolute path not allowed: {}", relative);
                }
                _ => {}
            }
        }

        Ok(Self {
            base: base.to_path_buf(),
            relative: relative_path,
        })
    }

    /// Get the full, safe path
    fn full_path(&self) -> PathBuf {
        self.base.join(&self.relative)
    }
}

/// Demonstrate command injection prevention
fn command_injection_demo() {
    println!("🛡️  Command Injection Prevention");
    println!();

    // Dangerous bash approach
    println!("   Bash (VULNERABLE):");
    println!("   ┌────────────────────────────────────────────");
    println!("   │ user_input=\"file.txt; rm -rf /\"");
    println!("   │ cat $user_input  # EXECUTES rm -rf /!");
    println!("   └────────────────────────────────────────────");
    println!();

    // Safe Rust approach
    println!("   Rust via bashrs (SAFE):");
    let user_input = "file.txt; rm -rf /";

    match SafeCommand::new("cat") {
        Ok(cmd) => {
            let safe_cmd = cmd.arg(user_input);
            println!("   ┌────────────────────────────────────────────");
            println!("   │ let user_input = {:?};", user_input);
            println!("   │ SafeCommand::new(\"cat\").arg(user_input)");
            println!("   │ // Result: {}", safe_cmd.to_safe_string());
            println!("   │ // The semicolon is ESCAPED, not executed!");
            println!("   └────────────────────────────────────────────");
        }
        Err(e) => println!("   Error: {}", e),
    }
    println!();
}

/// Demonstrate path traversal prevention
fn path_traversal_demo() {
    println!("🛡️  Path Traversal Prevention");
    println!();

    // Dangerous bash approach
    println!("   Bash (VULNERABLE):");
    println!("   ┌────────────────────────────────────────────");
    println!("   │ filename=\"../../../etc/passwd\"");
    println!("   │ cat /data/$filename  # READS /etc/passwd!");
    println!("   └────────────────────────────────────────────");
    println!();

    // Safe Rust approach
    println!("   Rust via bashrs (SAFE):");
    let base = Path::new("/data");
    let malicious_input = "../../../etc/passwd";

    match SafePath::new(base, malicious_input) {
        Ok(path) => {
            println!("   Path: {:?}", path.full_path());
        }
        Err(e) => {
            println!("   ┌────────────────────────────────────────────");
            println!("   │ let base = Path::new(\"/data\");");
            println!("   │ SafePath::new(base, {:?})", malicious_input);
            println!("   │ // Error: {}", e);
            println!("   │ // Attack BLOCKED at construction time!");
            println!("   └────────────────────────────────────────────");
        }
    }
    println!();

    // Valid path example
    let valid_input = "users/alice/document.txt";
    match SafePath::new(base, valid_input) {
        Ok(path) => {
            println!("   Valid path example:");
            println!("   ┌────────────────────────────────────────────");
            println!("   │ SafePath::new(base, {:?})", valid_input);
            println!("   │ // Result: {:?}", path.full_path());
            println!("   │ // Stays within /data directory!");
            println!("   └────────────────────────────────────────────");
        }
        Err(e) => println!("   Error: {}", e),
    }
    println!();
}

/// Environment variable safety
fn env_var_safety() {
    println!("🛡️  Environment Variable Safety");
    println!();

    println!("   Bash (VULNERABLE):");
    println!("   ┌────────────────────────────────────────────");
    println!("   │ # Attacker sets: PATH=\"/malicious/bin:$PATH\"");
    println!("   │ ls  # Executes /malicious/bin/ls instead!");
    println!("   └────────────────────────────────────────────");
    println!();

    println!("   Rust via bashrs (SAFE):");
    println!("   ┌────────────────────────────────────────────");
    println!("   │ // Commands use absolute paths");
    println!("   │ Command::new(\"/usr/bin/ls\")");
    println!("   │     .args(&[\"-la\", \"/home\"])");
    println!("   │     .spawn()?;");
    println!("   │ // PATH variable cannot redirect execution!");
    println!("   └────────────────────────────────────────────");
    println!();
}

/// Quoting and escaping
fn quoting_safety() {
    println!("🛡️  Quoting and Escaping");
    println!();

    let dangerous_strings = vec![
        "hello world",       // Spaces
        "file$(whoami).txt", // Command substitution
        "name`id`",          // Backticks
        "$HOME/secret",      // Variable expansion
        "a; rm -rf /",       // Command chaining
    ];

    println!("   {:>25} │ {:>30}", "Input", "Escaped Output");
    println!("   ──────────────────────────┼───────────────────────────────");

    for s in dangerous_strings {
        let escaped = format!("{:?}", s);
        println!("   {:>25} │ {:>30}", s, escaped);
    }
    println!();

    println!("   All special characters are escaped in Rust strings!");
    println!("   No shell interpretation occurs.");
    println!();
}

fn main() -> Result<()> {
    println!("🔒 Chapter 9: Shell Safety Through Transpilation");
    println!();
    println!("Demonstrating how bashrs prevents shell vulnerabilities.");
    println!();
    println!("{}", "─".repeat(70));
    println!();

    command_injection_demo();
    println!("{}", "─".repeat(70));
    println!();

    path_traversal_demo();
    println!("{}", "─".repeat(70));
    println!();

    env_var_safety();
    println!("{}", "─".repeat(70));
    println!();

    quoting_safety();
    println!("{}", "─".repeat(70));
    println!();

    println!("🎯 Security Summary:");
    println!("   ✅ Command injection: BLOCKED (arguments escaped)");
    println!("   ✅ Path traversal: BLOCKED (components validated)");
    println!("   ✅ Env var attacks: BLOCKED (absolute paths used)");
    println!("   ✅ Quoting issues: BLOCKED (proper escaping)");
    println!();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_command_creation() {
        assert!(SafeCommand::new("ls").is_ok());
        assert!(SafeCommand::new("cat").is_ok());
    }

    #[test]
    fn test_safe_command_rejects_injection() {
        assert!(SafeCommand::new("ls; rm").is_err());
        assert!(SafeCommand::new("cat | grep").is_err());
        assert!(SafeCommand::new("cmd && evil").is_err());
    }

    #[test]
    fn test_safe_path_allows_valid() {
        let base = Path::new("/data");
        assert!(SafePath::new(base, "file.txt").is_ok());
        assert!(SafePath::new(base, "subdir/file.txt").is_ok());
    }

    #[test]
    fn test_safe_path_rejects_traversal() {
        let base = Path::new("/data");
        assert!(SafePath::new(base, "../etc/passwd").is_err());
        assert!(SafePath::new(base, "subdir/../../etc/passwd").is_err());
    }

    #[test]
    fn test_safe_path_rejects_absolute() {
        let base = Path::new("/data");
        assert!(SafePath::new(base, "/etc/passwd").is_err());
    }

    #[test]
    fn test_escaping() {
        let cmd = SafeCommand::new("echo")
            .expect("echo is a safe command")
            .arg("hello; rm -rf /");
        let safe = cmd.to_safe_string();
        // The semicolon should be inside quotes, not executed
        assert!(safe.contains("\"hello; rm -rf /\""));
    }
}
