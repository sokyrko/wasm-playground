use wasmer::{imports, Instance, Module, Store, TypedFunction};

/// A text representation of a WebAssembly module (this format is called WAT).
/// This module imports a function called `print` from the `env` namespace, and exports a function
/// `answer` that calls `print` with the answer to life, the universe, and everything.
const GUEST_MODULE_WAT: &str = r#"
(module
  (import "env" "print" (func $print (param i32)))

  (func (export "answer")
     i32.const 42
     call $print
  )
)
"#;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut store = Store::default();

    // Here we can pass both WAT and the compiled Wasm binary.
    let module = Module::new(&store, GUEST_MODULE_WAT)?;

    let import_object = imports! {
        "env" => {
            // The actual function that will be called when `env.print` is called from the guest module.
            "print" => wasmer::Function::new_typed(&mut store, |number: i32| {
                println!("The answer is {}", number);
            }),
        }
    };
    let instance = Instance::new(&mut store, &module, &import_object)?;

    // Take the `answer` function from the WebAssembly instance.
    // As per signature, it takes no arguments and returns nothing.
    let answer_fn: TypedFunction<(), ()> =
        instance.exports.get_typed_function(&mut store, "answer")?;

    answer_fn.call(&mut store)?;

    Ok(())
}
