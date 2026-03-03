use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::program_evaluator::ProgramEvaluator;

/// Exposes OmniLang evaluation to C-compatible hosts (like HarmonyOS NAPI C++).
/// Takes a null-terminated C string, evaluates it, and returns a newly allocated
/// null-terminated C string containing the result or error message.
/// 
/// # Safety
/// The caller MUST ensure the input `code` is a valid null-terminated C string.
/// The caller MUST call `omnilang_free_string` on the returned pointer to avoid memory leaks.
#[no_mangle]
pub unsafe extern "C" fn omnilang_eval(code: *const c_char) -> *mut c_char {
    if code.is_null() {
        let err = CString::new("Error: Received null pointer").unwrap();
        return err.into_raw();
    }

    let c_str = CStr::from_ptr(code);
    let str_slice = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => {
            let err = CString::new("Error: Invalid UTF-8 sequence").unwrap();
            return err.into_raw();
        }
    };

    let mut lexer = Lexer::new(str_slice);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            let err_str = format!("Lexer Error: {}", e);
            return CString::new(err_str).unwrap_or_else(|_| CString::new("Lexer Error").unwrap()).into_raw();
        }
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.parse_program() {
        Ok(p) => p,
        Err(e) => {
            let err_str = format!("Parser Error: {}", e);
            return CString::new(err_str).unwrap_or_else(|_| CString::new("Parser Error").unwrap()).into_raw();
        }
    };

    let mut evaluator = ProgramEvaluator::new();
    match evaluator.evaluate_program(&program) {
        Ok(result) => {
             let out = format!("{:?}", result);
             CString::new(out).unwrap_or_else(|_| CString::new("Evaluation Success, serialization failed").unwrap()).into_raw()
        }
        Err(e) => {
            let err_str = format!("Runtime Error: {:?}", e);
            CString::new(err_str).unwrap_or_else(|_| CString::new("Runtime Error").unwrap()).into_raw()
        }
    }
}

/// Frees a string previously allocated by `omnilang_eval`.
/// 
/// # Safety
/// The pointer `s` must have been returned by `omnilang_eval`.
/// Calling this twice on the same pointer results in double-free undefined behavior.
#[no_mangle]
pub unsafe extern "C" fn omnilang_free_string(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    // Retake ownership and immediately drop it
    let _ = CString::from_raw(s);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::{CStr, CString};

    #[test]
    fn test_c_abi_eval() {
        // Evaluate module syntax to verify Rust execution engine via C-ABI FFI
        // Note: `evaluate_program` evaluates top-level items and returns `Unit`.
        let code = CString::new("module test_mod { fn main() {} }").unwrap();
        let result_ptr = unsafe { omnilang_eval(code.as_ptr()) };
        let result = unsafe { CStr::from_ptr(result_ptr) };
        assert_eq!(result.to_str().unwrap(), "Unit");
        unsafe { omnilang_free_string(result_ptr) };
    }
}
