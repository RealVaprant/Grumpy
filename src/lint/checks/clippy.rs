use crate::config::error::ConfigError;
use crate::config::settings::LintSettings;
use crate::lint::compliance_check::ComplianceCheck;
use crate::lint::violation::ComplianceViolation;
use colored::*;
use std::process::{Command, Stdio};

pub struct ClippyCheck;

impl ComplianceCheck for ClippyCheck {
    fn run(&self, config: &LintSettings) -> Result<Vec<Box<dyn ComplianceViolation>>, ConfigError> {
        if config.is_clippy_ignored {
            return Ok(Vec::new());
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
            return Ok(vec![Box::new(ClippyCheck)]);
        }

        Ok(Vec::new())
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
