extern crate cargo_gen_helpers;

use self::cargo_gen_helpers::{CargoGenerator, create_file, modify_file};
use self::cargo_gen_helpers::errors::Result as CGHResult;
use std::path::PathBuf;

pub struct AppGenerator {
    crate_path: PathBuf,
}

impl AppGenerator {
    pub fn new(crate_path: PathBuf) -> AppGenerator {
        AppGenerator { crate_path: crate_path }
    }
}

impl CargoGenerator for AppGenerator {
    fn gen(&self, _short_name: &str, _dry_run: bool) -> CGHResult<()> {
        modify_file(self.crate_path.join("src/lib.rs"), |mut contents| {
            contents.insert_str(0, "pub fn add_2(n: isize) -> isize {\n    n + 2\n}\n\n");
            Ok(Some(contents.replace("it_works", "it_adds_2")
                .replace("assert!(2 + 2, 4)", "assert_eq!(4, add_2(2))")))
        })?;
        // FIXME: use the actual crate name
        create_file(self.crate_path.join("tests/adds_2.rs"),
                    "extern crate app_test;\n\n\
                     #[test]\n\
                     fn it_adds_2() {\n    \
                        assert_eq!(4, app_test::add_2(2));\n\
                     }")
    }
}
