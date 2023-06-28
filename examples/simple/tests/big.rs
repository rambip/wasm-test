use wasm_test::wasm_test;

static N: usize = 100_000;

#[wasm_test]
fn vec_test() {
    let mut v = Vec::new();
    for i in 0..=N {
        v.push(i);
    }
    assert!(*v.last().unwrap() == N);
}
