use wasmer::{
    Store, 
    Module, 
    Instance, 
    imports, 
    Function, 
    Singlepass, 
};

use wasmer_types::TrapCode;

use colored::Colorize;

fn throw(_: i32, _: i32) { }

fn table_grow(_: i32) -> i32 { 0 }

fn table_set_null(_: i32) { }

fn describe(_: i32) { }

static EXPORTED_FUNCTION_PREFIX : &'static str = "__wasm_test_";

pub fn run(filename: &str) -> bool {

    let singlepass = Singlepass::new();

    let mut store = Store::new(singlepass);
    let module = Module::from_file(&store, filename).expect("unable to load module");

    let import_object = imports! {
        "__wbindgen_placeholder__" => {
            "__wbindgen_throw" => Function::new_typed(&mut store, throw),
            "__wbindgen_describe" => Function::new_typed(&mut store, describe)
        },
        "__wbindgen_externref_xform__" => {
            "__wbindgen_externref_table_grow" => Function::new_typed(&mut store, table_grow),
            "__wbindgen_externref_table_set_null" => Function::new_typed(&mut store, table_set_null),
        }
    };

    let instance = Instance::new(&mut store, &module, &import_object).expect("unable to create instance");

    let mut test_functions: Vec<(String, &Function)> = Vec::new();

    for (name, f) in instance.exports.iter().functions() {
        if name.starts_with(EXPORTED_FUNCTION_PREFIX){
            let (_, f_name) = name.split_at(EXPORTED_FUNCTION_PREFIX.len());
            test_functions.push((f_name.to_string(), &f));
        }
    }

    let n_functions = test_functions.len();

    if n_functions == 0 {
        println!("there is no test to run");
        return true;
    }

    let maybe_s = if n_functions == 1 {""} else {"s"};
    println!("running {} test{}\n", n_functions, maybe_s);

    let n_success = test_functions
        .into_iter()
        .map(|(name, f)| test_function(&mut store, &name, f))
        .filter(|x| *x)
        .count();

    println!();
    println!("result: {} tests passed; {} tests failed",
             n_success, n_functions - n_success);

    if n_functions == n_success {
        println!("----------------\n");
        true
    }
    else {
        false
    }
}

fn test_function(store: &mut Store, name: &str, f: &Function) -> bool {
    match f.call(store, &[]) {
        Ok(_) => {
            println!("\ttest `{}` --> {}", name, "ok".green());
            true
        },
        Err(_x) => {
            println!("\ttest `{}` --> {}", name, "FAILED".red());
            false
        }
    }
}

fn show_backtrace(x: wasmer::RuntimeError) {
    match x.clone().to_trap() {
        Some(c) if c == TrapCode::UnreachableCodeReached => {
            println!("test failed");
            for stackframe in x.trace() {
                println!("stack frame: {:?}", stackframe);
            }
        },
        Some(x) => println!("error: {}", x),
        None => println!("the runtime error is {}", x)
    }
}
