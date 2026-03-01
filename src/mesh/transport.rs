use std::net::TcpStream;
use std::io::{Write, Read};
use crate::program_evaluator::Value;
use super::rpc::{RpcValue, MeshRequest, MeshResponse};

pub fn send_mesh_request(target: &str, func_name: &str, args: &[Value], token: Option<String>) -> Result<Value, String> {
    // 1. Serialize arguments
    let mut rpc_args = Vec::new();
    for a in args {
        rpc_args.push(RpcValue::from_value(a)?);
    }
    
    let request = MeshRequest {
        function_name: func_name.to_string(),
        args: rpc_args,
        capability_token: token,
    };
    
    let request_json = serde_json::to_string(&request)
        .map_err(|e| format!("Serialization error: {}", e))?;

    // 2. Connect to remote node
    let mut stream = TcpStream::connect(target)
        .map_err(|e| format!("Failed to connect to mesh node {}: {}", target, e))?;

    // 3. Send Payload (we add a newline as delimiter to make reading easy)
    stream.write_all(request_json.as_bytes())
        .map_err(|e| format!("Network send error: {}", e))?;
    stream.write_all(b"\n")
        .map_err(|e| format!("Network send error: {}", e))?;

    // 4. Read Response (up to newline or EOF)
    let mut response_buf = String::new();
    // In naive PoC we read to end or use a BufReader. 
    // Since we expect exactly one response string, we can read_to_string if the server closes the connection.
    stream.read_to_string(&mut response_buf)
        .map_err(|e| format!("Network receive error: {}", e))?;

    // 5. Parse Response
    let response: MeshResponse = serde_json::from_str(&response_buf)
        .map_err(|e| format!("Failed to parse mesh response from {}: {}", target, e))?;

    // 6. Return Data
    match response.result {
        Ok(rpc_val) => Ok(rpc_val.to_value()),
        Err(e) => Err(format!("Remote error from {}: {}", target, e)),
    }
}
