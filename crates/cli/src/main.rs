use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "runtime")]
    runtime: Option<String>,

    file: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    let runtime = if let Some(runtime) = cli.runtime {
        runtime
    } else {
        String::from("wasmtime")
    };

    let mut child = Command::new(format!("{}-shim", runtime))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let child_stdin = child.stdin.as_mut().unwrap();

    let data = fs::read(cli.file).unwrap();
    child_stdin.write_all(&data).unwrap();

    let output = child.wait_with_output().unwrap();

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
