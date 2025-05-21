use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct CLIArgs {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand)]
pub enum Commands {
    Verify(InputArgs),
    Metadata(InputArgs),
    #[command(name = "to_json")] 
    Serialize(InputArgs)
}

#[derive(Parser)]
pub struct InputArgs {
    #[arg(index = 1)]
    pub domain_path: String,
    #[arg(short, long)]
    pub problem_path: Option<String>,
    #[arg(short, long)]
    pub output_file: Option<String>,
}