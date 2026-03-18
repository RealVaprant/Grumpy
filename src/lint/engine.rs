use colored::*;
use std::time::Instant;

use crate::config::error::ConfigError;
use crate::config::settings::LintSettings;
use crate::lint::checks::clippy::ClippyCheck;
use crate::lint::checks::crate_root::CrateRootCheck;
use crate::lint::checks::mod_definition::ModDefinitionCheck;
use crate::lint::checks::mod_logic::ModLogicCheck;
use crate::lint::compliance_check::ComplianceCheck;
use crate::lint::violation::ComplianceViolation;

pub struct LintEngine;

impl LintEngine {
    pub fn run_checks(
        config: &LintSettings,
        package_name: &str,
        package_version: &str,
    ) -> Result<Vec<Box<dyn ComplianceViolation>>, ConfigError> {
        let start_time = Instant::now();

        println!(
            "{:>12} {} v{} ({})",
            "Checking".green().bold(),
            package_name,
            package_version,
            std::env::current_dir()?.display()
        );

        let checks: Vec<Box<dyn ComplianceCheck>> = vec![
            Box::new(CrateRootCheck::default()),
            Box::new(ClippyCheck),
            Box::new(ModLogicCheck::default()),
            Box::new(ModDefinitionCheck::default()),
        ];

        let mut found_violations = Vec::new();

        for check in checks {
            found_violations.extend(check.run(config)?);
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
