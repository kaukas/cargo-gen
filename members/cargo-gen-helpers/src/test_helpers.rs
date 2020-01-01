use cargo::core::Workspace;
use cargo::ops;
use cargo::util::Config as CargoConfig;
use errors::*;
use helpers::FileHelper;
use std::ffi::OsStr;
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
    create_custom_empty_crate(name, tempdir.path(), false, true)?;
    Ok(tempdir)
}

pub fn create_custom_empty_crate<P>(name: &str, root_dir: P, bin: bool, lib: bool) -> Result<()>
where
    P: AsRef<Path> + AsRef<OsStr>,
{
    let config = CargoConfig::default();
    let options = if let Some(root) = Path::new(&root_dir).to_str() {
        ops::NewOptions::new(None, bin, lib, root, Some(name))
    } else {
        bail!("Failed to construct the root directory path string")
    };
    ops::init(options, &config?)?;
    Ok(())
}

fn find_lockfile() -> Result<PathBuf> {
    let current_path = Path::new(env!("CARGO_MANIFEST_DIR"));
    // The "x" at the end makes the current_path a parent entry and allows us to not special case
    // the logic in `while let` for the current_path.
    let mut path = current_path.join("x");
    while let Some(parent_path) = path.to_owned().parent() {
        path = parent_path.to_path_buf();
        let lock_path = path.join("Cargo.lock");
        if lock_path.is_file() {
            return Ok(lock_path);
        }
    }
    bail!(format!(
        "Could not find Cargo.lock in {} or any of its parent directories",
        current_path.display()
    ))
}

pub fn run_generated_tests(path: &PathBuf) -> Result<()> {
    // We want to avoid any network access and minimise dependent crate compilations. Reuse as many
    // dependencies from the current project as possible. To achieve that we use the Cargo.lock
    // from the current project. It is a hack since the lockfile does not apply but Cargo seems to
    // be able to pick up the applicable pieces from it.
    FileHelper::new(false).create_file(
        path.join("Cargo.lock"),
        &read_file_to_string(find_lockfile()?)?,
    )?;

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
