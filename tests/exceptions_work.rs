extern crate regex;
extern crate serde_json;

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

fn throw_with_context1() -> Result<(), &'static str> {
    throw_new!("Error with context", "code"=>78,"application"=>"rust_core")
}

fn throw_with_context2() -> Result<(), &'static str> {
    up!(throw_with_context1(), "project_secret"=>"omega");
    Ok(())
}

fn throw_with_context3() -> Result<(), &'static str> {
    up!(throw_with_context2(),  "score"=>0.75, "height"=>948);
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

fn throws_into_key_value() -> Result<(), String> {
    throw!(Err("some static string"), "key" => "value");
    Ok(())
}

fn throws_into_multiple_key_value_pairs() -> Result<(), String> {
    throw!(
        Err("some static string"),
        "key" => "value",
        "key2" => "value2",
        "key3" => "value3",
        "key4" => "value4",
    );
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
#[cfg(any(feature = "serde-1", feature = "serde-1-std"))]
fn serialize_json() {
    let error = throw_with_context3().unwrap_err();
    let json = serde_json::to_string(&error).unwrap();
    assert_eq!(r#"{"points":[{"line":40,"column":5,"module_path":"exceptions_work","file":"tests/exceptions_work.rs"},{"line":44,"column":5,"module_path":"exceptions_work","file":"tests/exceptions_work.rs"},{"line":49,"column":5,"module_path":"exceptions_work","file":"tests/exceptions_work.rs"}],"context":[{"key":"code","value":78},{"key":"application","value":"rust_core"},{"key":"project_secret","value":"omega"},{"key":"score","value":0.75},{"key":"height","value":948}],"error":"Error with context"}"#, json);
}

#[test]
fn test_throw_with_context() {
    let error = throw_with_context1().unwrap_err();
    assert_eq!(error.get_context().len(), 2);

    let error2 = throw_with_context3().unwrap_err();
    assert_eq!(error2.get_context().len(), 5);

    assert_matches!(
        r#"Error: "Error with context"
    height: 948
	score: 0.75
	project_secret: omega
	application: rust_core
	code: 78
    at [0-9]+:[0-9] in exceptions_work \([a-z/._-]+\)
    at [0-9]+:[0-9] in exceptions_work \([a-z/._-]+\)
    at [0-9]+:[0-9] in exceptions_work \([a-z/._-]+\)"#,
        format!("{:?}", error2)
    );
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

#[test]
fn test_throws_into_key_value() {
    let error = throws_into_key_value().unwrap_err();
    assert_matches!(
        r#"Error: some static string
    key: value
    at [0-9]+:[0-9] in exceptions_work \([a-z/._-]+\)"#,
        error
    )
}

#[test]
fn test_throws_into_multiple_key_value_pairs() {
    let error = throws_into_multiple_key_value_pairs().unwrap_err();
    assert_matches!(
        r#"Error: some static string
    key4: value4
    key3: value3
    key2: value2
    key: value
    at [0-9]+:[0-9] in exceptions_work \([a-z/._-]+\)"#,
        error
    )
}
