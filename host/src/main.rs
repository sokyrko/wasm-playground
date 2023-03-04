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
    let mut store = wasmer::Store::default();
    let module = wasmer::Module::new(&store, GUEST_MODULE_WAT)?;
    let import_object = wasmer::imports! {
        "env" => {
            "print" => wasmer::Function::new_typed(&mut store, |number: i32| {
                println!("The answer is {}", number);
            }),
        }
    };
    let instance = wasmer::Instance::new(&mut store, &module, &import_object)?;

    let answer_fn: wasmer::TypedFunction<(), ()> = instance.exports.get_typed_function(&mut store, "answer")?;
    answer_fn.call(&mut store)?;

    Ok(())
}
