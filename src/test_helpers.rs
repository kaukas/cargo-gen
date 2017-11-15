use cargo::core::Workspace;
use cargo::ops;
use cargo::util::Config as CargoConfig;
use errors::*;
use helpers::create_file;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use tempdir::TempDir;

pub fn read_file_to_string(path: PathBuf) -> Result<String> {
    let mut content = String::new();
    File::open(path)?.read_to_string(&mut content)?;
    Ok(content)
}

pub fn create_empty_crate(name: &str) -> Result<TempDir> {
    let tempdir = TempDir::new(name)?;
    {
        let config = CargoConfig::default();
        let options = if let Some(root) = tempdir.path().to_str() {
            ops::NewOptions::new(None, false, true, root, Some(name))
        } else {
            bail!("Failed to construct the tempdir path string")
        };
        ops::init(options, &config?)?;
    }
    Ok(tempdir)
}

pub fn run_generated_tests(path: PathBuf) -> Result<()> {
    // We want to avoid any network access and minimise dependent crate compilations. Reuse as many
    // dependencies form the current project as possible. To achieve that we use the Cargo.lock
    // from the current project. It is a hack since the lockfile does not apply but Cargo seems to
    // be able to pick up the right pieces from it.
    create_file(path.join("Cargo.lock"),
                &read_file_to_string(Path::new("./Cargo.lock").to_path_buf())?)?;

    let config = CargoConfig::default()?;
    config.configure(0, Some(false), &None, false, false, &[])?;
    let test_options = ops::TestOptions {
        no_run: false,
        no_fail_fast: false,
        only_doc: false,
        compile_opts: ops::CompileOptions::default(&config, ops::CompileMode::Test),
    };
    let manifest_path = path.join("Cargo.toml");
    let workspace = Workspace::new(&manifest_path, &config)?;
    ops::run_tests(&workspace, &test_options, &[])?;
    Ok(())
}
