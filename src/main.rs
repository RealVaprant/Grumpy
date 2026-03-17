mod config;
mod lint;

use crate::config::manager::ConfigLoader;
use crate::lint::engine::LintEngine;
use colored::*;

fn main() {
    let (package_name, package_version) = match ConfigLoader::load_metadata() {
        Ok(metadata) => metadata,
        Err(systemic_error) => {
            eprintln!("{}: {}", "error".red().bold(), systemic_error);
            std::process::exit(1);
        }
    };

    let config = match ConfigLoader::load_config() {
        Ok(cfg) => cfg,
        Err(systemic_error) => {
            eprintln!("{}: {}", "error".red().bold(), systemic_error);
            std::process::exit(1);
        }
    };

    let violations = match LintEngine::run_checks(&config, &package_name, &package_version) {
        Ok(list) => list,
        Err(systemic_error) => {
            eprintln!("{}: {}", "error".red().bold(), systemic_error);
            std::process::exit(1);
        }
    };

    if violations.is_empty() {
        return;
    }

    let violation_count = violations.len();
    for violation in &violations {
        violation.report();
    }

    let error_label = if violation_count == 1 {
        "error"
    } else {
        "errors"
    };
    println!(
        "{}: {} (bin \"{}\") generated {} {}",
        "error".red().bold(),
        package_name,
        package_name,
        violation_count,
        error_label
    );
}
