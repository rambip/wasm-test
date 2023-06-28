use wasm_test::wasm_test;

fn main() {
    println!("Hello, world!");
}

#[wasm_test]
fn wrong_test(){
    assert!(false);
}

#[wasm_test]
fn right_test(){
    assert!(true);
}
