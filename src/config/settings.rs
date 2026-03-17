use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone, Copy, PartialEq, Eq)]
pub struct ClippyCompliance(pub bool);

#[derive(Debug, Serialize, Deserialize, Default, Clone, Copy, PartialEq, Eq)]
pub struct CrateRootCheck(pub bool);

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GrumpyConfig {
    #[serde(rename = "ignore-clippy-compliance")]
    pub clippy_compliance: ClippyCompliance,

    #[serde(rename = "ignore-prohibited-crate-root-files")]
    pub crate_root_check: CrateRootCheck,
}
