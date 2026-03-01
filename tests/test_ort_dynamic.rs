use ort::{session::Session, value::Value};
use std::collections::HashMap;

fn test_dynamic_inputs() -> Result<(), Box<dyn std::error::Error>> {
    let session = Session::builder()?.commit_from_memory(&[])?; // dummy
    
    // Can we pass a Vec<Value>?
    let mut inputs: Vec<Value> = Vec::new();
    let shape = vec![1, 2];
    let data = vec![1.0f32, 2.0];
    inputs.push(Value::from_array((shape.clone(), data.clone()))?);
    
    // let outputs = session.run(inputs)?; // Trying this
    
    // What about Vec<(&str, Value)>?
    // let mut dict: Vec<(&str, Value)> = Vec::new();
    // let outputs = session.run(dict)?;

    Ok(())
}
