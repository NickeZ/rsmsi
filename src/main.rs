#[macro_use]
extern crate clap;

mod options;

use options::Options;

fn main() {
    println!("Hello, world!");

    let options = Options::parse_args();

    println!("{:?}", options.macros);
}
