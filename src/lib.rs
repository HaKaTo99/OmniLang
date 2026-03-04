pub mod ast;
pub mod checker;
pub mod lexer;
pub mod parser;
pub mod types;
pub mod evaluator;
pub mod runtime;
pub mod ir;
pub mod ir_interpreter;
pub mod emitter;
pub mod codegen;
pub mod stdlib;
pub mod error;
pub mod omniroutine;
pub mod observability;
pub mod action_abi;
pub mod program_evaluator;
#[cfg(not(target_arch = "wasm32"))]
pub mod lsp_server;
pub mod security;
#[cfg(not(target_arch = "wasm32"))]
pub mod onnx_oracle;
#[cfg(not(target_arch = "wasm32"))]
pub mod mesh;
#[cfg(not(target_arch = "wasm32"))]
pub mod opm;

#[cfg(target_arch = "wasm32")]
pub mod wasm_bindings;

#[cfg(any(target_os = "android", feature = "jni_bridge"))]
pub mod jni_bindings;

#[cfg(feature = "c_bridge")]
pub mod c_bindings;

pub use error::OmniError;
pub use omniroutine::{OmniRoutine, RoutineResult, RoutineTask};
pub use observability::{global_logger, init_global_logger, set_global_trace, Logger, TraceId};
pub use action_abi::ActionResult;
