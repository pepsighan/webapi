use weedle::{
    Definitions,
    Definition,
    InterfaceDefinition,
    attribute::ExtendedAttribute,
    common::Identifier
};
use std::collections::HashSet;

pub struct Types(HashSet<String>);

impl Types {
    pub fn scrape(from: &Definitions) -> Types {
        let mut idents = HashSet::new();
        for def in from.definitions.iter() {
            match *def {
                Definition::Interface(ref interface) => {
                    if is_interface_object(interface) {
                        idents.insert(interface.identifier.name.clone());
                    }
                }
                _ => {}
            };
        }
        Types(idents)
    }
}

impl Types {
    pub fn has(&self, name: &str) -> bool {
        self.0.iter().any(|ident| ident == name)
    }
}

// For the interface to be ECMAScript objects, they must not have `[NoInterfaceObject]`
// & `[LegacyNamespace]`
fn is_interface_object(interface: &InterfaceDefinition) -> bool {
    interface.attributes.as_ref().map(|attributes| {
        !attributes.body.list.iter().any(|attr| {
            match *attr {
                ExtendedAttribute::NoArgs(ref no_args) => {
                    no_args.identifier.name == "NoInterfaceObject" ||
                        no_args.identifier.name == "LegacyNamespace"
                },
                _ => false
            }
        })
    }).unwrap_or(true)
}
