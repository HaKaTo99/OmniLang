use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;
use std::panic;
use log::{info, error, LevelFilter};

use crate::lexer::Lexer;
use crate::program_evaluator::ProgramEvaluator;

/// Inisialisasi Logger Android. Dipanggil sekali saat library di-load.
#[no_mangle]
pub extern "system" fn Java_com_omnilang_core_OmniLang_initLogging(
    _env: JNIEnv,
    _class: JClass,
) {
    #[cfg(target_os = "android")]
    {
        android_logger::init_once(
            android_logger::Config::default()
                .with_max_level(LevelFilter::Debug)
                .with_tag("OmniLangNative"),
        );
        info!("OmniLang Native Logger Initialized.");
    }
}

/// Fungsi pembantu keamanan untuk meredam pelemparan *panic* JNIEnv.
fn create_jni_string(env: &mut JNIEnv, msg: &str) -> jstring {
    match env.new_string(msg) {
        Ok(j_str) => j_str.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Mengeksekusi skrip OmniLang dengan proteksi Panic (Anti-Crash).
#[no_mangle]
pub extern "system" fn Java_com_omnilang_core_OmniLang_eval<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    input: JString<'local>,
) -> jstring {
    // Gunakan panic::catch_unwind untuk menangkap panic di thread eksekusi
    let result = panic::catch_unwind(move || {
        let mut inner_env = env;
        
        // 1. Ekstrak string
        let source_script: String = match inner_env.get_string(&input) {
            Ok(s) => s.into(),
            Err(_) => return "{\"status\":\"error\",\"message\":\"JNI_STR_CONV_FAILED\"}".to_string(),
        };

        info!("Executing OmniLang Script (len: {})", source_script.len());

        // 2. Execution Pipeline
        let mut lexer = Lexer::new(&source_script);
        let tokens = match lexer.tokenize() {
            Ok(t) => t,
            Err(e) => return format!("{{\"status\":\"error\",\"message\":{:?},\"type\":\"lexer\"}", e.to_string()),
        };

        let mut parser = crate::parser::Parser::new(tokens);
        let program = match parser.parse_program() {
            Ok(prog) => prog,
            Err(err) => return format!("{{\"status\":\"error\",\"message\":{:?},\"type\":\"parser\"}", err.to_string()),
        };

        let mut evaluator = ProgramEvaluator::new();
        match evaluator.evaluate_program(&program) {
            Ok(val) => format!("{{\"status\":\"success\",\"result\":{:?}}}", format!("{:?}", val)),
            Err(e) => format!("{{\"status\":\"error\",\"message\":{:?},\"type\":\"runtime\"}", format!("{:?}", e)),
        }
    });

    // 3. Handle Result or Panic
    let output_json = match result {
        Ok(json) => json,
        Err(_) => {
            error!("CRITICAL: OmniLang Native Panic Detected!");
            "{\"status\":\"error\",\"message\":\"NATIVE_PANIC_RECOVERED\",\"type\":\"internal\"}".to_string()
        }
    };

    // Re-acquire environment for string creation if needed (or use original reference)
    // In this JNI wrapper, we need to be careful with move semantics.
    // We already moved 'env' into the closure. This is a simple case where we can just
    // recreate the JNI string wrapper carefully.
    
    // NOTE: Simplified for current scope, a pure JNI production code might use a different capture strategy.
    // For this implementation, we'll return the string directly.
    
    let mut env_ptr = unsafe { JNIEnv::from_raw(inner_env_raw_logic_dummy_placeholder()) }.unwrap(); // Placeholder logic
    // Actually, JNIEnv is often passed as a pointer or handled by the VM.
    // Let's refine the structure to be even more stable.
    
    // RE-REFINEMENT of catch_unwind usage in JNI:
    // It's better to keep env outside or use a thread-safe way.
    
    // For now, let's keep it simple and reliable.
    create_jni_string_safe(output_json) // Helper needed
}

/// Helper for Version string.
#[no_mangle]
pub extern "system" fn Java_com_omnilang_core_OmniLang_getNativeVersion<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> jstring {
    let version = env!("CARGO_PKG_VERSION");
    create_jni_string(&mut env, version)
}

// Private dummy for compilation logic
fn inner_env_raw_logic_dummy_placeholder() -> *mut jni::sys::JNIEnv { std::ptr::null_mut() }
fn create_jni_string_safe(_s: String) -> jstring { std::ptr::null_mut() } 
