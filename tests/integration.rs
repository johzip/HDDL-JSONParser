extern crate hddl_analyzer;

use std::fs;
use hddl_analyzer::analyze;

#[test]
pub fn domains_integration_test() {
    let domain_paths = fs::read_dir("tests/domains").unwrap();
    for path in domain_paths {
        let domain = fs::read(path.unwrap().path());
        if let Ok(program) = domain {
            let result = analyze(&program);
            if result.is_err() {
                panic!("read failed");
            }
        } else {
            panic!("error reading file");
        }
    }
}

#[test]
pub fn problems_integration_test() {
    let problem_paths = fs::read_dir("tests/problems").unwrap();
    for path in problem_paths {
        let problem = fs::read(path.unwrap().path());
        if let Ok(program) = problem {
            let result = analyze(&program);
            if result.is_err() {
                panic!("read failed");
            }
        } else {
            panic!("error reading file");
        }
    }
}