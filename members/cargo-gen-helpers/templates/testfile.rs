extern crate cargo_gen_helpers;
extern crate cargo_gen_test;

use self::cargo_gen_helpers::CargoGenerator;
use self::cargo_gen_helpers::test_helpers::{read_file_to_string, create_empty_crate,
                                            run_generated_tests};
use self::cargo_gen_test::cargo_generators::app::AppGenerator;
use std::vec::IntoIter;

fn args<'a>(suffix: &'a [&str]) -> IntoIter<&'a str> {
    let mut a = vec!["cargo", "gen", "gen-test.app"];
    a.extend(suffix.iter());
    a.into_iter()
}

fn run_with_args(a: IntoIter<&str>) {
    AppGenerator::new().gen(a).unwrap();
}

#[test]
fn it_creates_a_file_with_a_function_and_a_test() {
    let crate_dir = create_empty_crate("gen-test").unwrap();
    run_with_args(args(&["--crate-root", crate_dir.path().to_str().unwrap()]));
    let content = read_file_to_string(crate_dir.path().join("src/lib.rs")).unwrap();
    let expected = "pub fn add_2(n: isize) -> isize {";
    assert!(content.contains(expected),
            format!("{} expected to contain {}", content, expected));
}

#[test]
fn it_creates_a_unit_test() {
    let crate_dir = create_empty_crate("gen-test").unwrap();
    run_with_args(args(&["--crate-root", crate_dir.path().to_str().unwrap()]));
    let content = read_file_to_string(crate_dir.path().join("src/lib.rs")).unwrap();
    let expected = "#[test]\n    fn it_adds_2() {";
    assert!(content.contains(expected),
            format!("{} expected to contain {}", content, expected));
}

#[test]
fn it_creates_an_integration_test() {
    let crate_dir = create_empty_crate("gen-test").unwrap();
    run_with_args(args(&["--crate-root", crate_dir.path().to_str().unwrap()]));
    let content = read_file_to_string(crate_dir.path().join("tests/adds_2.rs")).unwrap();
    let expected = "#[test]\nfn it_adds_2() {";
    assert!(content.contains(expected),
            format!("{} expected to contain {}", content, expected));
}

#[test]
fn generated_code_passes_the_generated_tests() {
    let crate_dir = create_empty_crate("gen-test").unwrap();
    run_with_args(args(&["--crate-root", crate_dir.path().to_str().unwrap()]));
    run_generated_tests(crate_dir.path().to_path_buf()).unwrap();
}
