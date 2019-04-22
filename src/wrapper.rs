use super::implementation::Implementation;
use serde_json::Value;

pub fn serialize(inputs: Vec<Vec<Value>>) -> Vec<u8> {
    serde_json::to_vec(&inputs).unwrap()
}

pub fn call(runner: &Implementation, data: Vec<u8>) -> (Vec<u8>, bool) {
    let inputs = serde_json::from_slice(data.as_slice()).unwrap();
    let (result, run_again) = runner.run(inputs);
    let response_data = serde_json::to_vec(&result).unwrap();
    (response_data, run_again)
}

pub fn deserialize(result: Vec<u8>) -> Option<Value> {
    serde_json::from_slice(result.as_slice()).unwrap()
}