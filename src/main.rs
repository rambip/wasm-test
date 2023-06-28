#[cfg(target_arch = "wasm32")]
compile_error!("this cannot compile for wasm32 target !");

use std::env::args;
use wasm_test_runner::run;

use std::process::ExitCode;

#[cfg(not(target_family= "wasm"))]
fn main() -> ExitCode {
    let args = args().collect::<Vec<String>>();
    let filename = args.get(1).expect("please provide a path");
    println!();
    let success = run(filename);
    if success {
        ExitCode::SUCCESS
    }
    else {
        ExitCode::FAILURE
    }
}
