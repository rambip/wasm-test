use wasm_test::*;

#[wasm_test]
#[should_panic]
fn i_am_supposed_to_panic() {
    return
}

#[wasm_test]
fn i_am_not_supposed_to_panic() {
    panic!("I did fail")
}

