use anyhow::{bail, Result};
use colored::{Color, Colorize};
use crossbeam_utils::thread;
use rand::Rng;
use spec::{Module, Spec};

use std::{
    io::{BufRead, BufReader, Write},
    process::{Command, Stdio},
};
pub struct Runtime {
    runtime: String,
}

fn colored(id: String) -> colored::ColoredString {
    let colors = vec![
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
    ];

    let mut rng = rand::thread_rng();

    let random_string_index: usize = rng.gen_range(0..colors.len());
    let color = colors[random_string_index];

    id.color(color)
}

impl Runtime {
    pub fn new(runtime: String) -> Self {
        Runtime { runtime }
    }

    pub fn run(&self, spec: Spec) -> Result<()> {
        let mut pad = 0;
        spec.modules.iter().for_each(|(id, _)| {
            if pad < id.len() {
                pad = id.len();
            }
        });

        thread::scope(|s| {
            for (id, module) in spec.modules {
                s.spawn(move |_| {
                    self.run_module(id, pad + 2, &module).unwrap();
                });
            }
        })
        .unwrap();

        Ok(())
    }

    pub fn run_module(&self, id: String, pad: usize, module: &Module) -> Result<()> {
        let id = colored(format!("{}:", id));

        let mut child = Command::new(format!("{}-shim", self.runtime))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;

        if let Some(mut child_stdin) = child.stdin.take() {
            let data = serde_yaml::to_string(module)?;
            child_stdin.write_all(data.as_bytes())?;
            drop(child_stdin);

            if let Some(out) = child.stdout.take() {
                let reader = BufReader::new(out);

                reader
                    .lines()
                    .filter_map(|line| line.ok())
                    .for_each(|line| println!("{id:pad$} {line}"));
            }
        } else {
            bail!("unable to get child stdin");
        }

        Ok(())
    }
}

pub struct RuntimeLoader {}

impl RuntimeLoader {
    pub fn runtime(runtime: String) -> Runtime {
        Runtime { runtime }
    }
}
