use crate::prelude::*;

pub fn add_structure_lib(run_state: &mut RunState) -> Result<(), Error> {
    run_state.expose_macro("def-struct", def_struct)?;
    run_state.expose_macro("get-member", get_member)?;
    run_state.expose_macro("set-member", set_member)?;

    Ok(())
}

fn def_struct(run_state: &mut RunState, node: &Node) -> Result<Value, Error> {
    let children = node.get_children();
    if children.is_empty() {
        return Err(Error::new(
            "structure must have identifier".to_string(),
            node.get_token().clone(),
        ));
    }

    let structure_identifier = get_identifier(children.first().unwrap())?;

    let mut structure_template = StructureTemplate::new();

    for child in children.iter().skip(1) {
        let member_identifier = get_identifier(child)?;
        structure_template.add_member(member_identifier)?;
    }

    run_state
        .get_local_scope_mut()
        .set_structure_template(structure_identifier, structure_template);

    Ok(Value::default())
}

fn get_member(run_state: &mut RunState, node: &Node) -> Result<Value, Error> {
    let children = node.get_children();
    if children.len() != 2 {
        return Err(Error::new(
            "requires two arguments".to_string(),
            node.get_token().clone(),
        ));
    }

    let structure_identifier = get_identifier(&children[0])?;
    let member_identifier = get_identifier(&children[1])?;

    match run_state.find_local(structure_identifier) {
        Some(local) => match local {
            Value::Structure(structure) => match structure.get_member(member_identifier) {
                Some(value) => Ok(value.clone()),
                None => Err(Error::new(
                    format!("structure does not have member: \"{}\"", member_identifier),
                    node.get_token().clone(),
                )),
            },
            _ => Err(Error::new(
                format!(
                    "structure identifier: \"{}\" is not a structure",
                    structure_identifier
                ),
                node.get_token().clone(),
            )),
        },
        None => Err(Error::new(
            format!("could not find local: \"{}\"", structure_identifier),
            node.get_token().clone(),
        )),
    }
}

fn set_member(run_state: &mut RunState, node: &Node) -> Result<Value, Error> {
    let children = node.get_children();
    if children.len() != 3 {
        return Err(Error::new(
            "requires three arguments".to_string(),
            node.get_token().clone(),
        ));
    }

    let structure_identifier = get_identifier(&children[0])?;
    let member_identifier = get_identifier(&children[1])?;

    let value = children[2].evaluate(run_state)?;

    match run_state.find_local(structure_identifier) {
        Some(local) => match local {
            Value::Structure(structure) => {
                structure.set_member(member_identifier, value)?;

                Ok(Value::default())
            }
            _ => Err(Error::new(
                format!(
                    "structure identifier: \"{}\" is not a structure",
                    structure_identifier
                ),
                node.get_token().clone(),
            )),
        },
        None => Err(Error::new(
            format!("could not find local: \"{}\"", structure_identifier),
            node.get_token().clone(),
        )),
    }
}
