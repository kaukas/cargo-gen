use askama::Template;
use errors::*;
use helpers::{create_file, modify_file};
use gen_trait::CargoGenerator;
use std::path::PathBuf;

#[derive(Template)]
#[template(path = "genfile.rs")]
struct GenFileTemplate {}

#[derive(Template)]
#[template(path = "testfile.rs")]
struct TestFileTemplate {}

pub struct CargoGeneratorGenerator {
    project_path: PathBuf,
}

impl CargoGeneratorGenerator {
    pub fn new(project_path: PathBuf) -> CargoGeneratorGenerator {
        CargoGeneratorGenerator { project_path: project_path }
    }
}

impl CargoGenerator for CargoGeneratorGenerator {
    fn gen(&self, short_name: &str, _dry_run: bool) -> Result<()> {
        // cargo generators module
        // TODO: modify existing
        create_file(PathBuf::from(self.project_path.join("src/cargo_generators/mod.rs")),
                    "pub mod app;")?;

        // cargo generator
        let gen_file_content = GenFileTemplate {}.render()?;
        let path = self.project_path.join(format!("src/cargo_generators/{}.rs", short_name));
        create_file(path, &gen_file_content)?;

        // expose cargo generators in lib.rs
        modify_file(PathBuf::from(self.project_path.join("src/lib.rs")),
                    |mut contents| {
                        contents.insert_str(0, "pub mod cargo_generators;\n");
                        Ok(Some(contents))
                    })?;

        // cargo generators test loader module
        create_file(PathBuf::from(self.project_path.join("tests/cargo_gen.rs")),
                    "mod cargo_generators;")?;

        // cargo gen test module
        // TODO: modify existing
        create_file(PathBuf::from(self.project_path
                        .join(format!("tests/cargo_generators/mod.rs"))),
                    "mod app;")?;

        // cargo gen test
        let test_file_content = TestFileTemplate {}.render()?;
        let path = self.project_path.join(format!("tests/cargo_generators/{}.rs", short_name));
        create_file(path, &test_file_content)?;

        // Cargo.toml
        // TODO: move to helper
        modify_file(PathBuf::from(self.project_path.join("Cargo.toml")),
                    |mut contents| {
            // TODO: use a TOML parser that preserves order, whitespace, etc. At the moment the
            // toml crate does not.

            // Add the cargo-gen-helpers dependency.
            let deps_str = "[dependencies]\n";
            let crate_version = env!("CARGO_PKG_VERSION");
            let gen_helper_dep = format!("{}cargo-gen-helpers = \"{}\"", deps_str, crate_version);
            contents = contents.replace(deps_str, &gen_helper_dep);

            contents.push_str("\n[package.metadata.cargo_generators.\"cargo_gen_test.app\"]\n\
                               single_line_description = \"An app generator.\"\n\
                               command = \"cargo_gen_test::cargo_generators::app::AppGenerator\"\
                               \n");
            Ok(Some(contents))
        })
    }
}
