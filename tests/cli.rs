use std::error::Error;

use assert_cmd::Command;

// custom type for test results
type TestResult = Result<(), Box<dyn Error>>;

const PRGM: &str = "pdet";

#[test]
pub fn run_succeeds() -> TestResult {
    //Program runs with no erorrs

    Command::cargo_bin(PRGM)?.assert().success();

    Ok(())
}
