use result::GResult;
use self::attribute::Attribute;
use std::{collections::HashMap, io::Write};
use traits::WriteBindings;
use types::Types;
use weedle::{
    Definition,
    Definitions,
    attribute::{
        ExtendedAttributeList,
        ExtendedAttribute
    },
    interface::InterfaceMember
};

mod attribute;

#[derive(Debug)]
pub struct Members(Vec<Member>);

impl Members {
    pub fn scrape(from: &Definitions, types: &Types) -> Members {
        let mut members: Vec<Member> = vec![];
        for def in from.definitions.iter() {
            match *def {
                Definition::Interface(ref interface) => {
                    let name = &interface.identifier.name;
                    if types.has(name) {
                        for member in interface.members.body.iter() {
                            if let Some(member) = Member::scrape(member,&name, types) {
                                members.push(member);
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        members.sort();
        members.dedup_by(|a, b| {
            match (a, b) {
                (Member::Attribute(ref attra), Member::Attribute(ref attrb)) => {
                    (attra.interface == attrb.interface || attra.is_global == attrb.is_global)
                        && attra.identifier == attrb.identifier
                }
            }
        });

        Members(members)
    }
}

impl WriteBindings for Members {
    fn write_bindings<T: Write>(&self, buf: &mut T) -> GResult<()> {
        for mem in self.0.iter() {
            mem.write_bindings(buf)?;
        }
        Ok(())
    }
}

#[derive(Debug, Ord, PartialOrd, PartialEq, Eq)]
enum Member {
    Attribute(Attribute)
}

impl Member {
    fn scrape(from: &InterfaceMember, interface: &str, types: &Types) -> Option<Member> {
        match *from {
            InterfaceMember::Attribute(ref attribute) =>
                Some(Member::Attribute(Attribute::scrape(
                    attribute,
                    is_global(attribute.attributes.as_ref()),
                    interface,
                    types)?
                )),
            _ => None
        }
    }
}

impl WriteBindings for Member {
    fn write_bindings<T: Write>(&self, buf: &mut T) -> GResult<()> {
        match *self {
            Member::Attribute(ref attr) => attr.write_bindings(buf)
        }
    }
}

fn is_global(attrs: Option<&ExtendedAttributeList>) -> bool {
    attrs.map(|attrs| {
        attrs.body.list.iter().any(|attr| {
            match *attr {
                ExtendedAttribute::Ident(ref attr) => attr.lhs_identifier.name == "Global",
                ExtendedAttribute::IdentList(ref attr) => attr.identifier.name == "Global",
                _ => false
            }
        })
    }).unwrap_or(false)
}
