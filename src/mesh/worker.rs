use std::net::{TcpListener, TcpStream};
use std::io::{Write, BufRead, BufReader};
use crate::program_evaluator::ProgramEvaluator;
use super::rpc::{MeshRequest, MeshResponse, RpcValue};
use std::sync::{Arc, Mutex};

pub fn start_worker(port: u16, evaluator: Arc<Mutex<ProgramEvaluator>>, required_token: Option<String>) {
    let address = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&address).expect("Failed to bind to port");
    println!("OmniLang Mesh Worker listening on {}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let eval_clone = Arc::clone(&evaluator);
                let token_clone = required_token.clone();
                std::thread::spawn(move || {
                    handle_connection(stream, eval_clone, token_clone);
                });
            }
            Err(e) => println!("Connection failed: {}", e),
        }
    }
}

fn handle_connection(mut stream: TcpStream, evaluator: Arc<Mutex<ProgramEvaluator>>, required_token: Option<String>) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut request_buf = String::new();
    
    if let Ok(bytes_read) = reader.read_line(&mut request_buf) {
        if bytes_read == 0 { return; }
        
        let response = match serde_json::from_str::<MeshRequest>(&request_buf) {
            Ok(req) => {
                println!("[MESH] Received execution request for: {}", req.function_name);
                
                // Security Check
                if let Some(ref required) = required_token {
                    let provided = req.capability_token.as_ref().map(|s| s.as_str()).unwrap_or("");
                    if required != provided {
                        println!("[MESH] SECURITY HALT: Unauthorized token provided");
                        let err_resp = MeshResponse {
                            result: Err("[Security Halt] Unauthorized capability token".to_string())
                        };
                        let response_json = serde_json::to_string(&err_resp).unwrap_or_else(|_| "{\"result\":{\"Err\":\"Failed to serialize response\"}}".to_string());
                        let _ = stream.write_all(response_json.as_bytes());
                        let _ = stream.write_all(b"\n");
                        return;
                    }
                }
                
                let mut core_args = Vec::new();
                for a in req.args {
                    core_args.push(a.to_value());
                }
                
                let mut eval = evaluator.lock().unwrap();
                let res = eval.call_function_by_name(&req.function_name, core_args);
                
                MeshResponse {
                    result: res.and_then(|v| RpcValue::from_value(&v))
                }
            },
            Err(e) => {
                MeshResponse {
                    result: Err(format!("Invalid MeshRequest JSON: {}", e)),
                }
            }
        };

        let response_json = serde_json::to_string(&response).unwrap_or_else(|_| "{\"result\":{\"Err\":\"Failed to serialize response\"}}".to_string());
        let _ = stream.write_all(response_json.as_bytes());
        let _ = stream.write_all(b"\n");
    }
}
