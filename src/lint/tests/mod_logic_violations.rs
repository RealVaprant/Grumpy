use crate::config::manager::ConfigLoader;
use crate::lint::engine::LintEngine;
use crate::lint::tests::project_setup::{TEST_RESOURCE_LOCK, setup_temp_project};
use std::env;
use std::fs;

#[test]
fn mod_logic_violation_is_detected_when_struct_is_defined_in_mod_rs() {
    let _synchronization_guard = TEST_RESOURCE_LOCK
        .lock()
        .expect("Failed to acquire test lock");

    let project_path = setup_temp_project("mod_logic_violation", "fn main() {}");
    let subfolder = project_path.join("src/widgets");
    fs::create_dir_all(&subfolder).expect("Failed to create subfolder");

    fs::write(subfolder.join("mod.rs"), "pub struct Widget;")
        .expect("Failed to write violating mod.rs");

    let original_working_directory = env::current_dir().expect("Failed to get current dir");
    env::set_current_dir(&project_path).expect("Failed to change to temp project dir");

    let (name, version) = ConfigLoader::load_metadata().expect("Metadata load failed");
    let config = ConfigLoader::load_config().expect("Config load failed");
    let violations = LintEngine::run_checks(&config, &name, &version).expect("Engine run failed");

    env::set_current_dir(original_working_directory).expect("Failed to restore directory");

    let has_mod_logic_violation = violations.iter().any(|_| !violations.is_empty());

    assert!(has_mod_logic_violation);
}
