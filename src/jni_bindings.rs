use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;

use crate::lexer::Lexer;
use crate::program_evaluator::ProgramEvaluator;

/// Fungsi pembantu keamanan untuk meredam pelemparan *panic* JNIEnv saat OS 
/// mengalami kehabisan memori (_Out of Memory_) atau cacat UTF-8 sekunder.
fn create_jni_string(env: &mut JNIEnv, msg: &str) -> jstring {
    match env.new_string(msg) {
        Ok(j_str) => j_str.into_raw(),
        Err(_) => std::ptr::null_mut(), // Tolerate failure gracefully instead of crashing JVM JVM.
    }
}

/// Mengeksekusi string skrip OmniLang yang dipanggil dari Java Native Interface (JNI).
/// 
/// Penamaan fungsi sangat esensial sesuai spesifikasi JNI. Class pada sisi Android/Java
/// harus terletak dalam package `com.omnilang.core` dengan nama class `OmniLang`.
#[no_mangle]
pub extern "system" fn Java_com_omnilang_core_OmniLang_eval<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    input: JString<'local>,
) -> jstring {
    // 1. Ekstrak string jString dari VM Java ke String Native Rust
    let source_script: String = match env.get_string(&input) {
        Ok(s) => s.into(),
        Err(_) => return create_jni_string(&mut env, "❌ [JNI ERROR] Gagal mengonversi Java String!"),
    };

    // 2. Jalankan Logika Eksekusi Native OmniLang
    let mut lexer = Lexer::new(&source_script);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => return create_jni_string(&mut env, &format!("❌ [LEXER ERROR] {}", e)),
    };
    let mut parser = crate::parser::Parser::new(tokens);
    
    // Parsing
    let program = match parser.parse_program() {
        Ok(prog) => prog,
        Err(err) => return create_jni_string(&mut env, &format!("❌ [SYNTAX ERROR] {}", err)),
    };

    // Evaluasi Inti
    let mut evaluator = ProgramEvaluator::new();
    
    match evaluator.evaluate_program(&program) {
        Ok(result) => {
            // Evaluasi sukses: Teruskan hasil akhir eksekusi (Value) dari Rust back ke ranah JVM.
            create_jni_string(&mut env, &format!("{:?}", result))
        },
        Err(e) => {
            create_jni_string(&mut env, &format!("❌ [RUNTIME ERROR] {:?}", e))
        }
    }
}
