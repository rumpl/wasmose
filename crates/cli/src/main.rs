use std::fs;

use std::path::PathBuf;

use clap::Parser;
use runtime::RuntimeLoader;
use spec::SpecLoader;

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

    let r = RuntimeLoader::runtime(runtime);
    let data = fs::read_to_string(cli.file).unwrap();
    let spec = SpecLoader::deserialize(data).unwrap();
    r.run(spec).unwrap();

    // let mut child = Command::new(format!("{}-shim", runtime))
    //     .stdin(Stdio::piped())
    //     .stdout(Stdio::inherit())
    //     .spawn()
    //     .unwrap();

    // let child_stdin = child.stdin.as_mut().unwrap();

    // let data = fs::read(cli.file).unwrap();
    // child_stdin.write_all(&data).unwrap();

    // child.wait().unwrap();
}
