use errors::*;
use std::ffi::OsString;

pub trait CargoGenerator {
    fn gen<I, T>(&self, args: I) -> Result<()>
        where I: IntoIterator<Item = T>,
              T: Into<OsString> + Clone;
}
