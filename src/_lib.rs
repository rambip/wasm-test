pub use serde::Serialize;
pub use serde_json;

#[derive(Serialize)]
pub struct UnitMetadata {
    pub should_panic: bool,
    pub path: String,
}

extern "C" {
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

