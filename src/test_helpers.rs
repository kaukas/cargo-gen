// TODO: move this into a standalone crate. We don't want to compile cargo and tempdir for no
// reason.
extern crate cargo;
extern crate tempdir;

use self::cargo::ops;
use self::cargo::util::Config as CargoConfig;
use self::tempdir::TempDir;
use std::fs::File;
use std::io::{Error, Read};
use std::path::PathBuf;

pub fn read_file_to_string(path: PathBuf) -> Result<String, Error> {
    let mut content = String::new();
    File::open(path)?.read_to_string(&mut content)?;
    Ok(content)
}

pub fn create_empty_crate() -> TempDir {
    let tempdir = TempDir::new("cargo-gen-test").unwrap();
    {
        let config = CargoConfig::default();
        let options = ops::NewOptions::new(None,
                                           false,
                                           true,
                                           tempdir.path().to_str().unwrap(),
                                           Some("cargo-gen-test"));
        ops::init(options, &config.unwrap()).unwrap();
    }
    tempdir
}
