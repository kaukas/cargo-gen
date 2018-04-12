use askama::Template;
use errors::*;
use gen_trait::CargoGenerator;
use helpers::{create_file, modify_file};
use clap::{App, SubCommand};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

#[derive(Template)]
#[template(path = "genfile.rs")]
struct GenFileTemplate {}

#[derive(Template)]
#[template(path = "testfile.rs")]
struct TestFileTemplate {}

#[derive(Template)]
#[template(path = "cargo_generators.yaml")]
struct ClapYamlFileTemplate {}

pub struct CargoGeneratorGenerator {
    root: PathBuf,
    short_name: String,
}

impl<I, T> From<I> for CargoGeneratorGenerator
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    fn from(clargs: I) -> CargoGeneratorGenerator {
        let yml = load_yaml!("../cargo_generators.yaml");
        let args = App::new("")
            .subcommand(SubCommand::with_name("gen").subcommand(SubCommand::from_yaml(&yml[0])))
            .get_matches_from(clargs);
        let gen_args = args.subcommand_matches("gen")
            .expect("'gen' subcommand expected but not provided");
        let cgargs = gen_args
            .subcommand_matches("cargo-gen.generator")
            .expect("'cargo-gen.generator' subcommand expected but not provided");
        CargoGeneratorGenerator {
            short_name: cgargs.value_of("GENERATOR_NAME").unwrap().to_owned(),
            root: Path::new(cgargs.value_of("crate-root").unwrap()).to_path_buf(),
        }
    }
}

impl CargoGenerator for CargoGeneratorGenerator {
    fn gen(&self) -> Result<()> {
        // cargo generators module
        // TODO: modify existing
        create_file(
            self.root.join("src/cargo_generators/mod.rs"),
            "pub mod app;",
        )?;

        // cargo generator
        let gen_file_content = GenFileTemplate {}.render()?;
        let path = self.root
            .join(format!("src/cargo_generators/{}.rs", self.short_name));
        create_file(path, &gen_file_content)?;

        // expose cargo generators in lib.rs
        modify_file(self.root.join("src/lib.rs"), |mut contents| {
            contents.insert_str(
                0,
                "#[macro_use]\nextern crate clap;\n\npub mod cargo_generators;\n",
            );
            Ok(Some(contents))
        })?;

        // cargo generators test loader module
        create_file(
            self.root.join("tests/cargo_gen.rs"),
            "mod cargo_generators;",
        )?;

        // cargo gen test module
        // TODO: modify existing
        create_file(self.root.join("tests/cargo_generators/mod.rs"), "mod app;")?;

        // cargo gen test
        let test_file_content = TestFileTemplate {}.render()?;
        let path = self.root
            .join(format!("tests/cargo_generators/{}.rs", self.short_name));
        create_file(path, &test_file_content)?;

        // Create a clap command line specifications YAML file
        // TODO: modify existing
        let clap_file_content = ClapYamlFileTemplate {}.render()?;
        let path = self.root.join("cargo_generators.yaml");
        create_file(path, &clap_file_content)?;

        // Cargo.toml
        // TODO: move to helper
        modify_file(self.root.join("Cargo.toml"), |mut contents| {
            // TODO: use a TOML parser that preserves order, whitespace, etc. At the moment the
            // toml crate does not.

            // Add the cargo-gen-helpers dependency.
            let deps_str = "[dependencies]\n";
            let crate_version = env!("CARGO_PKG_VERSION");
            // FIXME: pick clap version from elsewhere automatically.
            let clap_version = "2.31";
            let gen_helper_dep = format!(
                "{}cargo-gen-helpers = \"{}\"\n\
                 clap = {{ version = \"{}\", features = [\"yaml\"] }}\n",
                deps_str, crate_version, clap_version
            );
            contents = contents.replace(deps_str, &gen_helper_dep);

            Ok(Some(contents))
        })
    }
}

#[cfg(test)]
mod arg_parsing {
    use std::path::Path;
    use super::CargoGeneratorGenerator;
    use std::vec::IntoIter;

    fn args<'a>(suffix: &'a [&str]) -> IntoIter<&'a str> {
        let mut a = vec!["cargo", "gen", "cargo-gen.generator"];
        a.extend(suffix.iter());
        a.into_iter()
    }

    #[test]
    fn it_parses_the_generator_name() {
        assert_eq!(
            "app",
            CargoGeneratorGenerator::from(args(&["app"])).short_name
        );
    }

    #[test]
    fn it_parses_the_crate_root() {
        assert_eq!(
            Path::new("/tmp").to_path_buf(),
            CargoGeneratorGenerator::from(args(&["app", "--crate-root", "/tmp"])).root
        );
    }

    #[test]
    fn the_crate_root_defaults_to_the_current_folder() {
        assert_eq!(
            Path::new(".").to_path_buf(),
            CargoGeneratorGenerator::from(args(&["app"])).root
        );
    }
}
