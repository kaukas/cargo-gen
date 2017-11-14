extern crate cargo_gen_helpers;

use self::cargo_gen_helpers::{CargoGenerator, modify_file};
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
            contents.insert_str(0, "fn foo() {\n\n}\n");
            Ok(Some(contents))
        })
    }
}
