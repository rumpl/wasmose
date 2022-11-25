use std::io::Write;
use std::process::{Command, Stdio};

use anyhow::Result;
use crossbeam_utils::thread;
use spec::{Module, Spec};
pub struct Runtime {
    runtime: String,
}

impl Runtime {
    pub fn new(runtime: String) -> Self {
        Runtime { runtime }
    }

    pub fn run(&self, spec: Spec) -> Result<()> {
        let modules = spec.modules.values();

        thread::scope(|s| {
            for module in modules {
                s.spawn(move |_| {
                    self.run_module(module);
                });
            }
        })
        .unwrap();

        Ok(())
    }

    pub fn run_module(&self, module: &Module) {
        let mut child = Command::new(format!("{}-shim", self.runtime))
            .stdin(Stdio::piped())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap();

        let child_stdin = child.stdin.as_mut().unwrap();

        let data = serde_yaml::to_string(module).unwrap();
        child_stdin.write_all(&data.as_bytes()).unwrap();

        child.wait().unwrap();
    }
}

pub struct RuntimeLoader {}

impl RuntimeLoader {
    pub fn runtime(runtime: String) -> Runtime {
        Runtime { runtime }
    }
}
