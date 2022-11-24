use std::{
    io::{self, Read},
    process::Command,
};

use crossbeam_utils::thread;
use spec::Spec;

fn main() {
    let mut stdin = io::stdin(); // We get `Stdin` here.
    let mut s = String::new();

    stdin.read_to_string(&mut s).unwrap();
    let spec = Spec::deserialize(s).unwrap();
    let modules = spec.modules.values();

    thread::scope(|s| {
        for module in modules {
            s.spawn(move |_| {
                let mut args = vec![];
                if let Some(dirs) = &module.dirs {
                    for dir in dirs {
                        args.push(String::from("--dir"));
                        args.push(format!("{}:{}", dir.target, dir.source))
                    }
                }

                args.push(module.name.clone());

                let output = Command::new("wasmedge").args(args).output().unwrap();

                print!("{}", String::from_utf8_lossy(&output.stdout));
            });
        }
    })
    .unwrap();
}
