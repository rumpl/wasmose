use std::{
    io::{self, Read},
    process::Command,
};

use spec::Spec;

fn main() {
    let mut stdin = io::stdin(); // We get `Stdin` here.
    let mut s = String::new();

    stdin.read_to_string(&mut s).unwrap();
    let spec = Spec::deserialize(s).unwrap();

    for (_, module) in spec.modules {
        let mut args = vec![];
        if let Some(dirs) = module.dirs {
            args.push(String::from("--dir"));
            args.push(String::from("."));

            for dir in dirs {
                args.push(String::from("--mapdir"));
                args.push(format!("{}::{}", dir.target, dir.source))
            }
        }

        args.push(module.name);

        let output = Command::new("wasmtime").args(args).output().unwrap();

        print!("{}", String::from_utf8_lossy(&output.stdout));
    }
}
