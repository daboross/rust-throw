extern crate regex;
#[macro_use]
extern crate throw;

use throw::Result;

macro_rules! assert_matches {
    ($expected:expr, $actual:expr) => ({
        let expected = ($expected).replace("    ", "\t");
        let re = regex::Regex::new(&expected).expect("expected hardcoded regex to compile");

        let actual = format!("{}", $actual);

        assert!(re.is_match(&actual),
            format!("expected error to match regex `\n{}\n`, but found `\n{}\n`", expected, actual));
    })
}

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
    assert_eq!(*error.error(), "hi");
    assert_matches!(
        r#"Error: hi
    at [0-9]+:[0-9] in exceptions_work \([a-z/._-]+\)"#,
        error
    );
    assert_eq!("hi".to_owned(), error.into_origin());
}

#[test]
#[allow(deprecated)]
fn test_multiple_throws() {
    let error = throw3().unwrap_err();
    assert_eq!(error.error(), &());
    assert_eq!(error.error(), error.original_error());
    assert_matches!(
        r#"Error: \(\)
    at [0-9]+:[0-9] in exceptions_work \([a-z/._-]+\)
    at [0-9]+:[0-9] in exceptions_work \([a-z/._-]+\)
    at [0-9]+:[0-9] in exceptions_work \([a-z/._-]+\)"#,
        format!("{:?}", error)
    );
}

#[test]
fn test_returns_ok() {
    let ok = throws_ok().unwrap();
    assert_eq!(ok, "ok");
}

#[test]
fn test_mod_throw() {
    let error = mod_test::throws().unwrap_err();
    assert_matches!(
        r#"Error: ahhhh
    at [0-9]+:[0-9] in exceptions_work::mod_test \([a-z/._-]+\)"#,
        error
    );
}

#[test]
fn test_throws_into() {
    let error = throws_into().unwrap_err();
    assert_matches!(
        r#"Error: some static string
    at [0-9]+:[0-9] in exceptions_work \([a-z/._-]+\)"#,
        error
    )
}
