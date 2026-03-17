use std::time::Instant;
use colored::*;

use crate::config::settings::LintSettings;
use crate::lint::violation::ComplianceViolation;
use crate::lint::checks::clippy::ClippyCheck;
use crate::lint::checks::crate_root::CrateRootCheck;
use crate::lint::checks::ComplianceCheck;
use crate::config::error::ConfigError;

pub struct LintEngine;

impl LintEngine {
    pub fn run_checks(config: &LintSettings, package_name: &str, package_version: &str) -> Result<Vec<Box<dyn ComplianceViolation>>, ConfigError> {
        let start_time = Instant::now();
        
        println!(
            "{:>12} {} v{} ({})", 
            "Checking".green().bold(), 
            package_name, 
            package_version, 
            std::env::current_dir()?.display()
        );

        let checks: Vec<Box<dyn ComplianceCheck>> = vec![
            Box::new(CrateRootCheck),
            Box::new(ClippyCheck),
        ];

        let mut found_violations = Vec::new();

        for check in checks {
            if let Some(violation) = check.run(config)? {
                found_violations.push(violation);
            }
        }

        if found_violations.is_empty() {
            let duration_seconds = start_time.elapsed().as_secs_f64();
            println!(
                "{:>12} checking target(s) in {:.2}s", 
                "Finished".green().bold(),
                duration_seconds
            );
        }

        Ok(found_violations)
    }
}
