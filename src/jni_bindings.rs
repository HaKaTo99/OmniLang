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
    // 1. Ekstrak string dengan aman di luar closure panic
    let source_script: String = match env.get_string(&input) {
        Ok(s) => s.into(),
        Err(_) => return create_jni_string(&mut env, "{\"status\":\"error\",\"message\":\"JNI_STR_CONV_FAILED\"}"),
    };

    info!("Executing OmniLang Script (len: {})", source_script.len());

    // 2. Gunakan panic::catch_unwind untuk membungkus logika evaluasi
    let execution_result = panic::catch_unwind(|| {
        // Pipeline Eksekusi
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

    // 3. Tangani hasil atau deteksi Panic
    let output_json = match execution_result {
        Ok(json) => json,
        Err(_) => {
            error!("CRITICAL: OmniLang Native Panic Detected!");
            "{\"status\":\"error\",\"message\":\"NATIVE_PANIC_RECOVERED\",\"type\":\"internal\"}".to_string()
        }
    };

    // 4. Kembalikan hasil ke JVM
    create_jni_string(&mut env, &output_json)
}

/// Helper untuk mendapatkan versi Native OmniLang.
#[no_mangle]
pub extern "system" fn Java_com_omnilang_core_OmniLang_getNativeVersion<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> jstring {
    let version = env!("CARGO_PKG_VERSION");
    create_jni_string(&mut env, version)
}
