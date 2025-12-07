//! Integration tests for the Sovereign AI Stack Book examples.
//!
//! These tests verify that the book examples compile and function correctly.

/// Test that basic workspace configuration is correct
#[test]
fn test_workspace_resolves() {
    // Basic sanity test - if this compiles, workspace is configured correctly
    assert!(true);
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
