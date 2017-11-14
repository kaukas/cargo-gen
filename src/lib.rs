#[macro_use]
extern crate askama;
#[macro_use]
extern crate error_chain;

pub mod errors {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
            Askama(::askama::Error);
        }
    }
}

mod gen_trait;
mod helpers;
pub mod gen;
pub mod test_helpers;

pub use gen_trait::CargoGenerator;
pub use helpers::{create_file, modify_file};
