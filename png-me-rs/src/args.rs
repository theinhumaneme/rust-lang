use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}
#[derive(Subcommand)]
pub enum Commands {
    Encode {
        path: String,
        chunk_type: String,
        message: String,
    },
    Decode {
        path: String,
        chunk_type: String,
    },
    Remove {
        path: String,
        chunk_type: String,
    },
    Print {
        path: String,
    },
}
