use std::collections::HashMap;
use self::attribute::Attribute;
use weedle::{Definitions, Definition, interface::InterfaceMember};
use types::Types;

mod attribute;

#[derive(Debug)]
pub struct Members(HashMap<String, Vec<Member>>);

impl Members {
    pub fn scrape(from: &Definitions, types: &Types) -> Members {
        let mut members = HashMap::new();
        for def in from.definitions.iter() {
            match *def {
                Definition::Interface(ref interface) => {
                    let name = &interface.identifier.name;
                    if types.has(name) {
                        let mut this_members = vec![];

                        for member in interface.members.body.iter() {
                            if let Some(member) = Member::scrape(member, types) {
                                this_members.push(member);
                            }
                        }

                        if members.contains_key(name) {
                            let mut existing_members = members.remove(name)
                                .unwrap();

                            this_members.append(&mut existing_members);

                            members.insert(name.clone(), this_members);
                        } else {
                            members.insert(name.clone(), this_members);
                        }
                    }
                },
                _ => {}
            }
        }
        Members(members)
    }
}

#[derive(Debug)]
enum Member {
    Attribute(Attribute)
}

impl Member {
    fn scrape(from: &InterfaceMember, types: &Types) -> Option<Member> {
        match *from {
            InterfaceMember::Attribute(ref attribute) =>
                Some(Member::Attribute(Attribute::scrape(attribute, types)?)),
            _ => None
        }
    }
}
