use crate::config::error::ConfigError;
use crate::config::settings::LintSettings;
use crate::lint::compliance_check::ComplianceCheck;
use crate::lint::diagnostic::{Diagnostic, Location};
use crate::lint::violation::ComplianceViolation;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Default)]
pub struct ModLogicCheck {
    pub file_path: PathBuf,
    pub line_number: usize,
    pub line_content: String,
    pub prohibited_keyword: String,
}

impl ComplianceCheck for ModLogicCheck {
    fn run(&self, config: &LintSettings) -> Result<Vec<Box<dyn ComplianceViolation>>, ConfigError> {
        if config.is_mod_logic_ignored {
            return Ok(Vec::new());
        }

        let mut violations = Vec::new();
        self.scan_directory(Path::new("src"), &mut violations)?;

        Ok(violations)
    }
}

impl ModLogicCheck {
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
        let prohibited_keywords = [
            "fn",
            "struct",
            "impl",
            "enum",
            "trait",
            "type",
            "macro_rules!",
        ];
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

            let keyword_candidate = if parts[0].starts_with("pub") && parts.len() > 1 {
                parts[1]
            } else {
                parts[0]
            };

            if prohibited_keywords.contains(&keyword_candidate) {
                violations.push(Box::new(ModLogicCheck {
                    file_path: path.to_path_buf(),
                    line_number: index + 1,
                    line_content: line.to_string(),
                    prohibited_keyword: keyword_candidate.to_string(),
                }));

                brace_depth += line.chars().filter(|&c| c == '{').count();
                brace_depth -= line.chars().filter(|&c| c == '}').count();
            }
        }

        Ok(())
    }
}

impl ComplianceViolation for ModLogicCheck {
    fn to_diagnostic(&self) -> Diagnostic {
        let title = match self.prohibited_keyword.as_str() {
            "struct" | "trait" | "enum" => format!("{} found in mod.rs", self.prohibited_keyword),
            _ => "logic found in mod.rs".to_string(),
        };

        let column_number = self
            .line_content
            .find(&self.prohibited_keyword)
            .unwrap_or(0)
            + 1;

        Diagnostic {
            title,
            help_text: "Logic isn't allowed in mod.rs files, move it somewhere else.".to_string(),
            location: Some(Location {
                file_path: self.file_path.clone(),
                line_number: Some(self.line_number),
                column_number: Some(column_number),
                snippet: Some(self.line_content.clone()),
            }),
        }
    }
}
