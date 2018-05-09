extern crate weedle;
#[macro_use]
extern crate failure;
extern crate heck;

use std::{fs, io::{Read, Write}};
use types::Types;
use result::GResult;
use traits::WriteBindings;
use members::Members;

mod traits;
mod types;
mod result;
mod members;

pub struct Defs {
    types: Types,
    members: Members
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
        let members = Members::scrape(&parsed, &types);

        Defs {
            types,
            members
        }
    }

    pub fn generate<T: Write>(&self, buf: &mut T) -> GResult<()> {
        write!(buf, "
#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {{

")?;
        self.types.write_bindings(buf)?;
        self.members.write_bindings(buf)?;
        write!(buf, "
}}
        ")?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::OpenOptions;

    #[test]
    fn generate() {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open("../target/test.rs").unwrap();

        Defs::read_defs().generate(&mut file).unwrap();
    }
}
