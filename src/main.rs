mod state;
mod to_do;
use serde_json::value::Value;
use serde_json::{json, Map};
use state::{read_file, write_to_file};
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let status = &args[1];
    let title = &args[2];
    let mut state: Map<String, Value> = read_file("./state.json");
    println!("Before Operation: {:?}", state);
    state.insert(title.to_string(), json!(status));
    println!("After Operation: {:?}", state);
    write_to_file("./state.json", &mut state);
}
