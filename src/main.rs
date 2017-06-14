extern crate clap;

mod options;
mod makro;

use options::Options;

fn main() {
    println!("Hello, world!");

    let options = Options::parse_args();

    println!("{:?}", options.macros);
}
