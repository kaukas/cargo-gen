extern crate cargo_gen_helpers;
extern crate clap;

use self::cargo_gen_helpers::errors::Result as CGHResult;
use self::cargo_gen_helpers::{create_file, modify_file, CargoGenerator};
use self::clap::{App, Arg, SubCommand};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

pub struct AppGenerator {
    root: PathBuf,
}

impl<I, T> From<I> for AppGenerator
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    fn from(clargs: I) -> AppGenerator {
        let args = App::new("")
            .subcommand(
                SubCommand::with_name("gen").subcommand(
                    SubCommand::with_name("gen-test.app")
                        .about("Generate an app")
                        .arg(
                            Arg::with_name("crate-root")
                                .help("The root folder of the crate")
                                .long("crate-root")
                                .value_name("FOLDER")
                                .default_value("."),
                        ),
                ),
            )
            .get_matches_from(clargs);
        let gen_args = args.subcommand_matches("gen")
            .expect("'gen' subcommand expected but not provided");
        // FIXME: real crate name
        let app_args = gen_args
            .subcommand_matches("gen-test.app")
            .expect("'gen-test.app' subcommand expected but not provided");
        AppGenerator {
            root: Path::new(app_args.value_of("crate-root").unwrap()).to_path_buf(),
        }
    }
}

impl CargoGenerator for AppGenerator {
    fn gen(&self) -> CGHResult<()> {
        modify_file(self.root.join("src/lib.rs"), |mut contents| {
            contents.insert_str(0, "pub fn add_2(n: isize) -> isize {\n    n + 2\n}\n\n");
            Ok(Some(
                contents
                    .replace("it_works", "it_adds_2")
                    .replace("assert!(2 + 2, 4)", "assert_eq!(4, add_2(2))"),
            ))
        })?;
        // FIXME: use the actual crate name
        create_file(
            self.root.join("tests/adds_2.rs"),
            "extern crate gen_test;\n\n\
             #[test]\n\
             fn it_adds_2() {\n    \
             assert_eq!(4, gen_test::add_2(2));\n\
             }",
        )
    }
}

#[cfg(test)]
mod args_parsing {
    use std::path::Path;
    use super::AppGenerator;
    use std::vec::IntoIter;

    fn args<'a>(suffix: &'a [&str]) -> IntoIter<&'a str> {
        let mut a = vec!["cargo", "gen", "gen-test.app"];
        a.extend(suffix.iter());
        a.into_iter()
    }

    #[test]
    fn it_parses_the_crate_root() {
        assert_eq!(
            Path::new("/tmp").to_path_buf(),
            AppGenerator::from(args(&["--crate-root", "/tmp"])).root
        );
    }

    #[test]
    fn it_defaults_to_the_current_folder() {
        assert_eq!(
            Path::new(".").to_path_buf(),
            AppGenerator::from(args(&[])).root
        );
    }
}
