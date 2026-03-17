use std::env;
use crate::config::manager::ConfigLoader;
use crate::lint::engine::LintEngine;
use crate::lint::tests::project_setup::{setup_temp_project, TEST_RESOURCE_LOCK};

#[test]
fn clippy_violation_is_detected_when_source_contains_lint_warnings() {
    let _synchronization_guard = TEST_RESOURCE_LOCK.lock().expect("Failed to acquire test lock");
    
    let violating_main_content = r#"
        fn main() {
            let first_value = 10;
            let is_comparison_redundant = first_value == first_value;
            if is_comparison_redundant {
                println!("This triggers clippy::eq_op");
            }
        }
    "#;
    
    let project_path = setup_temp_project("violating_project", violating_main_content);
    let original_working_directory = env::current_dir().expect("Failed to get current dir");
    
    env::set_current_dir(&project_path).expect("Failed to change to temp project dir");
    let (name, version) = ConfigLoader::load_metadata().expect("Metadata load failed");
    let config = ConfigLoader::load_config().expect("Config load failed");
    let violations = LintEngine::run_checks(&config, &name, &version).expect("Engine run failed");
    env::set_current_dir(original_working_directory).expect("Failed to restore directory");
    
    assert!(!violations.is_empty());
}
