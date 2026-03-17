use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Missing Cargo.toml: this program must be run in a Rust project root.")]
    MissingCargoToml,

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] toml::ser::Error),

    #[error("Deserialization error: {0}")]
    Deserialization(#[from] toml::de::Error),
}
