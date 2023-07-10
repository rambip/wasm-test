use wasmer::{
    Store, 
    Module, 
    Instance, 
    imports, 
    Function, 
    Singlepass, 
    FunctionEnv,
    FunctionEnvMut,
    Memory,
    MemoryView,
    MemoryType,
    WasmPtr,
    Value,
    Exports
};

use std::cell::RefCell;
use colored::Colorize;

use serde::Deserialize;
use serde_json;

fn throw(_: u32, _: u32) { }

fn table_grow(_: u32) -> u32 { 0 }

fn table_set_null(_: u32) { }

fn describe(_: u32) { }

struct WasmEnv {
    memory: RefCell<Memory>
}

fn get_string(view: MemoryView, ptr: u32) -> String {
    let ptr : WasmPtr<u8> = WasmPtr::new(ptr);
    ptr.read_utf8_string_with_nul(&view)
       .expect("error reading string")
}

fn wasm_print(env: FunctionEnvMut<WasmEnv>, ptr: u32){
    let memory = env.data().memory.borrow();
    let view = memory.view(&env);
    print!("{}", get_string(view, ptr));
}


fn init_wasm_module(filename: &str, store: &mut Store) -> Instance {

    let module = Module::from_file(&store, filename).expect("unable to load module");

    let fake_memory = Memory::new(store, MemoryType::new(0, None, false)).unwrap();
    let env = FunctionEnv::new(
        store,
        WasmEnv {
            memory: RefCell::new(fake_memory),
        },
    );

    let instance = {
        let import_object = imports! {
            "__wbindgen_placeholder__" => {
                "__wbindgen_throw" => Function::new_typed(store, throw),
                "__wbindgen_describe" => Function::new_typed(store, describe)
            },
            "__wbindgen_externref_xform__" => {
                "__wbindgen_externref_table_grow" => Function::new_typed(store, table_grow),
                "__wbindgen_externref_table_set_null" => Function::new_typed(store, table_set_null),
            },
            "env" => {
                "__wasm_print" => Function::new_typed_with_env(store, &env, wasm_print),
            }
        };

        Instance::new(store, &module, &import_object)
            .expect("unable to create instance")
    };

    // init memory
    let mut mem = env.as_ref(store).memory
        .borrow_mut();

    *mem = instance.exports.get_memory("memory").expect("could not find memory of wasm").clone(); 

    instance
}

static FUNCTION_PREFIX : &'static str = "__wasm_test_unit__";
static METADATA_PREFIX : &'static str = "__wasm_test_meta__";

#[derive(Deserialize)]
struct UnitMetaData{
    should_panic: bool,
    path: String,
}

struct UnitTest<'a> {
    should_panic: bool,
    name: String,
    path: String,
    function: &'a Function
}

fn make_unit_test<'a>(store: &mut Store, exports: &Exports, name: &str, f: &'a Function) -> UnitTest<'a> {

    let metadata_function = exports.get_function(&format!("{METADATA_PREFIX}{name}"))
        .expect("cannot find path function");

    let ptr = match metadata_function.call(store, &[]).unwrap()[0] {
        Value::I32(x) => x as u32,
        _ => panic!("this is not a i32 value")
    };
    let metadata : UnitMetaData = 
        serde_json::from_str(
            &get_string(exports.get_memory("memory").unwrap().view(&store), ptr)
        ).unwrap();

    UnitTest {
        function: f,
        name: name.to_string(),
        should_panic: metadata.should_panic,
        path: metadata.path
    }
}

pub fn run(filename: &str) -> bool {
    let singlepass = Singlepass::new();
    let mut store = Store::new(singlepass);

    let exports = init_wasm_module(filename, &mut store).exports;

    let test_functions = 
        exports.iter().functions()
        .filter(|(name, _)| name.starts_with(FUNCTION_PREFIX))
        .map(|(raw_name, f)| (raw_name.split_at(FUNCTION_PREFIX.len()).1, f));

    let unit_tests : Vec<UnitTest> = 
        test_functions
        .map(|(name, f)| make_unit_test(&mut store, &exports, name, f))
        .collect();


    let n_functions = unit_tests.len();

    if n_functions == 0 {
        println!("there is no test to run");
        return true;
    }

    let maybe_s = if n_functions == 1 {""} else {"s"};
    println!("running {} test{}\n", n_functions, maybe_s);

    let n_success = unit_tests
        .into_iter()
        .filter_map(|unit| run_unit_test(&mut store, unit))
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

fn run_unit_test(store: &mut Store, unit: UnitTest) -> Option<()> {
    match unit.function.call(store, &[]) {
        Ok(_) if ! unit.should_panic => {
            println!("\ttest `{}::{}` --> {}", unit.path, unit.name, "ok".green());
            Some(())
        },
        Err(_) if unit.should_panic => {
            println!("\ttest `{}::{}` --> {}", unit.path, unit.name, "ok".green());
            Some(())
        },
        Ok(_) => {
            println!("\ttest `{}::{}` --> {}", unit.path, unit.name, "FAILED".red());
            println!("this test should have panicked.\n");
            None
        }
        Err(_) => {
            println!("\ttest `{}::{}` --> {}", unit.path, unit.name, "FAILED".red());
            None
        }
    }
}
