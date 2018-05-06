use weedle::{types::Type, interface::AttributeInterfaceMember};
use types::Types;
use traits::{IsDefined, WriteBindings};
use std::{cmp::Ordering, io::Write};
use result::GResult;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Attribute {
    pub is_global: bool,
    pub interface: String,
    pub readonly: bool,
    pub identifier: String,
    pub type_: Type
}

impl Attribute {
    pub fn scrape(from: &AttributeInterfaceMember, is_global: bool, interface: &str, types: &Types) -> Option<Attribute> {
        if from.type_.type_.is_defined(types) {
            let readonly = from.readonly.is_some();
            let identifier = from.identifier.name.clone();
            let type_ = from.type_.type_.clone();

            Some(Attribute {
                is_global,
                interface: interface.to_string(),
                readonly,
                identifier,
                type_
            })
        } else {
            None
        }
    }
}

impl WriteBindings for Attribute {
    fn write_bindings<T: Write>(&self, buf: &mut T) -> GResult<()> {
        if self.is_global {
            write!(buf, "static {}: ", self.identifier)?;
            self.type_.write_bindings(buf)?;
            writeln!(buf, ";")?;
        } else {
            write!(buf, r#"
#[wasm_bindgen(method, getter)
fn {name}(this: &{interface}) -> "#, name = self.identifier, interface = self.interface)?;
            self.type_.write_bindings(buf)?;
            writeln!(buf, ";")?;

            if !self.readonly {
                write!(buf, r#"
#[wasm_bindgen(method, setter)]
fn set_{name}(this: &{interface}) -> "#, name = self.identifier, interface = self.interface)?;
                self.type_.write_bindings(buf)?;
                write!(buf, ";")?;
            }
        }
        Ok(())
    }
}
