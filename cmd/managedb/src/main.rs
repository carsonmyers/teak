mod populate;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use dotenvy::dotenv;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long, env)]
    database_url: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Populate(PopulateArgs),
}

#[derive(Args, Debug)]
pub struct PopulateArgs {
    #[arg(short, long)]
    clear: bool,
}

fn main() -> Result<()> {
    dotenv().ok();

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Populate(args)) => populate::run(args),
        None => Ok(()),
    }?;

    Ok(())
}
