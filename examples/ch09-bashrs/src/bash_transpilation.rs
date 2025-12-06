/// Chapter 9: bashrs - Bash to Rust Transpilation
///
/// **CLAIM:** bashrs converts Bash scripts to safe Rust:
/// - Eliminates shell injection vulnerabilities
/// - Cross-platform execution (no bash dependency)
/// - Type-safe command handling
///
/// **VALIDATION:** `make run-ch09`
///
/// **KEY PRINCIPLE:** Security Through Transpilation
/// - Shell commands become typed function calls
/// - No string interpolation vulnerabilities
use anyhow::Result;
use std::path::PathBuf;

/// Simulated Bash command types
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
enum BashCommand {
    /// echo "text"
    Echo(String),
    /// cd /path
    Cd(PathBuf),
    /// ls -la /path
    Ls { path: PathBuf, flags: Vec<String> },
    /// cat file1 file2
    Cat(Vec<PathBuf>),
    /// Variable assignment: VAR=value
    Assign { name: String, value: String },
    /// Variable reference: $VAR
    VarRef(String),
    /// Pipe: cmd1 | cmd2
    Pipe(Box<BashCommand>, Box<BashCommand>),
}

/// Generate safe Rust code from Bash commands
fn transpile_to_rust(cmd: &BashCommand) -> String {
    match cmd {
        BashCommand::Echo(text) => {
            format!("println!(\"{}\");", escape_string(text))
        }
        BashCommand::Cd(path) => {
            format!("std::env::set_current_dir(PathBuf::from({:?}))?;",
                    path.display())
        }
        BashCommand::Ls { path, flags } => {
            let flags_str = if flags.is_empty() {
                String::new()
            } else {
                format!(", flags: {:?}", flags)
            };
            format!("list_directory(PathBuf::from({:?}){});",
                    path.display(), flags_str)
        }
        BashCommand::Cat(files) => {
            let paths: Vec<String> = files.iter()
                .map(|p| format!("PathBuf::from({:?})", p.display()))
                .collect();
            format!("concatenate_files(&[{}]);", paths.join(", "))
        }
        BashCommand::Assign { name, value } => {
            format!("let {} = String::from({:?});", name, value)
        }
        BashCommand::VarRef(name) => {
            format!("&{}", name)
        }
        BashCommand::Pipe(left, right) => {
            format!("pipe({}, {});",
                    transpile_to_rust(left),
                    transpile_to_rust(right))
        }
    }
}

/// Escape strings for safe Rust output
fn escape_string(s: &str) -> String {
    s.replace('\\', "\\\\")
     .replace('"', "\\\"")
     .replace('\n', "\\n")
     .replace('\r', "\\r")
     .replace('\t', "\\t")
}

/// Demonstrate Bash to Rust transpilation
fn transpilation_demo() {
    println!("🔄 Bash to Rust Transpilation");
    println!();

    let examples = vec![
        (
            "echo \"Hello, World!\"",
            BashCommand::Echo("Hello, World!".to_string()),
        ),
        (
            "cd /home/user",
            BashCommand::Cd(PathBuf::from("/home/user")),
        ),
        (
            "ls -la /tmp",
            BashCommand::Ls {
                path: PathBuf::from("/tmp"),
                flags: vec!["-la".to_string()],
            },
        ),
        (
            "NAME=\"Alice\"",
            BashCommand::Assign {
                name: "name".to_string(),
                value: "Alice".to_string(),
            },
        ),
    ];

    for (bash, cmd) in examples {
        let rust = transpile_to_rust(&cmd);
        println!("   Bash: {}", bash);
        println!("   Rust: {}", rust);
        println!();
    }
}

/// Show security improvements
fn security_improvements() {
    println!("🛡️  Security Improvements");
    println!();

    println!("   Bash Vulnerability: Command Injection");
    println!("   ─────────────────────────────────────");
    println!("   Unsafe Bash:");
    println!("     name=\"$USER_INPUT\"");
    println!("     echo \"Hello, $name\"  # If name contains $(rm -rf /)...");
    println!();

    println!("   Safe Rust (via bashrs):");
    println!("     let name = String::from(user_input);");
    println!("     println!(\"Hello, {{}}\", name);  // No injection possible");
    println!();

    println!("   Bash Vulnerability: Path Traversal");
    println!("   ─────────────────────────────────────");
    println!("   Unsafe Bash:");
    println!("     cat /data/$filename  # If filename is ../../etc/passwd...");
    println!();

    println!("   Safe Rust (via bashrs):");
    println!("     let path = PathBuf::from(\"/data\").join(&filename);");
    println!("     let canonical = path.canonicalize()?;");
    println!("     if !canonical.starts_with(\"/data\") {{");
    println!("         return Err(\"Path traversal detected\");");
    println!("     }}");
    println!();
}

