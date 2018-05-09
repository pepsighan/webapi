extern crate webapi_gen;

use webapi_gen::Defs;
use std::{fs::OpenOptions, process};

fn main() {
    let mut lib = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open("../webapi/src/lib.rs")
        .expect("Could not open file");

    match Defs::read_defs()
        .generate(&mut lib) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };
}