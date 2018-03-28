extern crate yaml_rust;

use std::fs::File;
use std::ffi::OsStr;
use std::io::Read;
use std::path::Path;
use self::yaml_rust::{Yaml, YamlLoader};

#[derive(Debug)]
pub struct Generator {
    name: String,
}

impl Generator {
    pub fn find_all<P>(root_crate_path: P) -> Vec<Generator>
    where
        P: AsRef<Path> + AsRef<OsStr>,
    {
        let mut yaml_str = String::new();
        // FIXME: panics
        File::open(Path::new(&root_crate_path).join("cargo_generators.yaml"))
            .unwrap()
            .read_to_string(&mut yaml_str)
            .unwrap();
        let yaml = YamlLoader::load_from_str(&yaml_str);
        let generators = yaml.unwrap();
        generators[0]
            .as_vec()
            .unwrap()
            .iter()
            .map(|g| {
                g.as_hash()
                    .unwrap()
                    .get(&Yaml::from_str("name"))
                    .unwrap()
                    .as_str()
                    .unwrap()
            })
            .map(|name| Generator {
                name: name.to_owned(),
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    extern crate cargo_gen_helpers;

    use self::cargo_gen_helpers::test_helpers::create_empty_crate;
    use self::cargo_gen_helpers::create_file;
    use super::*;

    #[test]
    fn it_finds_generators_in_the_current_dir_and_prints_their_names() {
        let crate_dir = create_empty_crate("cargo-gen-test").unwrap();
        create_file(
            crate_dir.path().join("cargo_generators.yaml"),
            "- name: a.x\n- name: a.y",
        ).unwrap();
        assert_eq!(
            vec!["a.x", "a.y"],
            Generator::find_all(crate_dir.path())
                .iter()
                .map(|g| g.name.to_owned())
                .collect::<Vec<_>>()
        );
    }
}
