use common::{Request, Response};
use wasmer::{AsStoreRef, FunctionEnv, FunctionEnvMut, Memory, MemoryView, Module, Store, WasmPtr};

// At this point, the `guest.wasm` file must be compiled.
static WASM: &'static [u8] =
    include_bytes!("../../target/wasm32-unknown-unknown/release/guest.wasm");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut store = Store::default();
    let module = Module::new(&store, WASM)?;
    let env = FunctionEnv::new(&mut store, FunctionCtx::New);

    let import_object = wasmer::imports! {
        "env" => {
            "respond" => wasmer::Function::new_typed_with_env(&mut store, &env, |env: FunctionEnvMut<FunctionCtx>, a: WasmPtr<u8>, b: u32| {
                let ctx = env.data();
                let response = ctx.read_response(&env, a, b);

                println!("Received from guest: {:?}", &response);
            }),
        }
    };
    let instance = wasmer::Instance::new(&mut store, &module, &import_object)?;
    let memory = instance.exports.get_memory("memory")?;
    {
        let ctx = env.as_mut(&mut store);
        *ctx = FunctionCtx::Initialized(memory.clone());
    }

    let request = Request {
        message: "Hello, it's a message from host!".to_string(),
        number: 42,
    };

    println!("Sending to guest: {:?}", &request);

    let (offset, len) = env.as_ref(&store).write_request(&store, request);

    let answer_fn: wasmer::TypedFunction<(i32, i32), ()> = instance
        .exports
        .get_typed_function(&mut store, "hello_string_from_rust")?;

    answer_fn.call(&mut store, offset as i32, len as i32)?;

    Ok(())
}

enum FunctionCtx {
    New,
    Initialized(Memory),
}

impl FunctionCtx {
    fn memory_view(&self, store: &impl AsStoreRef) -> MemoryView {
        match self {
            FunctionCtx::Initialized(memory) => memory.view(&store),
            FunctionCtx::New => panic!("BUG: Memory is not initialized yet."),
        }
    }

    fn write_request(&self, store: &impl AsStoreRef, request: Request) -> (u64, usize) {
        let offset = 0;
        let memory_view = self.memory_view(store);
        let json = request.to_json();
        let bytes = json.as_bytes();

        memory_view
            .write(offset, bytes)
            .expect("failed to write string");

        (offset, bytes.len())
    }

    fn read_response(&self, store: &impl AsStoreRef, ptr: WasmPtr<u8>, length: u32) -> Response {
        let memory_view = self.memory_view(store);

        let json = ptr
            .slice(&memory_view, length)
            .unwrap()
            .read_to_vec()
            .unwrap();

        Response::from_json(&json)
    }
}
