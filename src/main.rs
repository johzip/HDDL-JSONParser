use hddl_analyzer::HDDLAnalyzer;
use std::{env, fs};

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::read(&args[1]);
    // ANSI escape code for green text
    let green = "\x1b[32m";
    // ANSI escape code for red text
    let red = "\x1b[31m";
    // ANSI escape code to reset text color
    let reset = "\x1b[0m";
    match file {
        Ok(program) => match HDDLAnalyzer::verify(&program) {
            Ok(_) => {
                println!("{}OK!{} {} has been successfully parsed.", green, reset, args[1])
            }
            Err(s) => eprintln!("{}", s),
        },
        Err(read_error) => {
            eprintln!("{}ERR{} Unable to read file, {}", red, reset, read_error)
        }
    }
}
