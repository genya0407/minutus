use minutus::types::*;
use std::collections::HashMap;

#[test]
fn check_mruby_version() {
    let evaluator = minutus::Evaluator::build();
    let mruby_version = String::try_from_mrb(evaluator.evaluate("MRUBY_VERSION").unwrap()).unwrap();
    assert_eq!(mruby_version, "3.2.0");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let evaluator = minutus::Evaluator::build();
    let script = std::fs::read_to_string("some_script.rb").unwrap();
    let mruby_value = evaluator.evaluate(&script)?;
    let parsed_json = <HashMap<String, String>>::try_from_mrb(mruby_value)?;
    println!("{:?}", parsed_json);

    Ok(())
}
