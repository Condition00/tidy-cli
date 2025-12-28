use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "tidy")]
#[command(about = "cli tool to stay organized. built in Rust")]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Scan {
        path: Option<PathBuf>,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { path } => {
            let path = match path {
                Some(p) => p,
                None => dirs::home_dir()
                    .ok_or_else(|| anyhow::anyhow!("could not determine home directory"))?,
            };

            let resolved_path = path.canonicalize()?;
            println!("{}", resolved_path.display());
        }
    }

    Ok(())
}
