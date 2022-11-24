use crossbeam_utils::thread;
use spec::Spec;
use std::{
    io::{self, Read},
    process::Command,
};

fn main() {
    let mut stdin = io::stdin();
    let mut s = String::new();

    stdin.read_to_string(&mut s).unwrap();
    let spec = Spec::deserialize(s).unwrap();
    let modules = spec.modules.values();

    thread::scope(|s| {
        for module in modules {
            s.spawn(move |_| {
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

                let output = Command::new("wasmtime").args(args).output().unwrap();

                println!("{}", String::from_utf8_lossy(&output.stdout));
            });
        }
    })
    .unwrap();
}
