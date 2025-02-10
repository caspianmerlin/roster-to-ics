use anyhow::{bail, Context};
use calamine::{Data, Reader, Xlsx};
use clap::Parser;

mod args;
mod xlsx;

fn main() -> Result<(), anyhow::Error> {
    let args = args::Args::parse();
    if let Some(year) = args.year {
        if !(2000..2100).contains(&year) {
            bail!("Invalid year ({year}). Must be between 2000 and 2099.");
        }
    }

    

    Ok(())
}

