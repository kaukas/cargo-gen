extern crate cargo_gen_helpers;
extern crate tempfile;

mod test_read_file_to_string {
    use std::io::Write;
    use tempfile::NamedTempFile;
    use cargo_gen_helpers::test_helpers::read_file_to_string;

    #[test]
    fn it_reads_the_file_content_to_a_string() {
        let mut tempfile = NamedTempFile::new().unwrap();
        tempfile.write_all(b"Some data").unwrap();
        assert_eq!(
            "Some data".to_string(),
            read_file_to_string(tempfile.path().to_path_buf()).unwrap()
        );
    }
}

mod test_create_empty_crate {
    use cargo_gen_helpers::test_helpers::{create_empty_crate, read_file_to_string};

    #[test]
    fn it_generates_a_new_bare_crate() {
        let crate_dir = create_empty_crate("cargo-gen-test").unwrap();
        let cargo_toml = read_file_to_string(crate_dir.path().join("Cargo.toml")).unwrap();
        assert!(
            cargo_toml.contains("name = \"cargo-gen-test\""),
            format!(
                "{} should contain metadata for the cargo-gen-test crate",
                cargo_toml
            )
        )
    }
}

mod test_run_generated_tests {
    use cargo_gen_helpers::FileHelper;
    use cargo_gen_helpers::test_helpers::{create_empty_crate, run_generated_tests};

    // TODO: find a way to test the failing test case.
    // #[test]
    // #[should_panic]
    // fn it_panics_when_the_generated_test_fails() {
    //     let crate_dir = create_empty_crate("cargo-gen-test").unwrap();
    //     create_file(crate_dir.path().join("tests/foo.rs"),
    //                 "#[test]\nfn t() { assert!(false); }")
    //         .unwrap();
    //     run_generated_tests(crate_dir.path().to_path_buf()).unwrap()
    // }

    #[test]
    fn it_runs_the_generated_tests() {
        let crate_dir = create_empty_crate("cargo-gen-test").unwrap();
        FileHelper::new(false)
            .create_file(
                crate_dir.path().join("tests/foo.rs"),
                "#[test]\nfn t() { assert!(true); }",
            )
            .unwrap();
        run_generated_tests(&crate_dir.path().to_path_buf()).unwrap()
    }
}
