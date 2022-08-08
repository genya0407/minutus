use minutus::types::*;

fn main() {
    type Hash = std::collections::HashMap<String, String>;
    let evaluator = minutus::Evaluator::new(Hash::from_mrb);
    let script = std::fs::read_to_string("some_script.rb").unwrap();
    let result = evaluator.evaluate(&script);
    match result {
        Ok(parsed_json) => {
            println!("{:?}", parsed_json);
        }
        Err(msg) => {
            println!("{}", msg);
        }
    }
}
