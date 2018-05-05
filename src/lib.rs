extern crate weedle;
#[macro_use]
extern crate failure;

use std::{fs, io::Read};
use types::Types;
use traits::Scrape;

mod traits;
mod types;
mod result;
mod members;

pub struct Defs {
    types: Types
}

impl Defs {
    fn read_idl() -> String {
        let paths = fs::read_dir("./defs")
            .unwrap();

        let mut file_contents = String::new();
        for path in paths {
            let path = path.unwrap().path();
            if path.is_file() {
                let mut file = fs::File::open(path).unwrap();
                file.read_to_string(&mut file_contents).unwrap();
            }
        }

        file_contents
    }

    pub fn read_defs() -> Defs {
        let content = Self::read_idl();
        let parsed = weedle::parse(&content).unwrap();
        let types = Types::scrape(&parsed);

        Defs {
            types
        }
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
