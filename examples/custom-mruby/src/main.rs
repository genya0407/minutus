use minutus::types::*;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let evaluator = minutus::Evaluator::build();
    let script = std::fs::read_to_string("some_script.rb").unwrap();
    let mruby_value = evaluator.evaluate(&script)?;
    let parsed_json = <HashMap<String, String>>::try_from_mrb(mruby_value)?;
    println!("{:?}", parsed_json);

    Ok(())
}
