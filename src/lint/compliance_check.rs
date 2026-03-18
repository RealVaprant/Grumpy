use crate::config::error::ConfigError;
use crate::config::settings::LintSettings;
use crate::lint::violation::ComplianceViolation;

pub trait ComplianceCheck {
    fn run(&self, config: &LintSettings) -> Result<Vec<Box<dyn ComplianceViolation>>, ConfigError>;
}
