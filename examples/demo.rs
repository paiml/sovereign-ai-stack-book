//! Demo example with rich terminal output
//!
//! Run with: cargo run --example demo

use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use std::time::Duration;

fn main() {
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".cyan());
    println!("{}", "  🚀 Sovereign AI Stack Demo".bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".cyan());
    println!();

    // Progress bar demo
    let pb = ProgressBar::new(100);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .expect("valid template")
            .progress_chars("█▓░"),
    );

    for i in 0..100 {
        pb.set_position(i);
        pb.set_message(format!("Processing step {}", i + 1));
        thread::sleep(Duration::from_millis(10));
    }
    pb.finish_with_message("Done!");

    println!();
    println!("{}", "✅ Demo complete!".green().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".cyan());
}
