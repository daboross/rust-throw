#![feature(test)]

extern crate test;

#[macro_use]
extern crate throw;

use test::Bencher;

fn gives_throw_ok() -> Result<&'static str, throw::Error<&'static str>> {
    test::black_box(Ok("ok"))
}

fn gives_ok() -> Result<&'static str, &'static str> {
    test::black_box(Ok("ok"))
}

fn throws_up_ok() -> Result<&'static str, throw::Error<&'static str>> {
    let ok_msg = up!(gives_throw_ok());
    Ok(ok_msg)
}

fn throws_throw_ok() -> Result<&'static str, throw::Error<&'static str>> {
    let ok_msg = throw!(gives_ok());
    Ok(ok_msg)
}

fn throws_try_ok() -> Result<&'static str, &'static str> {
    let ok_msg = try!(gives_ok());
    Ok(ok_msg)
}

#[bench]
fn bench_throw_ok_return(bench: &mut Bencher) {
    bench.iter(|| {
        throws_throw_ok()
    })
}

#[bench]
fn bench_up_ok_return(bench: &mut Bencher) {
    bench.iter(|| {
        throws_up_ok()
    })
}

#[bench]
fn bench_try_ok_return(bench: &mut Bencher) {
    bench.iter(|| {
        throws_try_ok()
    })
}
