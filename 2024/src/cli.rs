use anyhow::{Context, Result};
use clap::Parser;
use std::time::Instant;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    input: std::path::PathBuf,
}

impl Cli {
    pub fn new() -> Self {
        Self::parse()
    }

    pub fn content(&self) -> Result<String> {
        let content = std::fs::read_to_string(&self.input)
            .with_context(|| format!("can't read file {}", self.input.display()))?;
        Ok(content)
    }
}

pub fn stage<F: Fn() -> i64>(stage: i32, f: F) {
    let now = Instant::now();
    let answer = f();
    println!(
        "stage {}: {}\nin {}ms",
        stage,
        answer,
        now.elapsed().as_millis()
    );
}
