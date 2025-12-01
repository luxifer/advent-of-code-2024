use advent_of_code::cli;
use anyhow::Result;

fn main() -> Result<()> {
    let app = cli::Cli::new();
    let lines = app.content()?;

    Ok(())
}
