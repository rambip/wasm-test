fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use wasm_test::*;

    #[wasm_test]
    #[should_panic]
    fn wrong_test(){
        assert!(false);
    }

    #[wasm_test]
    fn right_test(){
        assert!(true);
    }

    #[wasm_test]
    fn print_test(){
        println!("a={}", 42);
    }
}
