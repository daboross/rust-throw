#[macro_use]
extern crate throw;


fn throw_static_message() -> Result<(), throw::Error> {
    throw!("hi");

    Ok(())
}

#[test]
fn test_static_message() {
    let error = throw_static_message().unwrap_err();
    assert_eq!(error.original_error().to_string(), "hi");
    assert_eq!("Error: hi\n\tat 6:4 in lib (tests/lib.rs)", format!("{}", error));
}
