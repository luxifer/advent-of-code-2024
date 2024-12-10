use anyhow::{Context, Result};
use clap::Parser;

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
