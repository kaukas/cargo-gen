extern crate cargo_metadata;
extern crate yaml_rust;

use std::fs::File;
use std::ffi::OsStr;
use std::io::Read;
use std::path::{Path, PathBuf};
use self::yaml_rust::{Yaml, YamlLoader};
use self::cargo_metadata::metadata_deps;
use failure::{err_msg, Error, SyncFailure};

#[derive(Debug)]
pub struct Generator {
    name: String,
    factory: String,
}

impl Generator {
    // FIXME: more context in error messages.
    fn try_from_yaml(yaml_doc: &Yaml) -> Result<Generator, Error> {
        let gen_hash = yaml_doc
            .as_hash()
            .ok_or(err_msg("A generator config is not a hash"))?;
        let name = gen_hash
            .get(&Yaml::from_str("name"))
            .ok_or(err_msg("A generator name is not present"))?
            .as_str()
            .ok_or(err_msg("A generator name is not a string"))?
            .to_owned();
        let factory = gen_hash
            .get(&Yaml::from_str("factory"))
            .ok_or(format_err!(
                "A generator {} does not have a factory defined",
                name
            ))?
            .as_str()
            .ok_or(format_err!(
                "A factory of generator {} is not a string",
                name
            ))?
            .to_owned();
        Ok(Generator {
            name: name,
            factory: factory,
        })
    }
}

pub fn find_all<P>(root_crate_path: P) -> Vec<Result<Generator, Error>>
where
    P: AsRef<Path> + AsRef<OsStr>,
{
    // Find roots of all crates.
    match list_dep_root_dirs(root_crate_path) {
        Err(e) => vec![Err(e)],
        Ok(dep_root_dirs) => {
            // Find all cargo_generators.yaml's in all roots.
            let cg_yamls = find_yaml_files_in_dirs(dep_root_dirs);
            // Parse each yaml
            let parsed_yamls = parse_yamls(cg_yamls);
            // Parse generators
            parse_generators(parsed_yamls)
        }
    }
}

fn list_dep_root_dirs<P>(root_crate_path: P) -> Result<Vec<PathBuf>, Error>
where
    P: AsRef<Path> + AsRef<OsStr>,
{
    let manifest_path = Path::new(&root_crate_path).join("Cargo.toml");
    let metadata = metadata_deps(Some(&manifest_path), true).map_err(SyncFailure::new)?;
    Ok(metadata
        .packages
        .iter()
        .map(|package| Path::new(&package.manifest_path))
        .filter_map(|path| path.parent()) // Drop the Cargo.toml at the end.
        .map(|path| path.to_path_buf())
        .collect())
}

fn find_yaml_files_in_dirs(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    paths
        .iter()
        .map(|p| p.join("cargo_generators.yaml"))
        .filter(|path| path.is_file())
        .collect()
}

fn parse_yamls(paths: Vec<PathBuf>) -> Vec<Result<Yaml, Error>> {
    let mut results: Vec<Result<Yaml, Error>> = Vec::new();

    for path in paths {
        let mut yaml_str = String::new();
        // FIXME: panics
        File::open(path)
            .unwrap()
            .read_to_string(&mut yaml_str)
            .unwrap();
        match YamlLoader::load_from_str(&yaml_str) {
            Err(err) => results.push(Err(Error::from(err))),
            Ok(yamls) => match yamls[0].as_vec() {
                None => results.push(Err(err_msg("A generators YAML file is not an array"))),
                Some(yamls) => results.extend(yamls.iter().map(|yaml| Ok(yaml.clone()))),
            },
        }
    }
    results
}

fn parse_generators(yamls: Vec<Result<Yaml, Error>>) -> Vec<Result<Generator, Error>> {
    yamls
        .into_iter()
        .map(|res| res.and_then(|yaml| Generator::try_from_yaml(&yaml)))
        .collect()
}

#[cfg(test)]
mod from_yaml_test {
    extern crate cargo_gen_helpers;

    use super::*;

    #[test]
    fn it_parses_generator_name_from_yaml() {
        let yaml = YamlLoader::load_from_str("name: a.x\nfactory: f").unwrap();
        assert_eq!("a.x", Generator::try_from_yaml(&yaml[0]).unwrap().name);
    }

    #[test]
    fn it_fails_if_generator_name_is_missing() {
        let yaml = YamlLoader::load_from_str("factory: f").unwrap();
        assert!(Generator::try_from_yaml(&yaml[0]).is_err());
    }

