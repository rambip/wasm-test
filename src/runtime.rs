//! Internal-only runtime module used for the `wasm-test` crate.
//!
//! No API contained in this module will respect semver, these should all be
//! considered private APIs.


// see https://docs.rs/wasm-bindgen-test/latest/src/wasm_bindgen_test/rt/mod.rs.html#1-704
// for a similar project

// Architecture:
// the `wasm-test-macro` crate will generate code
// that will use the functions defined here, using the `wasm_test::__runtime` path
// At the end, these functions will become webassembly imports: 
// that way, the runner (`wasm-test-runner`) will allow the generated code to 
// print error message, provide metadata inforamtion about the tests, and so on.

pub use serde::Serialize;
pub use serde_json;

/// Represents a test to be run.
#[derive(Serialize)]
pub struct UnitMetadata {
    /// wether the test should panic, specified with the `should_panic` attribute
    pub should_panic: bool,

    // path of the module where the test is defined
    pub path: String,
}

extern "C" {
    /// external function used to print strings.
    /// It uses utf8 null-terminated strings.
    fn __wasm_print(start: u32) -> ();
}

pub fn print(mut s: String){
    s.push('\0');
    unsafe {
        __wasm_print(s.as_ptr() as u32)
    }
}

pub fn println(mut s: String){
    s.push('\n');
    s.push('\0');
    unsafe {
        __wasm_print(s.as_ptr() as u32)
    }
}

