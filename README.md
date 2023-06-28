# Goal

This crate offer a way to do unit-tests in yew.
It is a feature that many people want, but it is quite hard to do it right because of the way `wasm-bindgen` is build.

# Usage

First, add wasm-test as a dev-dependency in your `Cargo.toml`
```toml
[dev-dependencies]
wasm-test = {git="https://github.com/rambip/wasm-test"}
```

In your rust project, you can replace the `#[test]` invocation by
```
[wasm_test]
fn test_should_pass(){
    assert!(true);
}

[wasm_test]
fn test_should_fail(){
    assert!(false);
}
```

Install the special runner with cargo:
```bash
cargo install --git https://github.com/rambip/wasm-test
```

Then, add this line to your `.cargo/config.toml`
```toml
[test]
target = "wasm32-unknown-unknown"

[target.wasm32-unknown-unknown]
runner = "~/.cargo/bin/wasm-test"
```


Now, you should be able to run your unit-tests as usual !
```bash
cargo test
```

# Note

For now, the #[should_panic] attribute is not supported, sorry

# TODO

- support should_panic
- create tests
- split into 2 crates:
    - wasm-test with only the proc-macro
    - wasm-test-runner with the wasmer project

Or set a compilation flag to compile
- only the proc-macro when wasm32
- only the wasmer when native

# SOURCES
https://docs.rs/wasm-bindgen-test-macro/0.3.37/src/wasm_bindgen_test_macro/lib.rs.html#14-17

https://doc.rust-lang.org/test/fn.test_main_static.html

https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/browsers.html

https://docs.rs/wasmer/latest/wasmer/
