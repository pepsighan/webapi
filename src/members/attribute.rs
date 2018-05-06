use weedle::{types::Type, interface::AttributeInterfaceMember};
use types::Types;
use traits::{IsDefined, WriteBindings};
use std::io::Write;
use result::GResult;
use heck::SnakeCase;

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
        let snake_name = self.identifier.to_snake_case();
        if self.is_global {
            if self.identifier != snake_name {
                writeln!(buf, "#[wasm_bindgen(js_name = {})]", self.identifier)?;
            }

            write!(buf, "static {}: ", snake_name)?;
            self.type_.write_bindings(buf)?;
            writeln!(buf, ";")?;
        } else {
            if self.identifier == snake_name {
                writeln!(buf, "#[wasm_bindgen(method, getter)")?;
            } else {
                writeln!(buf, "#[wasm_bindgen(method, getter = {}]", self.identifier)?;
            }

            write!(buf, "fn {name}(this: &{interface}) -> ", name = snake_name, interface = self.interface)?;
            self.type_.write_bindings(buf)?;
            writeln!(buf, ";\n")?;

            if !self.readonly {
                if self.identifier == snake_name {
                    writeln!(buf, "#[wasm_bindgen(method, setter)")?;
                } else {
                    writeln!(buf, "#[wasm_bindgen(method, setter = {}]", self.identifier)?;
                }

                write!(buf, "fn set_{name}(this: &{interface}) -> ", name = snake_name, interface = self.interface)?;
                self.type_.write_bindings(buf)?;
                writeln!(buf, ";\n")?;
            }
        }
        Ok(())
    }
}
