use spec::{Module, SpecLoader};
use std::{
    error::Error,
    io::{self, Read},
    process::ExitCode,
};
use wasmedge_sdk::{
    config::{
        CommonConfigOptions, ConfigBuilder, HostRegistrationConfigOptions, RuntimeConfigOptions,
    },
    params, Vm,
};

fn main() -> Result<ExitCode, Box<dyn Error>> {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s)?;
    let module = SpecLoader::deserialize_module(s)?;

    std::process::exit(run_module(&module)? as i32);
}

fn run_module(module: &Module) -> Result<u32, Box<dyn Error>> {
    let common_options = CommonConfigOptions::default()
        .bulk_memory_operations(true)
        .multi_value(true)
        .mutable_globals(true)
        .non_trap_conversions(true)
        .reference_types(true)
        .sign_extension_operators(true)
        .simd(true);

    let runtime_options = RuntimeConfigOptions::default().max_memory_pages(1024);

    let host_options = HostRegistrationConfigOptions::default().wasi(true);

    let config = ConfigBuilder::new(common_options)
        .with_runtime_config(runtime_options)
        .with_host_registration_config(host_options)
        .build()?;

    let mut env: Option<Vec<&str>> = None;
    let envs: Vec<String>;
    if let Some(env_vars) = &module.environment {
        envs = env_vars
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();
        env = Some(envs.iter().map(AsRef::as_ref).collect());
    }

    let mut preopens: Option<Vec<&str>> = None;
    let directories: Vec<String>;
    if let Some(dirs) = &module.dirs {
        directories = dirs
            .iter()
            .map(|dir| format!("{}:{}", dir.target, dir.source))
            .collect();
        preopens = Some(directories.iter().map(AsRef::as_ref).collect());
    }

    let mut vm = Vm::new(Some(config))?;
    let mut wasi_module = vm.wasi_module()?;
    wasi_module.initialize(None, env, preopens);
    vm.run_func_from_file(&module.module, "_start", params!())?;

    Ok(wasi_module.exit_code())
}
