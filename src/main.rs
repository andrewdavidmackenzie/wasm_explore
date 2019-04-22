#[macro_use]
extern crate serde_json;

mod add;
mod implementation;
mod wrapper;

use implementation::Implementation;
use add::Add;

fn main() {
    let adder = &Add{} as &Implementation;

    let inputs = vec!(vec!(json!(1)), vec!(json!(2)));
    let input_data = wrapper::serialize(inputs);

    let (result_data, run_again) = wrapper::call(adder, input_data);

    let result = wrapper::deserialize(result_data);

    println!("Result = {}, run_again = {}", result.unwrap(), run_again);
}
