extern crate assert_cli;
extern crate cargo_gen_helpers;

use assert_cli::Assert;
// use cargo_gen_helpers::test_helpers::create_empty_crate;

#[test]
fn it_prints_the_help_text_when_called_without_arguments() {
    // let crate_dir = create_empty_crate("gen-test").unwrap();
    Assert::cargo_binary("cargo-gen")
        .fails()
        .and()
        .stderr()
        .contains("--help")
        .unwrap();
}

#[test]
fn it_prints_the_help_text_when_called_with_the_subcommand_only() {
    Assert::cargo_binary("cargo-gen")
        .with_args(&["gen"])
        .fails()
        .and()
        .stderr()
        .contains("--help")
        .unwrap();
}

#[test]
#[ignore]
fn it_fails_when_both_list_and_subcommand_provided() {
    Assert::cargo_binary("cargo-gen")
        .with_args(&["gen", "--list", "app"])
        .fails()
        .and()
        .stderr()
        .contains("--help")
        .unwrap();
}

#[test]
fn it_returns_a_list_of_available_generators() {
    Assert::cargo_binary("cargo-gen")
        .with_args(&["gen", "--list"])
        .stdout()
        .is("cargo-gen.generator\n")
        .unwrap();
}
