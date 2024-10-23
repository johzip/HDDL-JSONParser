mod cli_args;

use clap::Parser;
use hddl_analyzer::HDDLAnalyzer;
use std::fs;

use cli_args::CLIArgs;

pub fn main() {
    // ANSI escape code for green text
    let green = "\x1b[32m";
    // ANSI escape code for red text
    let red = "\x1b[31m";
    // ANSI escape code to reset text color
    let reset = "\x1b[0m";
    let args = CLIArgs::parse();
    let domain = fs::read(args.domain_path);
    match domain {
        Ok(domain_content) => {
            match HDDLAnalyzer::get_metadata(&domain_content, None) {
                Ok(metadata) => {
                    println!("{}", metadata)
                }
                Err(error) => {
                    eprintln!("{}", error)
                }
            }
            // if let Some(problem_path) = args.problem_path {
            //     let problem = fs::read(problem_path);
            //     match problem {
            //         Ok(problem_content) => {
            //             match HDDLAnalyzer::verify(&domain_content, Some(&problem_content)) {
            //                 Ok(warnings) => {
            //                     for warning in warnings {
            //                         println!("{}", warning);
            //                     }
            //                     println!("");
            //                     println!(
            //                         "{}OK!{}",
            //                         green, reset
            //                     )
            //                 }
            //                 Err(s) => eprintln!("{}", s),
            //             }
            //         }
            //         Err(read_error) => {
            //             eprintln!("{}ERR!{} Unable to read file, {}", red, reset, read_error)
            //         }
            //     }
            // }
        }
        Err(read_error) => {
            eprintln!("{}ERR!{} Unable to read file, {}", red, reset, read_error)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn recursion_type_test_integration() {
        let domain = fs::read("domain.hddl");
        match domain {
            Ok(domain_content) => {
                match HDDLAnalyzer::get_metadata(&domain_content, None) {
                    Ok(metadata) => {
                        println!("{}", metadata)
                    }
                    _ => panic!()
                }
            }
            _ => panic!()
        }
    }
}