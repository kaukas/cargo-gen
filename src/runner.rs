extern crate cargo_gen_helpers;
extern crate tempdir;

use self::cargo_gen_helpers::test_helpers::create_custom_empty_crate;
use self::cargo_gen_helpers::test_helpers::read_file_to_string;
use self::cargo_gen_helpers::FileHelper;
use self::tempdir::TempDir;
use askama::Template;
use cargo::core::Workspace;
use cargo::ops;
use cargo::util::Config as CargoConfig;
use failure::{Error, SyncFailure};
use gen::Generator;
use std::ffi::OsStr;
use std::path::Path;

struct EntryPoint {
    ep: String,
}

impl EntryPoint {
    fn crate_name(&self) -> String {
        self.ep.split(':').next().unwrap().to_string()
    }
}

#[derive(Template)]
#[template(path = "main.rs.tmpl")]
struct MainFileTmpl {
    pub factory: EntryPoint,
}

fn setup_runner_crate(generator: Generator) -> Result<TempDir, Error> {
    let crate_name = format!("cargo-gen-{}", generator.name);
    let tempdir = TempDir::new(&crate_name)?;
    create_custom_empty_crate(&crate_name, tempdir.path(), true, false).map_err(SyncFailure::new)?;
    let file_helper = FileHelper::new(false);
    let main_rs_content = MainFileTmpl {
        factory: EntryPoint {
            ep: generator.factory,
        },
    }.render()
        .map_err(SyncFailure::new)?;
    file_helper
        .modify_file(tempdir.path().join("src/main.rs"), |_| {
            Ok(Some(main_rs_content))
        })
        .map_err(SyncFailure::new)?;
    println!(
        "{}",
        read_file_to_string(tempdir.path().join("src/main.rs")).unwrap()
    );
    Ok(tempdir)
}

fn run_runner_crate(tempdir: &TempDir) -> Result<(), Error> {
    let config = CargoConfig::default().map_err(SyncFailure::new)?;
    config
        .configure(0, Some(false), &None, false, false, &[])
        .map_err(SyncFailure::new)?;

    let manifest_path = tempdir.path().join("Cargo.toml");

    let workspace = Workspace::new(&manifest_path, &config).map_err(SyncFailure::new)?;
    let pkg = workspace.current().map_err(SyncFailure::new)?;
    let compile_options = ops::CompileOptions::default(&config, ops::CompileMode::Build);
    let compile = ops::compile(&workspace, &compile_options).map_err(SyncFailure::new)?;
    let mut process = compile
        .target_process(compile.binaries[0].to_path_buf(), pkg)
        .map_err(SyncFailure::new)?;
    process.cwd(config.cwd());
    config
        .shell()
        .status("Running", process.to_string())
        .map_err(SyncFailure::new)?;
    process.exec().map_err(SyncFailure::new)?;
    Ok(())
}

pub fn run<P>(_root_crate_path: P, generator: Generator) -> Result<(), Error>
where
    P: AsRef<Path> + AsRef<OsStr>,
{
    let tempdir = setup_runner_crate(generator)?;
    run_runner_crate(&tempdir).unwrap();
    Ok(())
}

#[cfg(test)]
mod runner_test {
    extern crate cargo_gen_helpers;

    use self::cargo_gen_helpers::test_helpers::create_empty_crate;
    use self::cargo_gen_helpers::FileHelper;
    use super::run;
    use gen::Generator;
    use std::path::Path;

    #[test]
    fn it_runs_the_supplied_generator() {
        let crate_dir = create_empty_crate("cargo-gen-test").unwrap();
        let gen_helpers_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("members/cargo-gen-helpers")
            .as_os_str();
        FileHelper::new(false)
            .modify_file(crate_dir.path().join("Cargo.toml"), |contents| {
                let deps_str = "[dependencies]\n";
                let new_deps_str = format!(
                    "{}cargo-gen-helpers = {{ path = {:?} }}\n",
                    deps_str, gen_helpers_path
                );
                Ok(Some(contents.replace(deps_str, &new_deps_str)))
            })
            .unwrap();

        // FIXME: maybe avoid hardcoding the factory like that?..
        let generator = Generator {
            name: "x".to_string(),
            crate_path: gen_helpers_path,
            factory: "cargo_gen_helpers::gen::make_cargo_gen_gen".to_string(),
        };
        run(crate_dir.path(), generator).unwrap();
        assert!(crate_dir.path().join("cargo_generators.yaml").exists());
    }
}
