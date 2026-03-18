use crate::config::error::ConfigError;
use crate::config::settings::LintSettings;
use crate::lint::compliance_check::ComplianceCheck;
use crate::lint::diagnostic::{Diagnostic, Location};
use crate::lint::violation::ComplianceViolation;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Default)]
pub struct CrateRootCheck {
    pub prohibited_file: String,
}

impl ComplianceCheck for CrateRootCheck {
    fn run(&self, config: &LintSettings) -> Result<Vec<Box<dyn ComplianceViolation>>, ConfigError> {
        if config.is_crate_root_check_ignored {
            return Ok(Vec::new());
        }

        let source_directory = "src";
        let allowed_root_files = ["main.rs", "lib.rs", "build.rs"];
        let source_path = Path::new(source_directory);

        if !source_path.is_dir() {
            return Ok(Vec::new());
        }

        let mut violations: Vec<Box<dyn ComplianceViolation>> = Vec::new();
        for entry in fs::read_dir(source_path)? {
            let entry = entry?;
            let path = entry.path();

            let is_rust_file = path.extension().and_then(|ext| ext.to_str()) == Some("rs");
            if !is_rust_file {
                continue;
            }

            let file_name = path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("");

            if !allowed_root_files.contains(&file_name) {
                violations.push(Box::new(CrateRootCheck {
                    prohibited_file: file_name.to_string(),
                }));
            }
        }

        Ok(violations)
    }
}

impl ComplianceViolation for CrateRootCheck {
    fn to_diagnostic(&self) -> Diagnostic {
        Diagnostic {
            title: "found prohibited file in crate root directory".to_string(),
            help_text: "Only main.rs, lib.rs, and build.rs are allowed in the src/ directory. Move this file to a subdirectory.".to_string(),
            location: Some(Location {
                file_path: PathBuf::from("src").join(&self.prohibited_file),
                line_number: None,
                column_number: None,
                snippet: None,
            }),
        }
    }
}
