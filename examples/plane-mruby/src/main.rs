fn main() {
    let runtime = minutus::build_simple_evaluator();
    runtime.evaluate("puts 'aaa'").unwrap();
}
