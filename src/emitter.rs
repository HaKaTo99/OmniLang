use crate::ast::Policy;
use crate::codegen::{generate_native, generate_wasm};
use crate::ir::build_policy_ir;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompileTarget {
    Ir,
    Native,
    Wasm,
}

pub fn emit(policy: &Policy, target: CompileTarget, out_path: &str) -> Result<(), String> {
    let ir = build_policy_ir(policy);
    let ir_value = serde_json::to_value(&ir).map_err(|e| format!("serialize IR failed: {}", e))?;

    let output_bytes: Vec<u8> = match target {
        CompileTarget::Ir => serde_json::to_string_pretty(&ir_value)
            .map_err(|e| format!("serialize IR failed: {}", e))?
            .into_bytes(),
        CompileTarget::Native => {
            let json_str = serde_json::to_string_pretty(&ir_value)
                .map_err(|e| format!("serialize IR failed: {}", e))?;
            generate_native(&json_str)?
        }
        CompileTarget::Wasm => {
            let json_str = serde_json::to_string(&ir_value)
                .map_err(|e| format!("serialize IR failed: {}", e))?;
            generate_wasm(&json_str)?
        }
    };

    let path = Path::new(out_path);
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("cannot create dir {}: {}", parent.display(), e))?;
        }
    }

    fs::write(path, output_bytes)
        .map_err(|e| format!("write output {} failed: {}", path.display(), e))?;

    Ok(())
}
