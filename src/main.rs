extern crate getopts;
extern crate mrusty;

use std::fs::File;
use std::io::Read;
use std::env::args;
use getopts::Options;
use mrusty::{Mruby, MrubyImpl};

fn main() {
    let args: Vec<String> = args().collect();

    let mut opts = Options::new();

    opts.optopt("f", "file", "target file path", "FILE");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let mut data = Vec::new();
    let path = matches.opt_str("f");

    let mut file = File::open(path.unwrap()).expect("Unable to open file");
    file.read_to_end(&mut data).expect("Unable to read data");

    let mruby = Mruby::new();
    let result = mruby.run(&String::from_utf8(data).unwrap()).unwrap();

    println!("{}", result.to_i32().unwrap());
}