    #[test]
    fn it_fails_if_generator_name_is_not_a_string() {
        let yaml = YamlLoader::load_from_str("name: 15\nfactory: f").unwrap();
        assert!(Generator::try_from_yaml(&yaml[0]).is_err());
    }

    #[test]
    fn it_parses_generator_factory_from_yaml() {
        let yaml = YamlLoader::load_from_str("name: a.x\nfactory: f").unwrap();
        assert_eq!("f", Generator::try_from_yaml(&yaml[0]).unwrap().factory);
    }

    #[test]
    fn it_fails_if_factory_name_is_missing() {
        let yaml = YamlLoader::load_from_str("name: a").unwrap();
        assert!(Generator::try_from_yaml(&yaml[0]).is_err());
    }

    #[test]
    fn it_fails_if_factory_name_is_not_a_string() {
        let yaml = YamlLoader::load_from_str("name: a\nfactory: 15").unwrap();
        assert!(Generator::try_from_yaml(&yaml[0]).is_err());
    }

    #[test]
    fn it_fails_if_generator_config_is_not_a_hash() {
        let yaml = YamlLoader::load_from_str("[]").unwrap();
        assert!(Generator::try_from_yaml(&yaml[0]).is_err());
    }
}

#[cfg(test)]
mod find_all_test {
    extern crate cargo_gen_helpers;

    use self::cargo_gen_helpers::test_helpers::create_empty_crate;
    use self::cargo_gen_helpers::{create_file, modify_file};
    use super::*;

    #[test]
    fn it_finds_generators_in_the_current_dir_and_prints_their_names() {
        let crate_dir = create_empty_crate("cargo-gen-test").unwrap();
        create_file(
            crate_dir.path().join("cargo_generators.yaml"),
            "- name: a.x\n  factory: f\n- name: a.y\n  factory: f",
        ).unwrap();
        assert_eq!(
            vec!["a.x", "a.y"],
            find_all(crate_dir.path())
                .into_iter()
                .map(|res| res.map(|generator| generator.name)
                    .unwrap_or_else(|e| format!("{}", e)))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn it_finds_generators_in_the_root_dir_of_a_dependency_and_prints_their_names() {
        let dep_crate_dir = create_empty_crate("cargo-gen-dep").unwrap();
        create_file(
            dep_crate_dir.path().join("cargo_generators.yaml"),
            "- name: a.x\n  factory: f\n- name: a.y\n  factory: f",
        ).unwrap();

        let crate_dir = create_empty_crate("cargo-gen-test").unwrap();
        modify_file(crate_dir.path().join("Cargo.toml"), |contents| {
            let deps_str = "[dependencies]\n";
            let new_deps_str = format!(
                "{}cargo-gen-dep = {{ path = {:?} }}\n",
                deps_str,
                dep_crate_dir.path().as_os_str()
            );
            Ok(Some(contents.replace(deps_str, &new_deps_str)))
        }).unwrap();

        assert_eq!(
            vec!["a.x", "a.y"],
            find_all(crate_dir.path())
                .into_iter()
                .map(|res| res.map(|generator| generator.name)
                    .unwrap_or_else(|e| format!("{}", e)))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn it_skips_and_reports_invalid_generators() {
        let crate_dir = create_empty_crate("cargo-gen-test").unwrap();
        create_file(
            crate_dir.path().join("cargo_generators.yaml"),
            "- name: a.x\n- name: a.y\n  factory: f",
        ).unwrap();
        assert_eq!(
            vec!["ERROR", "a.y"],
            find_all(crate_dir.path())
                .into_iter()
                .map(|res| res.map(|generator| generator.name)
                    .unwrap_or("ERROR".to_string()))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn it_fails_on_invalid_yaml() {
        let crate_dir = create_empty_crate("cargo-gen-test").unwrap();
        create_file(crate_dir.path().join("cargo_generators.yaml"), "[{]}").unwrap();
        assert_eq!(
            vec!["ERROR"],
            find_all(crate_dir.path())
                .into_iter()
                .map(|res| res.map(|generator| generator.name)
                    .unwrap_or("ERROR".to_string()))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn it_fails_if_yaml_is_not_an_array() {
        let crate_dir = create_empty_crate("cargo-gen-test").unwrap();
        create_file(crate_dir.path().join("cargo_generators.yaml"), "{}").unwrap();
        assert_eq!(
            vec!["ERROR"],
            find_all(crate_dir.path())
                .into_iter()
                .map(|res| res.map(|generator| generator.name)
                    .unwrap_or("ERROR".to_string()))
                .collect::<Vec<_>>()
        );
    }
}
