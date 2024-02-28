#![no_std]
#![no_main]

use anyhow::Result;
use wasmi::{Config, Engine, Extern, Linker, Module, StackLimits, Store};

psp::module!("sample_module", 1, 1);

type HostState = u32;

fn real_main() -> Result<()> {
    let mut config = Config::default();
    config.set_stack_limits(StackLimits::new(256, 512, 128).unwrap());

    let engine = Engine::new(&config);

    let wasm_binary: &'static [u8] = include_bytes!("add.wasm");

    let module = Module::new(&engine, &wasm_binary[..]).unwrap();

    let mut store = Store::new(&engine, 42);

    let linker = <Linker<HostState>>::new(&engine);

    let instance = linker
        .instantiate(&mut store, &module)
        .unwrap()
        .start(&mut store)
        .unwrap();

    let add = instance
        .get_export(&store, "add")
        .and_then(Extern::into_func)
        .expect("could not find function \"add\"")
        .typed::<(i32, i32), i32>(&mut store)
        .unwrap();

    let num = add.call(&mut store, (1, 2)).unwrap();
    psp::dprintln!("Got {}", num);

    Ok(())
}

fn psp_main() {
    psp::enable_home_button();
    match real_main() {
        Ok(_) => psp::dprint!("Success"),
        Err(e) => psp::dprint!("Error: {:?}", e),
    }
}
