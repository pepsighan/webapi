#[macro_use]
extern crate wasm_bindgen;
extern crate weedle;

use std::{fs, io::Read};
use weedle::{Definitions, Parse, CompleteStr};

pub struct Defs {

}

impl Defs {
    pub fn generate() {
        let paths = fs::read_dir("./defs")
            .unwrap();

        let mut file_contents = String::new();
        for path in paths {
            let path = path.unwrap().path();
            if path.is_file() {
                let mut file = fs::File::open(path).unwrap();
                let file_content =
                file.read_to_string(&mut file_contents).unwrap();
            }
        }

        let parsed = weedle::parse(&file_contents).unwrap();

        println!("{:?}", parsed);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generate() {
        Defs::generate();
    }
}
