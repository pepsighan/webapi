use weedle::{
    Definitions,
    Definition,
    InterfaceDefinition,
    attribute::ExtendedAttribute,
    common::Identifier
};
use traits::Scrape;

pub struct Types(Vec<String>);

impl Scrape for Types {
    type From = Definitions;

    fn scrape(from: &Definitions) -> Types {
        let mut idents = vec![];
        for def in from.definitions.iter() {
            match *def {
                Definition::Interface(ref interface) => {
                    if is_interface_object(interface) {
                        idents.push(interface.identifier.name.clone());
                    }
                }
                _ => {}
            };
        }
        Types(idents)
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
