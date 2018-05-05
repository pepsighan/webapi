use weedle::{types::Type, interface::AttributeInterfaceMember};
use types::Types;
use traits::IsDefined;

#[derive(Debug)]
pub struct Attribute {
    readonly: bool,
    identifier: String,
    type_: Type
}

impl Attribute {
    pub fn scrape(from: &AttributeInterfaceMember, types: &Types) -> Option<Attribute> {
        if from.type_.type_.is_defined(types) {
            let readonly = from.readonly.is_some();
            let identifier = from.identifier.name.clone();
            let type_ = from.type_.type_.clone();

            Some(Attribute {
                readonly,
                identifier,
                type_
            })
        } else {
            None
        }
    }
}
