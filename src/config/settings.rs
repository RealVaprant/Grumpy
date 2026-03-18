use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LintSettings {
    #[serde(rename = "ignore-clippy-compliance")]
    pub is_clippy_ignored: bool,

    #[serde(rename = "ignore-prohibited-crate-root-files")]
    pub is_crate_root_check_ignored: bool,

    #[serde(rename = "ignore-mod-logic")]
    pub is_mod_logic_ignored: bool,
}
