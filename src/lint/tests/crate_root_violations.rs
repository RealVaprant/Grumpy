use crate::config::manager::ConfigLoader;
use crate::lint::engine::LintEngine;
use crate::lint::tests::project_setup::{TEST_RESOURCE_LOCK, setup_temp_project};
use std::env;
use std::fs;

#[test]
fn crate_root_structure_violation_is_detected_when_prohibited_file_exists_in_src() {
    let _synchronization_guard = TEST_RESOURCE_LOCK
        .lock()
        .expect("Failed to acquire test lock");

    let project_path = setup_temp_project("structure_violation", "fn main() {}");
    fs::write(project_path.join("src/widgets.rs"), "struct Widget;")
        .expect("Failed to write prohibited file");

    let original_working_directory = env::current_dir().expect("Failed to get current dir");
    env::set_current_dir(&project_path).expect("Failed to change to temp project dir");

    let (name, version) = ConfigLoader::load_metadata().expect("Metadata load failed");
    let config = ConfigLoader::load_config().expect("Config load failed");
    let violations = LintEngine::run_checks(&config, &name, &version).expect("Engine run failed");

    env::set_current_dir(original_working_directory).expect("Failed to restore directory");

    assert!(!violations.is_empty());
}
