fn main() {
    let runtime = minutus::Evaluator::build();
    runtime.evaluate("puts 'aaa'").unwrap();
}
