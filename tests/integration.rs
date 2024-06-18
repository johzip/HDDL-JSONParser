extern crate hddl_analyzer;

use std::fs;
use hddl_analyzer::analyze;

#[test]
pub fn succesful_parsing_test() {
    let domain = fs::read("tests/success.hddl");
    if let Ok(program) = domain {
        let result = analyze(program);
        if result.is_err() {
            panic!("read failed");
        }
    } else {
        panic!("error reading file");
    }
}