extern crate cargo_gen;

use cargo_gen::cmd_args::CLArgs;
use cargo_gen::gen::Generator;
use std::env::args_os;
use std::env::current_dir;

fn main() {
    let clargs = CLArgs::parse(args_os());
    if clargs.list {
        // FIXME: panics
        for generator in Generator::find_all(current_dir().unwrap()) {
            println!("{:?}", generator);
        }
    }
}
