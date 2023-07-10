# Goal

This crate offer a way to do unit-tests in yew.
It is a feature that many people want, but it is quite hard to do it right because of the way `wasm-bindgen` is build.

# Usage

First, add wasm-test as a dev-dependency in your `Cargo.toml`
```toml
[dev-dependencies]
wasm-test = {git="https://github.com/rambip/wasm-test"}
```

In your rust project, you can replace the `#[test]` invocation by `#[wasm_test]` like so:
```rust
#[cfg(test)]
mod tests {
    use wasm_test::*;

    #[wasm_test]
    fn test_should_pass(){
        assert!(true);
    }

    #[wasm_test]
    #[should_panic]
    fn test_should_fail(){
        assert!(false);
    }
}
```

Install the special runner with cargo:
```bash
cargo install wasm-test-runner
```
(if it doesn't work, you can add the `--target <your target>` to cargo install, see [here](https://github.com/rust-lang/cargo/pull/5614) for why)

Then, add this line to your `.cargo/config.toml`
```toml
[build]
target = "wasm32-unknown-unknown"

[target.wasm32-unknown-unknown]
runner = "wasm-test-run"
```


Now, you should be able to run your unit-tests as usual !
```bash
cargo test
```


# Notes

- The goal of this crate is to test the logic all the **wasm** part of your code.
That means you can write unit-tests, bun you cannot test javascript code.
If you want integration tests with javascript instead, please use [wasm-bindgen-test](https://rustwasm.github.io/docs/wasm-bindgen/wasm-bindgen-test/index.html)


# Todo
- [ ] Show backtrace, at least get the line of the error
