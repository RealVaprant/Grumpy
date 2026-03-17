pub mod clippy;
pub mod crate_root;

use crate::config::settings::LintSettings;
use crate::lint::violation::ComplianceViolation;
use crate::config::error::ConfigError;

pub trait ComplianceCheck {
    fn run(&self, config: &LintSettings) -> Result<Option<Box<dyn ComplianceViolation>>, ConfigError>;
}
