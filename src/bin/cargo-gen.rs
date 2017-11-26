extern crate cargo_gen;

use cargo_gen::CLArgs;
use std::env::args_os;

fn main() {
    let clargs = CLArgs::parse(args_os());
}
