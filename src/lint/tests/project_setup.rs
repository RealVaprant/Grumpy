use std::env;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

pub static TEST_RESOURCE_LOCK: Mutex<()> = Mutex::new(());

const TEST_CARGO_VERSION: &str = "0.1.0";
const TEST_RUST_EDITION: &str = "2024";

pub fn setup_temp_project(project_name: &str, main_file_content: &str) -> PathBuf {
    let mut target_temp_directory = env::temp_dir();
    target_temp_directory.push(format!("grumpy_test_{}", project_name));

    if target_temp_directory.exists() {
        let _ = fs::remove_dir_all(&target_temp_directory);
    }

    fs::create_dir_all(target_temp_directory.join("src"))
        .expect("Failed to create src directory for test");

    let cargo_manifest_content = format!(
        r#"[package]
name = "{}"
version = "{}"
edition = "{}"
"#,
        project_name, TEST_CARGO_VERSION, TEST_RUST_EDITION
    );

    fs::write(
        target_temp_directory.join("Cargo.toml"),
        cargo_manifest_content,
    )
    .expect("Failed to write Cargo.toml for test");
    fs::write(target_temp_directory.join("src/main.rs"), main_file_content)
        .expect("Failed to write main.rs for test");

    target_temp_directory
}
