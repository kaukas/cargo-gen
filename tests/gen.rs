extern crate cargo;
extern crate cargo_gen_helpers;
extern crate tempdir;

use cargo_gen_helpers::gen::CargoGeneratorGenerator;
use cargo_gen_helpers::test_helpers::{read_file_to_string, create_empty_crate, run_generated_tests};
use cargo_gen_helpers::{modify_file, CargoGenerator};

#[test]
// :-)
fn it_generates_a_generator_file_in_a_cargo_generator_module() {
    let tempdir = create_empty_crate("cargo-gen-test").unwrap();
    CargoGeneratorGenerator::new(tempdir.path().to_path_buf()).gen("app", false).unwrap();
    let content = read_file_to_string(tempdir.path().join("src/cargo_generators/app.rs")).unwrap();
    assert!(content.contains("pub struct AppGenerator"));
    assert!(content.contains("impl CargoGenerator for AppGenerator"));
}

// TODO: modify if exists.
#[test]
fn it_creates_a_cargo_generator_module() {
    let tempdir = create_empty_crate("cargo-gen-test").unwrap();
    CargoGeneratorGenerator::new(tempdir.path().to_path_buf()).gen("app", false).unwrap();
    let content = read_file_to_string(tempdir.path().join("src/cargo_generators/mod.rs")).unwrap();
    assert!(content.contains("pub mod app"));
}

#[test]
fn it_publicly_exposes_the_cargo_generator_module() {
    let tempdir = create_empty_crate("cargo-gen-test").unwrap();
    CargoGeneratorGenerator::new(tempdir.path().to_path_buf()).gen("app", false).unwrap();
    let content = read_file_to_string(tempdir.path().join("src/lib.rs")).unwrap();
    assert!(content.contains("pub mod cargo_generators"),
            format!("{} expected to contain \"pub mod cargo_generators\"",
                    content));
}

#[test]
fn it_creates_some_tests_for_the_generator() {
    let tempdir = create_empty_crate("cargo-gen-test").unwrap();
    CargoGeneratorGenerator::new(tempdir.path().to_path_buf()).gen("app", false).unwrap();
    let content = read_file_to_string(tempdir.path().join("tests/cargo_generators/app.rs"))
        .unwrap();
    assert!(content.contains("#[test]"));
    assert!(content.contains("fn it_creates_a_file"));
}

// TODO: modify if exists.
#[test]
fn it_creates_a_cargo_generator_tests_module() {
    let tempdir = create_empty_crate("cargo-gen-test").unwrap();
    CargoGeneratorGenerator::new(tempdir.path().to_path_buf()).gen("app", false).unwrap();
    let content = read_file_to_string(tempdir.path().join("tests/cargo_generators/mod.rs"))
        .unwrap();
    assert!(content.contains("mod app"));
}

#[test]
fn generated_code_passes_the_generated_tests() {
    let tempdir = create_empty_crate("cargo-gen-test").unwrap();
    CargoGeneratorGenerator::new(tempdir.path().to_path_buf()).gen("app", false).unwrap();
    // Patch the cargo-gen-helpers dependency to point to the current project.
    modify_file(tempdir.path().join("Cargo.toml"), |contents| {
            let replaced =
                contents.replace(&format!("cargo-gen-helpers = \"{}\"", env!("CARGO_PKG_VERSION")),
                                 &format!("cargo-gen-helpers = {{ path = \"{}\" }}",
                                          env!("CARGO_MANIFEST_DIR")));
            Ok(Some(replaced))
        })
        .unwrap();
    run_generated_tests(tempdir.path().to_path_buf()).unwrap();
}

#[test]
fn it_adds_cargo_gen_helpers_as_a_dependency() {
    let tempdir = create_empty_crate("cargo-gen-test").unwrap();
    let cargo_toml = read_file_to_string(tempdir.path().join("Cargo.toml")).unwrap();
    assert!(!cargo_toml.contains("cargo-gen-helpers = "),
            format!("{} should not contain the cargo-gen-helpers dependency",
                    cargo_toml));
    CargoGeneratorGenerator::new(tempdir.path().to_path_buf()).gen("app", false).unwrap();
    let cargo_toml = read_file_to_string(tempdir.path().join("Cargo.toml")).unwrap();
    assert!(cargo_toml.contains("[dependencies]\ncargo-gen-helpers = "),
            format!("{} should contain the cargo-gen-helpers dependency",
                    cargo_toml));
}

#[test]
fn it_adds_the_cargo_generator_entry_into_cargo_toml_package_metadata() {
    let tempdir = create_empty_crate("cargo-gen-test").unwrap();
    let original_cargo_toml = read_file_to_string(tempdir.path().join("Cargo.toml")).unwrap();
    CargoGeneratorGenerator::new(tempdir.path().to_path_buf()).gen("app", false).unwrap();
    let new_cargo_toml = read_file_to_string(tempdir.path().join("Cargo.toml")).unwrap();
    let expected_content = "[package.metadata.cargo_generators.\"cargo_gen_test.app\"]\n\
                            single_line_description = \"An app generator.\"\n\
                            command = \"cargo_gen_test::cargo_generators::app::AppGenerator\"";
    assert!(!original_cargo_toml.contains(expected_content),
            format!("{} should not contain {}", new_cargo_toml, expected_content));
    assert!(new_cargo_toml.contains(expected_content),
            format!("{} should contain {}", new_cargo_toml, expected_content));
}
