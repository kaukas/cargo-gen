extern crate cargo;
extern crate cargo_gen_helpers;
extern crate tempdir;

use cargo_gen_helpers::gen::CargoGeneratorGenerator;
use cargo_gen_helpers::test_helpers::{create_empty_crate, read_file_to_string, run_generated_tests};
use cargo_gen_helpers::{modify_file, CargoGenerator};
use std::vec::IntoIter;
use tempdir::TempDir;

fn read_toml(crate_dir: &TempDir) -> String {
    read_file_to_string(crate_dir.path().join("Cargo.toml")).unwrap()
}

fn args<'a>(suffix: &'a [&str]) -> IntoIter<&'a str> {
    let mut a = vec!["cargo", "gen", "cargo_generator.generator"];
    a.extend(suffix.iter());
    a.into_iter()
}

fn run_with_args(a: IntoIter<&str>) {
    CargoGeneratorGenerator::from(a).gen().unwrap();
}

#[test]
fn it_generates_a_generator_file_in_a_cargo_generator_module() {
    // :-)
    let crate_dir = create_empty_crate("cargo-gen-test").unwrap();
    run_with_args(args(&[
        "app",
        "--crate-root",
        crate_dir.path().to_str().unwrap(),
    ]));
    let content =
        read_file_to_string(crate_dir.path().join("src/cargo_generators/app.rs")).unwrap();
    assert!(content.contains("pub struct AppGenerator"));
    assert!(content.contains("impl CargoGenerator for AppGenerator"));
}

// TODO: modify if exists.
#[test]
fn it_creates_a_cargo_generator_module() {
    let crate_dir = create_empty_crate("cargo-gen-test").unwrap();
    run_with_args(args(&[
        "app",
        "--crate-root",
        crate_dir.path().to_str().unwrap(),
    ]));
    let content =
        read_file_to_string(crate_dir.path().join("src/cargo_generators/mod.rs")).unwrap();
    assert!(content.contains("pub mod app"));
}

#[test]
fn it_publicly_exposes_the_cargo_generator_module() {
    let crate_dir = create_empty_crate("cargo-gen-test").unwrap();
    run_with_args(args(&[
        "app",
        "--crate-root",
        crate_dir.path().to_str().unwrap(),
    ]));
    let content = read_file_to_string(crate_dir.path().join("src/lib.rs")).unwrap();
    assert!(
        content.contains("pub mod cargo_generators"),
        format!(
            "{} expected to contain \"pub mod cargo_generators\"",
            content
        )
    );
}

#[test]
fn it_imports_clap_with_macros() {
    let crate_dir = create_empty_crate("cargo-gen-test").unwrap();
    run_with_args(args(&[
        "app",
        "--crate-root",
        crate_dir.path().to_str().unwrap(),
    ]));
    let content = read_file_to_string(crate_dir.path().join("src/lib.rs")).unwrap();
    assert!(
        content.contains("#[macro_use]\nextern crate clap;"),
        format!(
            "{} expected to contain \"#[macro_use]\nextern crate clap;\"",
            content
        )
    );
}

#[test]
fn it_creates_some_tests_for_the_generator() {
    let crate_dir = create_empty_crate("cargo-gen-test").unwrap();
    run_with_args(args(&[
        "app",
        "--crate-root",
        crate_dir.path().to_str().unwrap(),
    ]));
    let content =
        read_file_to_string(crate_dir.path().join("tests/cargo_generators/app.rs")).unwrap();
    assert!(content.contains("#[test]"));
    assert!(content.contains("fn it_creates_a_file"));
}

// TODO: modify if exists.
#[test]
fn it_creates_a_cargo_generator_tests_module() {
    let crate_dir = create_empty_crate("cargo-gen-test").unwrap();
    run_with_args(args(&[
        "app",
        "--crate-root",
        crate_dir.path().to_str().unwrap(),
    ]));
    let content =
        read_file_to_string(crate_dir.path().join("tests/cargo_generators/mod.rs")).unwrap();
    assert!(content.contains("mod app"));
}

#[test]
fn generated_code_passes_the_generated_tests() {
    let crate_dir = create_empty_crate("cargo-gen-test").unwrap();
    run_with_args(args(&[
        "app",
        "--crate-root",
        crate_dir.path().to_str().unwrap(),
    ]));
    // Patch the cargo-gen-helpers dependency to point to the current project.
    modify_file(crate_dir.path().join("Cargo.toml"), |contents| {
        let replaced = contents.replace(
            &format!("cargo-gen-helpers = \"{}\"", env!("CARGO_PKG_VERSION")),
            &format!(
                "cargo-gen-helpers = {{ path = \"{}\" }}",
                env!("CARGO_MANIFEST_DIR")
            ),
        );
        Ok(Some(replaced))
    }).unwrap();
    run_generated_tests(&crate_dir.path().to_path_buf()).unwrap();
}

#[test]
fn it_adds_cargo_gen_helpers_as_a_dependency() {
    let crate_dir = create_empty_crate("cargo-gen-test").unwrap();
    let cargo_toml = read_toml(&crate_dir);
    assert!(
        !cargo_toml.contains("cargo-gen-helpers = "),
        format!(
            "{} should not contain the cargo-gen-helpers dependency",
            cargo_toml
        )
    );
    run_with_args(args(&[
        "app",
        "--crate-root",
        crate_dir.path().to_str().unwrap(),
    ]));
    let cargo_toml = read_toml(&crate_dir);
    assert!(
        cargo_toml.contains("cargo-gen-helpers = "),
        format!(
            "{} should contain the cargo-gen-helpers dependency",
            cargo_toml
        )
    );
}

#[test]
fn it_adds_clap_with_yaml_as_a_dependency() {
    let crate_dir = create_empty_crate("cargo-gen-test").unwrap();
    let cargo_toml = read_toml(&crate_dir);
    assert!(
        !cargo_toml.contains("clap = "),
        format!("{} should not contain the clap dependency", cargo_toml)
    );
    run_with_args(args(&[
        "app",
        "--crate-root",
        crate_dir.path().to_str().unwrap(),
    ]));
    let cargo_toml = read_toml(&crate_dir);
    assert!(
        cargo_toml.contains("clap = "),
        format!("{} should contain the clap dependency", cargo_toml)
    );
    assert!(
        cargo_toml.contains("features = [\"yaml\"]"),
        format!("{} should enable the YAML parser", cargo_toml)
    );
}

#[test]
fn it_creates_a_cargo_generators_yaml_file() {
    let crate_dir = create_empty_crate("cargo-gen-test").unwrap();
    run_with_args(args(&[
        "app",
        "--crate-root",
        crate_dir.path().to_str().unwrap(),
    ]));
    let content = read_file_to_string(crate_dir.path().join("cargo_generators.yml")).unwrap();
    assert!(content.contains("name: gen-test.app"));
}
