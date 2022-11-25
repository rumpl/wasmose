use spec::SpecLoader;
use std::{
    io::{self, Read},
    process::{Command, Stdio},
};

fn main() {
    let mut stdin = io::stdin();
    let mut s = String::new();

    stdin.read_to_string(&mut s).unwrap();
    let module = SpecLoader::deserialize_module(s).unwrap();

    let mut args = vec![];
    if let Some(dirs) = &module.dirs {
        args.push(String::from("--dir"));
        args.push(String::from("."));

        for dir in dirs {
            args.push(String::from("--mapdir"));
            args.push(format!("{}::{}", dir.target, dir.source))
        }
    }

    args.push(module.name.clone());

    let mut child = Command::new("wasmtime")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .args(args)
        .spawn()
        .unwrap();

    child.wait().unwrap();
}
