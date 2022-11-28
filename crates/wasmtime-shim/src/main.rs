use anyhow::Result;
use spec::{Dir, Module, SpecLoader};
use std::{
    error::Error,
    io::{self, Read},
    process::ExitCode,
};
use wasmtime::*;
use wasmtime_wasi::sync::{ambient_authority, Dir as WasiDir};
use wasmtime_wasi::WasiCtxBuilder;

#[derive(Default)]
struct Host {
    wasi: Option<wasmtime_wasi::WasiCtx>,
}

fn main() -> Result<ExitCode, Box<dyn Error>> {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s)?;
    let module = SpecLoader::deserialize_module(s)?;

    std::process::exit(run_module(&module)?);
}

fn run_module(module: &Module) -> Result<i32, Box<dyn Error>> {
    let engine = Engine::default();
    let mut store = Store::new(&engine, Host::default());

    let wasm_module = wasmtime::Module::from_file(&engine, &module.module)?;

    let mut linker = Linker::new(&engine);

    populate_with_wasi(
        &mut store,
        &mut linker,
        &module.dirs,
        &[module.module.clone()],
    )?;

    linker.module(&mut store, "", &wasm_module)?;
    let func = linker.get_default(&mut store, "")?;

    let values = Vec::new();
    let ty = func.ty(&store);
    let mut results = vec![Val::null(); ty.results().len()];
    func.call(&mut store, &values, &mut results)?;

    Ok(0)
}

fn populate_with_wasi(
    store: &mut Store<Host>,
    linker: &mut Linker<Host>,
    preopen_dirs: &Option<Vec<Dir>>,
    argv: &[String],
) -> Result<()> {
    wasmtime_wasi::add_to_linker(linker, |host| host.wasi.as_mut().unwrap())?;

    let mut builder = WasiCtxBuilder::new();
    // TODO: env vars
    builder = builder.inherit_stdio().args(argv)?;

    if let Some(dirs) = &preopen_dirs {
        for d in dirs {
            builder = builder.preopened_dir(
                WasiDir::open_ambient_dir(&d.source, ambient_authority())?,
                &d.target,
            )?;
        }
    }

    store.data_mut().wasi = Some(builder.build());

    Ok(())
}
