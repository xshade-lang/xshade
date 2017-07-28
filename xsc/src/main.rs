extern crate xshade;
extern crate getopts;

use getopts::Options;
use std::env;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();

    opts.optflag("", "glsl", "output glsl");

    let source = args[1].to_string();

    let matches = match opts.parse(&args[2..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let glsl = matches.opt_present("glsl");

    println!("{:?}", glsl);
}