/// Demonstrate type safety
fn type_safety_demo() {
    println!("📋 Type Safety");
    println!();

    println!("   Bash (untyped):");
    println!("     count=5");
    println!("     result=$((count + \"hello\"))  # Silent failure or error");
    println!();

    println!("   Rust (typed):");
    println!("     let count: i32 = 5;");
    println!("     let result = count + \"hello\";  // Compile error!");
    println!("     // error: cannot add `&str` to `i32`");
    println!();
}

/// Cross-platform advantages
fn cross_platform() {
    println!("🌍 Cross-Platform Execution");
    println!();

    println!("   Bash scripts require:");
    println!("   ├─ Bash interpreter installed");
    println!("   ├─ Unix-like environment");
    println!("   └─ Platform-specific paths (/usr/bin vs /bin)");
    println!();

    println!("   Transpiled Rust:");
    println!("   ├─ Single native binary");
    println!("   ├─ Works on Windows, macOS, Linux");
    println!("   └─ No runtime dependencies");
    println!();
}

/// EU AI Act compliance
fn eu_compliance() {
    println!("🇪🇺 EU AI Act Compliance");
    println!();

    println!("   Article 10 (Data Governance):");
    println!("   ├─ No shell expansion of untrusted data");
    println!("   └─ All inputs validated at compile time");
    println!();

    println!("   Article 13 (Transparency):");
    println!("   ├─ Source-to-source mapping preserved");
    println!("   └─ Behavior fully auditable");
    println!();

    println!("   Article 15 (Robustness):");
    println!("   ├─ Memory-safe execution");
    println!("   ├─ No shell injection possible");
    println!("   └─ Cross-platform reliability");
    println!();
}

fn main() -> Result<()> {
    println!("🐚 Chapter 9: bashrs - Bash to Rust Transpilation");
    println!();
    println!("Convert Bash scripts to safe, cross-platform Rust.");
    println!();
    println!("{}", "─".repeat(70));
    println!();

    transpilation_demo();
    println!("{}", "─".repeat(70));
    println!();

    security_improvements();
    println!("{}", "─".repeat(70));
    println!();

    type_safety_demo();
    println!("{}", "─".repeat(70));
    println!();

    cross_platform();
    println!("{}", "─".repeat(70));
    println!();

    eu_compliance();
    println!("{}", "─".repeat(70));
    println!();

    println!("🎯 Key Takeaways:");
    println!("   1. bashrs eliminates shell injection vulnerabilities");
    println!("   2. Typed commands catch errors at compile time");
    println!("   3. Cross-platform: one binary runs everywhere");
    println!("   4. Full EU AI Act compliance for shell operations");
    println!();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_echo_transpilation() {
        let cmd = BashCommand::Echo("Hello".to_string());
        let rust = transpile_to_rust(&cmd);
        assert!(rust.contains("println!"));
        assert!(rust.contains("Hello"));
    }

    #[test]
    fn test_cd_transpilation() {
        let cmd = BashCommand::Cd(PathBuf::from("/home"));
        let rust = transpile_to_rust(&cmd);
        assert!(rust.contains("set_current_dir"));
        assert!(rust.contains("/home"));
    }

    #[test]
    fn test_escape_string() {
        assert_eq!(escape_string("hello"), "hello");
        assert_eq!(escape_string("he\"llo"), "he\\\"llo");
        assert_eq!(escape_string("line1\nline2"), "line1\\nline2");
    }

    #[test]
    fn test_assign_transpilation() {
        let cmd = BashCommand::Assign {
            name: "x".to_string(),
            value: "42".to_string(),
        };
        let rust = transpile_to_rust(&cmd);
        assert!(rust.contains("let x"));
        assert!(rust.contains("42"));
    }
}
