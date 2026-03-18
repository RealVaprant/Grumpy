use crate::config::error::ConfigError;
use crate::config::settings::LintSettings;
use crate::lint::compliance_check::ComplianceCheck;
use crate::lint::diagnostic::{Diagnostic, Location};
use crate::lint::violation::ComplianceViolation;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Default)]
pub struct ModDefinitionCheck {
    pub file_path: PathBuf,
    pub line_number: usize,
    pub line_content: String,
    pub keyword: String,
    pub definition_name: String,
}

impl ComplianceCheck for ModDefinitionCheck {
    fn run(&self, config: &LintSettings) -> Result<Vec<Box<dyn ComplianceViolation>>, ConfigError> {
        if config.is_mod_definition_ignored {
            return Ok(Vec::new());
        }

        let mut violations = Vec::new();
        self.scan_directory(Path::new("src"), &mut violations)?;

        Ok(violations)
    }
}

impl ModDefinitionCheck {
    fn scan_directory(
        &self,
        path: &Path,
        violations: &mut Vec<Box<dyn ComplianceViolation>>,
    ) -> Result<(), ConfigError> {
        if !path.is_dir() {
            return Ok(());
        }

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_dir() {
                self.scan_directory(&entry_path, violations)?;
                continue;
            }

            let is_mod_file = entry_path.file_name().and_then(|n| n.to_str()) == Some("mod.rs");
            if is_mod_file {
                self.check_mod_file(&entry_path, violations)?;
            }
        }

        Ok(())
    }

    fn check_mod_file(
        &self,
        path: &Path,
        violations: &mut Vec<Box<dyn ComplianceViolation>>,
    ) -> Result<(), ConfigError> {
        let content = fs::read_to_string(path)?;
        let prohibited_definitions = ["struct", "enum", "trait", "type"];
        let mut brace_depth = 0;

        for (index, line) in content.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            if brace_depth > 0 {
                brace_depth += line.chars().filter(|&c| c == '{').count();
                brace_depth -= line.chars().filter(|&c| c == '}').count();
                continue;
            }

            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            let (keyword_candidate, name_index) = if parts[0].starts_with("pub") && parts.len() > 1
            {
                (parts[1], 2)
            } else {
                (parts[0], 1)
            };

            if prohibited_definitions.contains(&keyword_candidate) {
                let definition_name = parts
                    .get(name_index)
                    .map(|s| s.trim_end_matches(|c: char| !c.is_alphanumeric() && c != '_'))
                    .unwrap_or("unknown")
                    .to_string();

                violations.push(Box::new(ModDefinitionCheck {
                    file_path: path.to_path_buf(),
                    line_number: index + 1,
                    line_content: line.to_string(),
                    keyword: keyword_candidate.to_string(),
                    definition_name,
                }));

                brace_depth += line.chars().filter(|&c| c == '{').count();
                brace_depth -= line.chars().filter(|&c| c == '}').count();
            }
        }

        Ok(())
    }
}

impl ComplianceViolation for ModDefinitionCheck {
    fn to_diagnostic(&self) -> Diagnostic {
        let (plural_subject, verb) = match self.keyword.as_str() {
            "struct" => ("Structs", "aren't"),
            "enum" => ("Enums", "aren't"),
            "trait" => ("Traits", "aren't"),
            "type" => ("Types", "aren't"),
            _ => ("Definitions", "aren't"),
        };

        let title = format!("{} is not allowed in mod.rs", self.definition_name);
        let help_text = format!(
            "{} {} allowed in mod.rs files, move it somewhere else.",
            plural_subject, verb
        );

        let column_number = self.line_content.find(&self.definition_name).unwrap_or(0) + 1;

        Diagnostic {
            title,
            help_text,
            location: Some(Location {
                file_path: self.file_path.clone(),
                line_number: Some(self.line_number),
                column_number: Some(column_number),
                snippet: Some(self.line_content.clone()),
            }),
        }
    }
}
