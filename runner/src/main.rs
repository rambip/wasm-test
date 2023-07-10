use std::env::args;

mod runner;
use runner::run;

use std::process::ExitCode;

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
