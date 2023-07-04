fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use wasm_test::wasm_test;

    #[wasm_test]
    fn wrong_test(){
        assert!(false);
    }

    #[wasm_test]
    fn right_test(){
        assert!(true);
    }
}
