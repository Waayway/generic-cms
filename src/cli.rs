use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short, long)]
    pub dev: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    Serve,
    Migrate {
        #[command(subcommand)]
        command: MigrateCommands,
    },
}

#[derive(Subcommand)]
pub enum MigrateCommands {
    Up,
    Down,
}
