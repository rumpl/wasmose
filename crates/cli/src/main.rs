use std::error::Error;
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

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let runtime = if let Some(runtime) = cli.runtime {
        runtime
    } else {
        String::from("wasmtime")
    };

    let r = RuntimeLoader::runtime(runtime);
    let data = fs::read_to_string(cli.file)?;
    let spec = SpecLoader::deserialize(data)?;
    r.run(spec)?;

    Ok(())
}
