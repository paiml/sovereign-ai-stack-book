//! Integration tests for the Sovereign AI Stack Book examples.
//!
//! These tests verify that the book examples compile and function correctly.

/// Test that the workspace declares every chapter crate it ships, under resolver 2.
///
/// This replaces an `assert!(true)` that measured nothing: it passed identically
/// whether the workspace was correct or empty.
#[test]
fn test_workspace_resolves() {
    let cargo_toml = include_str!("../Cargo.toml");

    assert!(cargo_toml.contains(r#"resolver = "2""#), "workspace must pin resolver 2");

    let members: Vec<&str> = cargo_toml
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with("\"examples/ch"))
        .collect();
    assert_eq!(
        members.len(),
        22,
        "expected 22 chapter crates in [workspace] members, found {}: {members:?}",
        members.len()
    );

    // Every declared member must exist on disk, or `cargo` cannot resolve the workspace.
    for member in &members {
        let path = member.trim_matches(|c| c == '"' || c == ',');
        assert!(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join(path)
                .join("Cargo.toml")
                .is_file(),
            "declared workspace member {path} has no Cargo.toml"
        );
    }
}

/// Test Criterion benchmark framework is available
#[test]
fn test_criterion_available() {
    // Verify benchmark dependencies are accessible
    use criterion::Criterion;
    let _criterion = Criterion::default();
}

/// Test that the root package builds
#[test]
fn test_root_package_exists() {
    let cargo_toml = include_str!("../Cargo.toml");
    assert!(cargo_toml.contains("[workspace]"));
    assert!(cargo_toml.contains("members"));
}

/// Test workspace member count
#[test]
fn test_workspace_has_chapters() {
    let cargo_toml = include_str!("../Cargo.toml");
    // Verify chapters exist
    assert!(cargo_toml.contains("ch01-intro"));
    assert!(cargo_toml.contains("ch22-deployment"));
}

/// Test workspace dependencies are declared
#[test]
fn test_workspace_dependencies_declared() {
    let cargo_toml = include_str!("../Cargo.toml");
    assert!(cargo_toml.contains("[workspace.dependencies]"));
    assert!(cargo_toml.contains("criterion"));
    assert!(cargo_toml.contains("proptest"));
}

/// Test lints configuration exists
#[test]
fn test_workspace_lints_configured() {
    let cargo_toml = include_str!("../Cargo.toml");
    assert!(cargo_toml.contains("[workspace.lints.rust]"));
    assert!(cargo_toml.contains("[workspace.lints.clippy]"));
}
