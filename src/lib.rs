#[cfg(not(target_arch = "wasm32"))]
compile_error!{"`wasm-test` can only be used for wasm32, try changing your `target` settings"}

pub use wasm_test_macro::wasm_test;

#[path = "./runtime.rs"]
pub mod __runtime;
