#[macro_use]
extern crate throw;


fn throw_static_message() -> Result<(), throw::Error<&'static str>> {
    throw!("hi");

    Ok(())
}

fn throw1() -> Result<(), throw::Error<()>> {
    throw_new!(());
}

fn throw2() -> Result<(), throw::Error<()>> {
    throw!(throw1());
    Ok(())
}

fn throw3() -> Result<(), throw::Error<()>> {
    throw!(throw2());
    Ok(())
}

fn gives_ok() -> Result<&'static str, throw::Error<&'static str>> {
    Ok("ok")
}

fn throws_ok() -> Result<&'static str, throw::Error<&'static str>> {
    let ok_msg = throw!(gives_ok());
    Ok(ok_msg)
}

mod mod_test {
    use throw;

    pub fn throws() -> Result<(), throw::Error<&'static str>> {
        throw_new!("ahhhh");
    }
}

#[test]
fn test_static_message() {
    let error = throw_static_message().unwrap_err();
    assert_eq!(*error.original_error(), "hi");
    assert_eq!(format!("{}", error), "Error: hi\n\tat 6:4 in lib (tests/lib.rs)");
}

#[test]
fn test_multiple_throws() {
    let error = throw3().unwrap_err();
    assert_eq!(error.original_error(), &());
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
    assert_eq!(format!("{}", error), "Error: ahhhh\
    \n\tat 38:8 in lib::mod_test (tests/lib.rs)");
}
