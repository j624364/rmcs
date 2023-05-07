use crate::error::Error;
use crate::value::Value;
use crate::variable::Variable;
use ::std::collections::{HashMap, HashSet};
use ::std::fmt;

#[derive(Debug, Clone)]
pub struct StructureTemplate {
    members: HashSet<String>,
}

impl StructureTemplate {
    pub fn new() -> Self {
        Self {
            members: HashSet::new(),
        }
    }

    pub fn add_member(&mut self, identifier: &String) -> Result<(), Error> {
        if self.members.contains(identifier) {
            return Err(Error::new(
                format!(
                    "member identifier \"{}\" in struct already exists",
                    identifier
                ),
                None,
            ));
        }

        self.members.insert(identifier.clone());

        Ok(())
    }

    pub fn get_members(&self) -> &HashSet<String> {
        &self.members
    }
}

#[derive(Clone, PartialEq)]
pub struct StructureInstance {
    // todo: eventually use something more quick (perhaps with bytecode and then you
    // can use an offset)
    members: HashMap<String, Variable>,
}

impl StructureInstance {
    pub fn from_template(structure_template: &StructureTemplate) -> Self {
        let mut members = HashMap::with_capacity(structure_template.get_members().len());
        for member in structure_template.get_members() {
            members.insert(member.clone(), Variable::new(Value::default()));
        }

        Self { members }
    }

    pub fn has_member(&self, identifier: &String) -> bool {
        self.members.contains_key(identifier)
    }

    pub fn get_member(&self, identifier: &String) -> Option<Value> {
        Some(self.members.get(identifier)?.get())
    }

    pub fn set_member(&self, identifier: &String, value: Value) -> Result<(), Error> {
        match self.members.get(identifier) {
            Some(member) => {
                if member.is_const() {
                    Err(Error::new(
                        format!("member: \"{}\" is const", identifier),
                        None,
                    ))
                }
                else {
                    member.set(value);
                    Ok(())
                }
            }
            None => {
                Err(Error::new(
                    format!("member: \"{}\" does not exist", identifier),
                    None,
                ))
            }
        }
    }
}

impl fmt::Debug for StructureInstance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{")?;

        for (i, (identifier, member)) in self.members.iter().enumerate() {
            write!(f, "\"{}\": {:?}", identifier, member.get())?;

            if i + 1 < self.members.len() {
                write!(f, ", ")?;
            }
        }

        write!(f, "}}")?;

        Ok(())
    }
}

impl fmt::Display for StructureInstance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{")?;

        for (i, (identifier, member)) in self.members.iter().enumerate() {
            write!(f, "\"{}\": {}", identifier, member.get())?;

            if i + 1 < self.members.len() {
                write!(f, ", ")?;
            }
        }

        write!(f, "}}")?;

        Ok(())
    }
}
