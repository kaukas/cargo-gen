extern crate cargo_gen_helpers;
extern crate cargo_gen_test;

use self::cargo_gen_helpers::test_helpers::{read_file_to_string, create_empty_crate};
use self::cargo_gen_helpers::CargoGenerator;
use self::cargo_gen_test::cargo_generators::app::AppGenerator;

#[test]
fn it_creates_a_file() {
    let crate_dir = create_empty_crate();
    AppGenerator::new(crate_dir.path().to_path_buf()).gen("app", false).unwrap();
    let content = read_file_to_string(crate_dir.path().join("src/lib.rs")).unwrap();
    let expected = "fn foo() {";
    assert!(content.contains(expected), format!("{} expected to contain {}", content, expected));
}
