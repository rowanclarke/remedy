mod object;

use std::collections::HashMap;

use anyhow::Result;
use object::Object;
use pest::{error::Error, Parser};
use pest_meta::{parse_and_optimize, parser::Rule};
use pest_vm::Vm;
use remedy::pest::Pairs;
use trql::query;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn execute(input: &str, grammar: &str, query: &str) -> String {
    let vm = Vm::new(parse_and_optimize(&grammar).unwrap().1);
    let pairs = Pairs(vm.parse("root", input).unwrap());
    serde_yml::to_string(&query::execute::<_, Object>(query, pairs)).unwrap()
}

#[test]
fn test() {
    println!("{}", execute("a", "x = { \"a\" }", "a = x"));
}
