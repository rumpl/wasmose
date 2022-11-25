use spec::SpecLoader;
use std::{
    error::Error,
    io::{self, Read},
    process::{Command, Stdio},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdin = io::stdin();
    let mut s = String::new();

    stdin.read_to_string(&mut s)?;
    let module = SpecLoader::deserialize_module(s)?;

    let mut args = vec![];
    if let Some(dirs) = &module.dirs {
        args.push(String::from("--dir"));
        args.push(String::from("."));

        for dir in dirs {
            args.push(String::from("--mapdir"));
            args.push(format!("{}::{}", dir.target, dir.source))
        }
    }

    if let Some(env_vars) = &module.environment {
        for (k, v) in env_vars {
            args.push(String::from("--env"));
            args.push(format!("{}={}", k, v));
        }
    }

    args.push(module.name);

    let mut child = Command::new("wasmtime")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .args(args)
        .spawn()?;

    child.wait()?;

    Ok(())
}
