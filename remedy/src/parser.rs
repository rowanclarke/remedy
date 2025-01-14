use std::{collections::HashMap, io::Read};

use pest::error::Error;
use pest_meta::{parse_and_optimize, parser::Rule};
use pest_vm::Vm;

use crate::workspace::Workspace;

pub fn get_root_vm<'a, W: Workspace>(workspace: &W) -> Result<Vm, Vec<Error<Rule>>> {
    let mut grammar = String::new();
    workspace.read_config(&["root.pest".into()], &mut grammar);
    Ok(Vm::new(parse_and_optimize(&grammar)?.1))
}

pub fn get_vm<'a, W: Workspace>(workspace: &W) -> Result<Vm, Vec<Error<Rule>>> {
    let mut grammar = String::new();
    let mut start = 0;
    let mut map = HashMap::new();
    workspace
        .get_descendants(&workspace.from_config(&[]), &[&["root.pest".into()]])
        .into_iter()
        .for_each(|name| {
            let end = start + workspace.read_config(&name.absolute_path, &mut grammar);
            map.insert(name.absolute_path.clone(), (start, end));
            start = end;
        });
    // println!("{:?}", map);
    // println!("{}", grammar);
    Ok(Vm::new(parse_and_optimize(&grammar)?.1))
}
