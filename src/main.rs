use anyhow::Result;
use clap::Parser;
use glimpse_cli::{Cli, dispatch};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    dispatch(cli)?;

    Ok(())
}
