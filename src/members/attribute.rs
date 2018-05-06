use weedle::{types::Type, interface::AttributeInterfaceMember};
use types::Types;
use traits::IsDefined;
use std::cmp::Ordering;

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