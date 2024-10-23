use std::path::Path;

use clap::Parser;
#[derive(Parser)]
pub struct CLIArgs {
    #[arg(index = 1)]
    pub domain_path: String,
    #[arg(short, long, default_value = None)]
    pub problem_path: Option<String>
}