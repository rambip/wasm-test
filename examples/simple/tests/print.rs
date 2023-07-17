use wasm_test::*;

#[wasm_test]
fn print() {
    print!("foo");
    println!("foobar");
    print!("x={} y={}", 1, 2);
    panic!("I want to see the result !")
}

