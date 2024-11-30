use std::io::Result;
use structopt::StructOpt;
mod cli;

fn main() -> Result<()> {
    let cli::Adapters { apps } = cli::Adapters::from_args();
    apps.match_apps()?;
    Ok(())
}
