use crate::config::manager::ConfigLoader;
use crate::lint::engine::LintEngine;
use crate::lint::tests::project_setup::{TEST_RESOURCE_LOCK, setup_temp_project};
use std::env;

#[test]
fn clippy_check_passes_when_source_code_is_fully_compliant() {
    let _synchronization_guard = TEST_RESOURCE_LOCK
        .lock()
        .expect("Failed to acquire test lock");

    let compliant_main_content = "fn main() { println!(\"Hello, valid world!\"); }";
    let project_path = setup_temp_project("compliant_project", compliant_main_content);
    let original_working_directory = env::current_dir().expect("Failed to get current dir");

    env::set_current_dir(&project_path).expect("Failed to change to temp project dir");
    let (name, version) = ConfigLoader::load_metadata().expect("Metadata load failed");
    let config = ConfigLoader::load_config().expect("Config load failed");
    let violations = LintEngine::run_checks(&config, &name, &version).expect("Engine run failed");
    env::set_current_dir(original_working_directory).expect("Failed to restore directory");

    assert!(violations.is_empty());
}
