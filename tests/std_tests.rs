#![cfg(feature = "std")]
#[macro_use]
extern crate throw;

use throw::Result;

#[derive(Debug)]
struct CustomError(String);

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "CustomError: {}", self.0)
    }
}

impl std::error::Error for CustomError {
    fn description(&self) -> &str {
        self.0.as_str()
    }
}

fn throws_error_with_description() -> Result<(), CustomError> {
    throw!(Err(CustomError("err".to_owned())));
    Ok(())
}

fn throws_error_with_description_and_key_value_pairs() -> Result<(), CustomError> {
    throw!(
        Err(CustomError("err".to_owned())),
        "key" => "value"
    );
    Ok(())
}

#[test]
fn test_error_description() {
    use std::error::Error;

    let error = throws_error_with_description().unwrap_err();
    assert_eq!(error.description(), "err");
}

#[test]
fn test_error_description_with_key_value_pairs() {
    use std::error::Error;

    let error = throws_error_with_description_and_key_value_pairs().unwrap_err();
    assert_eq!(error.description(), "err");
}

#[test]
fn test_error_with_cause() {
    use std::error::Error;

    let error = throws_error_with_description().unwrap_err();
    assert_eq!(
        format!("{}", error.cause().unwrap()),
        "CustomError: err"
    );
}
