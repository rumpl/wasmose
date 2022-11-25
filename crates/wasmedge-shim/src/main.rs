use std::{
    error::Error,
    io::{self, Read},
    process::Command,
};

use spec::SpecLoader;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdin = io::stdin();
    let mut s = String::new();

    stdin.read_to_string(&mut s)?;
    let module = SpecLoader::deserialize_module(s)?;
    let mut args = vec![];
    if let Some(dirs) = &module.dirs {
        for dir in dirs {
            args.push(String::from("--dir"));
            args.push(format!("{}:{}", dir.target, dir.source))
        }
    }

    args.push(module.name.clone());

    let output = Command::new("wasmedge").args(args).output()?;

    print!("{}", String::from_utf8_lossy(&output.stdout));

    Ok(())
}
