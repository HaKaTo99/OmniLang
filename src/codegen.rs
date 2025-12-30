use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

/// Build a native runner embedding the policy IR JSON and return the binary bytes.
pub fn generate_native(policy_ir_json: &str) -> Result<Vec<u8>, String> {
    let root = workspace_root()?;
    let root_str = escape_path_for_toml(&root);
    let workdir = root.join("target/codegen/native");
    fs::create_dir_all(&workdir).map_err(|e| format!("create workdir failed: {}", e))?;

    let cargo_toml = format!(
        r#"[package]
name = "omnilang_native_runner"
version = "0.0.1"
edition = "2021"

[dependencies]
omnilang_core = {{ package = "omnilang", path = "{}" }}
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"
"#,
        root_str
    );
    fs::write(workdir.join("Cargo.toml"), cargo_toml)
        .map_err(|e| format!("write native Cargo.toml failed: {}", e))?;

    let src_dir = workdir.join("src");
    fs::create_dir_all(&src_dir).map_err(|e| format!("create src dir failed: {}", e))?;
    let mut main_rs = fs::File::create(src_dir.join("main.rs"))
        .map_err(|e| format!("create main.rs failed: {}", e))?;
    writeln!(
        main_rs,
        r##"use std::fs;
use std::io::Read;

static IR_JSON: &str = r#"{ir}"#;

fn read_context() -> Result<serde_json::Value, String> {{
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Some(path) = args.get(0) {{
        let content = fs::read_to_string(path)
            .map_err(|e| format!("cannot read context file {{}}: {{}}", path, e))?;
        serde_json::from_str(&content).map_err(|e| format!("invalid context JSON: {{}}", e))
    }} else {{
        let mut buf = String::new();
        std::io::stdin()
            .read_to_string(&mut buf)
            .map_err(|e| format!("cannot read stdin: {{}}", e))?;
        if buf.trim().is_empty() {{
            return Ok(serde_json::json!({{}}));
        }}
        serde_json::from_str(&buf).map_err(|e| format!("invalid context JSON: {{}}", e))
    }}
}}

fn main() {{
    let ir: omnilang_core::ir::PolicyIR =
        serde_json::from_str(IR_JSON).expect("embedded IR must be valid");
    let ctx = read_context().unwrap_or_else(|e| {{
        eprintln!("[ERROR] {{}}", e);
        std::process::exit(1);
    }});

    let decision = omnilang_core::ir_interpreter::execute_ir(&ir, ctx);
    match serde_json::to_string(&decision) {{
        Ok(s) => println!("{{}}", s),
        Err(e) => {{
            eprintln!("[ERROR] serialize decision failed: {{}}", e);
            std::process::exit(1);
        }}
    }}
}}
"##,
        ir = policy_ir_json.replace('"', "\\\"")
    )
    .map_err(|e| format!("write main.rs failed: {}", e))?;

    let target_dir = workdir.join("target");
    let status = Command::new("cargo")
        .arg("build")
        .arg("--target-dir")
        .arg(&target_dir)
        .current_dir(&workdir)
        .status()
        .map_err(|e| format!("failed to spawn cargo build (native): {}", e))?;
    if !status.success() {
        return Err(format!("cargo build native runner failed with status {:?}", status));
    }

    let bin_path = target_dir.join("debug").join(if cfg!(windows) {
        "omnilang_native_runner.exe"
    } else {
        "omnilang_native_runner"
    });
    fs::read(&bin_path).map_err(|e| format!("read built native runner failed: {}", e))
}

/// Generate wasm32-wasi binary exporting evaluate that runs IR interpreter.
pub fn generate_wasm(policy_ir_json: &str) -> Result<Vec<u8>, String> {
    let root = workspace_root()?;
    let root_str = escape_path_for_toml(&root);
    let workdir = root.join("target/codegen/wasm");
    fs::create_dir_all(&workdir).map_err(|e| format!("create workdir failed: {}", e))?;

    let cargo_toml = format!(
        r#"[package]
name = "omnilang_wasm_runner"
version = "0.0.1"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
omnilang_core = {{ package = "omnilang", path = "{}" }}
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"
"#,
        root_str
    );
    fs::write(workdir.join("Cargo.toml"), cargo_toml)
        .map_err(|e| format!("write wasm Cargo.toml failed: {}", e))?;

    let src_dir = workdir.join("src");
    fs::create_dir_all(&src_dir).map_err(|e| format!("create src dir failed: {}", e))?;
    let mut lib_rs = fs::File::create(src_dir.join("lib.rs"))
        .map_err(|e| format!("create lib.rs failed: {}", e))?;
    writeln!(
        lib_rs,
        r##"use std::sync::Mutex;

static IR_JSON: &str = r#"{ir}"#;
static OUTPUT: Mutex<Vec<u8>> = Mutex::new(Vec::new());

#[no_mangle]
pub extern "C" fn evaluate(ptr: *const u8, len: usize) -> i32 {{
    let ctx_slice = unsafe {{ std::slice::from_raw_parts(ptr, len) }};
    let ctx_str = match std::str::from_utf8(ctx_slice) {{
        Ok(s) => s,
        Err(_) => return -1,
    }};

    let ir: omnilang_core::ir::PolicyIR = match serde_json::from_str(IR_JSON) {{
        Ok(v) => v,
        Err(_) => return -2,
    }};
    let ctx: serde_json::Value = match serde_json::from_str(ctx_str) {{
        Ok(v) => v,
        Err(_) => return -3,
    }};

    let decision = omnilang_core::ir_interpreter::execute_ir(&ir, ctx);
    let out = match serde_json::to_vec(&decision) {{
        Ok(v) => v,
        Err(_) => return -4,
    }};

    let mut buf = OUTPUT.lock().unwrap();
    buf.clear();
    buf.extend_from_slice(&out);

    0
}}

#[no_mangle]
pub extern "C" fn get_output_ptr() -> *const u8 {{
    let buf = OUTPUT.lock().unwrap();
    buf.as_ptr()
}}

#[no_mangle]
pub extern "C" fn get_output_len() -> usize {{
    let buf = OUTPUT.lock().unwrap();
    buf.len()
}}
"##,
        ir = policy_ir_json.replace('"', "\\\"")
    )
    .map_err(|e| format!("write lib.rs failed: {}", e))?;

    let target_dir = workdir.join("target");
    let status = Command::new("cargo")
        .arg("build")
        .arg("--target")
        .arg("wasm32-wasi")
        .arg("--target-dir")
        .arg(&target_dir)
        .current_dir(&workdir)
        .status()
        .map_err(|e| format!("failed to spawn cargo build (wasm): {}", e))?;
    if !status.success() {
        return Err(format!("cargo build wasm runner failed with status {:?}", status));
    }

    let bin_path = target_dir
        .join("wasm32-wasi")
        .join("debug")
        .join("omnilang_wasm_runner.wasm");
    fs::read(&bin_path).map_err(|e| format!("read built wasm runner failed: {}", e))
}

fn workspace_root() -> Result<PathBuf, String> {
    std::env::current_dir()
        .map_err(|e| format!("cannot get current dir: {}", e))?
        .canonicalize()
        .map_err(|e| format!("cannot canonicalize cwd: {}", e))
}

fn escape_path_for_toml(path: &PathBuf) -> String {
    path.to_string_lossy().replace('\\', "\\\\")
}
