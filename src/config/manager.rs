use crate::config::error::ConfigError;
use crate::config::settings::LintSettings;
use std::fs;
use std::path::Path;

pub struct ConfigLoader;

impl ConfigLoader {
    const CARGO_PATH: &'static str = "Cargo.toml";
    const GRUMPY_PATH: &'static str = "grumpy.toml";

    pub fn load_metadata() -> Result<(String, String), ConfigError> {
        if !Path::new(Self::CARGO_PATH).exists() {
            return Err(ConfigError::MissingCargoToml);
        }

        let content = fs::read_to_string(Self::CARGO_PATH)?;
        let value: serde_json::Value =
            toml::from_str(&content).map_err(|_| ConfigError::MissingCargoToml)?;

        let name = value["package"]["name"]
            .as_str()
            .unwrap_or("unknown")
            .to_string();
        let version = value["package"]["version"]
            .as_str()
            .unwrap_or("0.0.0")
            .to_string();

        Ok((name, version))
    }

    pub fn load_config() -> Result<LintSettings, ConfigError> {
        let path = Path::new(Self::GRUMPY_PATH);
        if !path.exists() {
            return Self::create_default_config(path);
        }

        let content = fs::read_to_string(path)?;
        match toml::from_str::<LintSettings>(&content) {
            Ok(config) => Ok(config),
            Err(_) => {
                eprintln!(
                    "Warning: Configuration in {} is invalid or outdated. Resetting...",
                    Self::GRUMPY_PATH
                );
                Self::create_default_config(path)
            }
        }
    }

    fn create_default_config(path: &Path) -> Result<LintSettings, ConfigError> {
        let default_config = LintSettings::default();
        let toml_string = toml::to_string_pretty(&default_config)?;
        fs::write(path, toml_string)?;
        Ok(default_config)
    }
}
