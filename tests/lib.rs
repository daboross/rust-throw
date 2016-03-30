#[macro_use]
extern crate throw;

use throw::Result;


fn throw_static_message() -> Result<(), &'static str> {
    throw_new!("hi");
}

fn throw1() -> Result<(), ()> {
    throw_new!(());
}

fn throw2() -> Result<(), ()> {
    up!(throw1());
    Ok(())
}

fn throw3() -> Result<(), ()> {
    up!(throw2());
    Ok(())
}

fn gives_ok() -> Result<&'static str, &'static str> {
    Ok("ok")
}

fn throws_ok() -> Result<&'static str, &'static str> {
    let ok_msg = up!(gives_ok());
    Ok(ok_msg)
}

mod mod_test {
    use throw::Result;

    pub fn throws() -> Result<(), &'static str> {
        throw_new!("ahhhh");
    }
}

fn throws_into() -> Result<(), String> {
    throw!(Err("some static string"));
    Ok(())
}

#[test]
fn test_static_message() {
    let error = throw_static_message().unwrap_err();
    assert_eq!(*error.err(), "hi");
    assert_eq!(error.to_string(), "Error: hi\n\tat 8:4 in lib (tests/lib.rs)");
    assert_eq!("hi".to_owned(), error.into_err::<String>());
}

#[test]
fn test_multiple_throws() {
    let error = throw3().unwrap_err();
    assert_eq!(error.err(), &());
    assert_eq!(format!("{:?}", error), "Error: ()\
    \n\tat 21:4 in lib (tests/lib.rs)\
    \n\tat 16:4 in lib (tests/lib.rs)\
    \n\tat 12:4 in lib (tests/lib.rs)");
}

#[test]
fn test_returns_ok() {
    let ok = throws_ok().unwrap();
    assert_eq!(ok, "ok");
}

#[test]
fn test_mod_throw() {
    let error = mod_test::throws().unwrap_err();
    assert_eq!(error.to_string(), "Error: ahhhh\
    \n\tat 38:8 in lib::mod_test (tests/lib.rs)");
}

#[test]
fn test_throws_into() {
    let error = throws_into().unwrap_err();
    assert_eq!(error.to_string(), "Error: some static string\
    \n\tat 43:4 in lib (tests/lib.rs)")
}
