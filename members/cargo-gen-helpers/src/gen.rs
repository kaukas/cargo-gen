extern crate clap;

use askama::Template;
use errors::*;
use gen_trait::CargoGenerator;
use helpers::{create_file, modify_file};
use self::clap::{App, Arg, SubCommand};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

struct CLArgs {
    root: PathBuf,
    short_name: String,
}

impl<I, T> From<I> for CLArgs
    where I: IntoIterator<Item = T>,
          T: Into<OsString> + Clone
{
    fn from(clargs: I) -> CLArgs {
        let args = App::new("")
            .subcommand(SubCommand::with_name("gen")
                .subcommand(SubCommand::with_name("cargo_generator.generator")
                    .about("Generate a scaffold of an empty but functional generator")
                    .arg(Arg::with_name("GENERATOR_NAME")
                        .help("The short (unqualified) name of the generator")
                        .required(true)
                        .index(1))
                    .arg(Arg::with_name("crate-root")
                        .help("The root folder of the crate")
                        .long("crate-root")
                        .value_name("FOLDER")
                        .default_value("."))))
            .get_matches_from(clargs);
        let gen_args = args.subcommand_matches("gen")
            .expect("'gen' subcommand expected but not provided");
        let cgargs = gen_args.subcommand_matches("cargo_generator.generator")
            .expect("'cargo_generator.generator' subcommand expected but not provided");
        CLArgs {
            short_name: cgargs.value_of("GENERATOR_NAME").unwrap().to_owned(),
            root: Path::new(cgargs.value_of("crate-root").unwrap()).to_path_buf(),
        }
    }
}

#[derive(Template)]
#[template(path = "genfile.rs")]
struct GenFileTemplate {}

#[derive(Template)]
#[template(path = "testfile.rs")]
struct TestFileTemplate {}

pub struct CargoGeneratorGenerator {}

impl CargoGeneratorGenerator {
    pub fn new() -> CargoGeneratorGenerator {
        CargoGeneratorGenerator {}
    }
}

impl CargoGenerator for CargoGeneratorGenerator {
    fn gen<I, T>(&self, args: I) -> Result<()>
        where I: IntoIterator<Item = T>,
              T: Into<OsString> + Clone
    {
        let args = CLArgs::from(args);

        // cargo generators module
        // TODO: modify existing
        create_file(PathBuf::from(args.root.join("src/cargo_generators/mod.rs")),
                    "pub mod app;")?;

        // cargo generator
        let gen_file_content = GenFileTemplate {}.render()?;
        let path = args.root.join(format!("src/cargo_generators/{}.rs", args.short_name));
        create_file(path, &gen_file_content)?;

        // expose cargo generators in lib.rs
        modify_file(PathBuf::from(args.root.join("src/lib.rs")),
                    |mut contents| {
                        contents.insert_str(0, "pub mod cargo_generators;\n");
                        Ok(Some(contents))
                    })?;

        // cargo generators test loader module
        create_file(PathBuf::from(args.root.join("tests/cargo_gen.rs")),
                    "mod cargo_generators;")?;

        // cargo gen test module
        // TODO: modify existing
        create_file(PathBuf::from(args.root.join(format!("tests/cargo_generators/mod.rs"))),
                    "mod app;")?;

        // cargo gen test
        let test_file_content = TestFileTemplate {}.render()?;
        let path = args.root.join(format!("tests/cargo_generators/{}.rs", args.short_name));
        create_file(path, &test_file_content)?;

        // Cargo.toml
        // TODO: move to helper
        modify_file(PathBuf::from(args.root.join("Cargo.toml")),
                    |mut contents| {
            // TODO: use a TOML parser that preserves order, whitespace, etc. At the moment the
            // toml crate does not.

            // Add the cargo-gen-helpers dependency.
            let deps_str = "[dependencies]\n";
            let crate_version = env!("CARGO_PKG_VERSION");
            // FIXME: pick clap version from elsewhere.
            let gen_helper_dep = format!("{}cargo-gen-helpers = \"{}\"\nclap = \"2.27\"",
                                         deps_str,
                                         crate_version);
            contents = contents.replace(deps_str, &gen_helper_dep);

            // FIXME: use the actual crate name.
            contents.push_str("\n[package.metadata.cargo_generators.\"cargo_gen_test.app\"]\n\
                               single_line_description = \"An app generator.\"\n\
                               command = \"cargo_gen_test::cargo_generators::app::AppGenerator\"\
                               \n");
            Ok(Some(contents))
        })
    }
}

#[cfg(test)]
mod args_parsing {
    use std::path::Path;
    use super::CLArgs;
    use std::vec::IntoIter;

    fn args<'a>(suffix: &'a [&str]) -> IntoIter<&'a str> {
        let mut a = vec!["cargo", "gen", "cargo_generator.generator"];
        a.extend(suffix.iter());
        a.into_iter()
    }

    #[test]
    fn it_parses_the_generator_name() {
        assert_eq!("app", CLArgs::from(args(&["app"])).short_name);
    }

    #[test]
    fn it_parses_the_crate_root() {
        assert_eq!(Path::new("/tmp").to_path_buf(),
                   CLArgs::from(args(&["app", "--crate-root", "/tmp"])).root);
    }

    #[test]
    fn it_defaults_to_the_current_folder() {
        assert_eq!(Path::new(".").to_path_buf(),
                   CLArgs::from(args(&["app"])).root);
    }
}