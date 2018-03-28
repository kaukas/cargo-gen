#[macro_use]
extern crate askama;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;

// TODO: move this into a standalone crate. We don't want to compile cargo and tempdir when they
// are not used.
extern crate cargo;
extern crate tempdir;

pub mod errors {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
            Askama(::askama::Error);
            Cargo(::cargo::CargoError);
        }
    }
}

mod gen_trait;
mod helpers;
pub mod gen;
pub mod test_helpers;

pub use gen_trait::CargoGenerator;
pub use helpers::{create_file, modify_file};
