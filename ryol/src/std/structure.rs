use crate::prelude::*;

pub fn add_structure_lib(run_state: &mut RunState) -> Result<(), Error> {
    run_state.expose_macro("def-struct", def_struct)?;

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
