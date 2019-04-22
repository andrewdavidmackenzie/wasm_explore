#[macro_use]
extern crate serde_json;

mod add;
mod implementation;

use implementation::Implementation;
use add::Add;

fn main() {
    let adder = &Add{} as &Implementation;
    let inputs = vec!(vec!(json!(1)), vec!(json!(2)));
    let (result, run_again) = adder.run(inputs);

    println!("Result = {}, run_again = {}", result.unwrap(), run_again);
}
