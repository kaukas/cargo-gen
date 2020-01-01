use std::ffi::OsString;

#[derive(Debug)]
pub struct Generator {
    pub name: String,
    pub crate_path: OsString,
    pub factory: String,
}
