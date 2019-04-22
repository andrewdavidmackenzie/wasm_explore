#[macro_use]
extern crate serde_json;

mod add;
mod implementation;
mod wrapper;

use implementation::Implementation;
use crate::wrapper::Wrapper;
use add::Add;

fn main() {
    let inputs = vec!(vec!(json!(1)), vec!(json!(2)));
    let implementation = &Wrapper{implementation: &Add{}} as &Implementation;

    let (result, run_again) = implementation.run(inputs);

    println!("Result = {}, run_again = {}", result.unwrap(), run_again);
}
