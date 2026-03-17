use crate::config::error::ConfigError;
use crate::config::settings::GrumpyConfig;
use crate::lint::checks::ComplianceCheck;
use crate::lint::violation::ComplianceViolation;
use colored::*;
use std::process::{Command, Stdio};

pub struct ClippyCheck;

impl ComplianceCheck for ClippyCheck {
    fn run(
        &self,
        config: &GrumpyConfig,
    ) -> Result<Option<Box<dyn ComplianceViolation>>, ConfigError> {
        if config.clippy_compliance.0 {
            return Ok(None);
        }

        let status = Command::new("cargo")
            .arg("clippy")
            .arg("--")
            .arg("-D")
            .arg("warnings")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()?;

        if !status.success() {
            // We return a Boxed version of ourselves as the violation
            return Ok(Some(Box::new(ClippyCheck)));
        }

        Ok(None)
    }
}

impl ComplianceViolation for ClippyCheck {
    fn report(&self) {
        let error_label = "error".red();
        let prefix = format!("{}{}", error_label, ":".white()).bold();

        println!(
            "{} {}",
            prefix,
            "clippy checks failed with warnings or errors".bold()
        );
        println!("   {}", "|".blue().bold());
        println!(
            "   {} {}",
            "=".blue().bold(),
            "help: Run 'cargo clippy' to see the specific warnings/errors.".bold()
        );
        println!();
    }
}
