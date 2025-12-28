use clap::Parser;
use std::path::PathBuf;

#[dervive(Parser)]
#[command(name = "tidy")]
#[command(about = "cli tool to stay organized. built in Rust")]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[dervive(clap::subcommand)]
enum Commands {
    Scan {
        #[arg(default_value_t]
    }
}

