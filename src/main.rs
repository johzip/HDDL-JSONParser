use hddl_analyzer::HDDLAnalyzer;
use std::{env, fs};

pub fn main() {
    // ANSI escape code for green text
    let green = "\x1b[32m";
    // ANSI escape code for red text
    let red = "\x1b[31m";
    // ANSI escape code to reset text color
    let reset = "\x1b[0m";

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("expected one or two arguments, found zero.");
        return;
    } else {
        let domain = fs::read(&args[1]);
        match domain {
            Ok(domain_content) => {
                if args.len()  > 2 {
                    let problem = fs::read(&args[2]);
                    match problem {
                        Ok(problem_content) => {
                            match HDDLAnalyzer::verify(&domain_content, Some(&problem_content)) {
                                Ok(warnings) => {
                                    println!(
                                        "{}OK!{} {} has been successfully parsed.",
                                        green, reset, args[1]
                                    )
                                }
                                Err(s) => eprintln!("{}", s),
                            }
                        }
                        Err(read_error) => {
                            eprintln!("{}ERR!{} Unable to read file, {}", red, reset, read_error)
                        }
                    }
                }
            }
            Err(read_error) => {
                eprintln!("{}ERR!{} Unable to read file, {}", red, reset, read_error)
            }
        }
    }
}
