mod cli_args;

use clap::Parser;
use hddl_analyzer::HDDLAnalyzer;
use std::fs;

use cli_args::{CLIArgs, Commands};

pub fn main() {
    // ANSI escape code for green text
    let green = "\x1b[32m";
    // ANSI escape code for red text
    let red = "\x1b[31m";
    // ANSI escape code to reset text color
    let reset = "\x1b[0m";
    let args = CLIArgs::parse();
    match args.command {
        Commands::Metadata(info) => {
            let domain = fs::read(info.domain_path);
            match domain {
                Ok(domain_content) => match HDDLAnalyzer::get_metadata(&domain_content, None) {
                    Ok(result) => {
                        print!("{}", result)
                    }
                    Err(error) => {
                        eprintln!("{}", error)
                    }
                },
                Err(read_error) => {
                    eprintln!("{}ERR!{} Unable to read file, {}", red, reset, read_error)
                }
            }
        }
        Commands::Verify(_) => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn recursion_type_test_integration() {
        let domain = fs::read("domain.hddl");
        match domain {
            Ok(domain_content) => match HDDLAnalyzer::get_metadata(&domain_content, None) {
                Ok(metadata) => {
                    println!("{}", metadata)
                }
                _ => panic!(),
            },
            _ => panic!(),
        }
    }
}
