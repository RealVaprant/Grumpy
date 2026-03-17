pub mod clippy;
pub mod crate_root;

use crate::config::error::ConfigError;
use crate::config::settings::GrumpyConfig;
use crate::lint::violation::ComplianceViolation;

pub trait ComplianceCheck {
    fn run(
        &self,
        config: &GrumpyConfig,
    ) -> Result<Option<Box<dyn ComplianceViolation>>, ConfigError>;
}
