use std::path::Path;

pub mod expected_sets;

pub fn cleanup(database_path: &str) {
    if Path::new(database_path).exists() {
        std::fs::remove_file(database_path).unwrap();
    }
}