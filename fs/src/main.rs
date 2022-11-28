use std::env;
use walkdir::WalkDir;

// Based on https://natclark.com/tutorials/rust-list-all-files/
fn main() {
    let arguments: Vec<String> = env::args().collect();

    for (k, v) in env::vars() {
        println!("got env var: {} = {}", k, v);
    }

    let root_dir = if arguments.len() == 1 {
        "/"
    } else {
        arguments[1].as_str()
    };
    println!("Starting from root_dir {:#?}", root_dir);

    for file in WalkDir::new(root_dir)
        .into_iter()
        .filter_map(|file| file.ok())
    {
        println!("{}", file.path().display());
    }
}
