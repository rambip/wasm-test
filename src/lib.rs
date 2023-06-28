#[cfg(not(target_arch = "wasm32"))]
compile_error!("this can only compile for wasm32 target !");

pub use wasm_test_macro::wasm_test;
