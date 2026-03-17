use crate::config::error::ConfigError;
use crate::config::settings::GrumpyConfig;
use crate::lint::checks::ComplianceCheck;
use crate::lint::violation::ComplianceViolation;
use colored::*;
use std::fs;
use std::path::Path;

pub struct CrateRootCheck;

impl ComplianceCheck for CrateRootCheck {
    fn run(
        &self,
        config: &GrumpyConfig,
    ) -> Result<Option<Box<dyn ComplianceViolation>>, ConfigError> {
        if config.crate_root_check.0 {
            return Ok(None);
        }

        let source_directory = "src";
        let allowed_root_files = ["main.rs", "lib.rs", "build.rs"];
        let source_path = Path::new(source_directory);

        if !source_path.is_dir() {
            return Ok(None);
        }

        let mut prohibited_files = Vec::new();
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
                prohibited_files.push(file_name.to_string());
            }
        }

        if prohibited_files.is_empty() {
            return Ok(None);
        }

        Ok(Some(Box::new(CrateRootViolation { prohibited_files })))
    }
}

struct CrateRootViolation {
    prohibited_files: Vec<String>,
}

impl ComplianceViolation for CrateRootViolation {
    fn report(&self) {
        let error_label = "error".red();
        let prefix = format!("{}{}", error_label, ":".white()).bold();

        for prohibited_file in &self.prohibited_files {
            println!(
                "{} {}",
                prefix,
                "found prohibited file in crate root directory".bold()
            );
            println!("  {} src/{}", "-->".blue().bold(), prohibited_file);
            println!("   {}", "|".blue().bold());
            println!(
                "   {} {}", 
                "=".blue().bold(), 
                "help: Only main.rs, lib.rs, and build.rs are allowed in the src/ directory. Move this file to a subdirectory.".bold()
            );
            println!();
        }
    }
}
