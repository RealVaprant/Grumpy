use crate::config::error::ConfigError;
use crate::config::settings::LintSettings;
use crate::lint::compliance_check::ComplianceCheck;
use crate::lint::diagnostic::Diagnostic;
use crate::lint::violation::ComplianceViolation;
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
    fn to_diagnostic(&self) -> Diagnostic {
        Diagnostic {
            title: "clippy checks failed with warnings or errors".to_string(),
            help_text: "Run 'cargo clippy' to see the specific warnings/errors.".to_string(),
            location: None,
        }
    }
}
