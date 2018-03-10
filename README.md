Throw!
------
[![Travis CI status][travis-image]][travis-builds]
[![crates.io version badge][cratesio-badge]][crates-io-page]

Efficient, statically-calculated backtraces wrapping any error type.

Documentation: https://docs.rs/throw

Throw does not replace existing error handling systems: instead, it simply provides a
`throw::Error<E>` type which wraps your error and provides additional context.

Throw exports two structs, `throw::ErrorPoint` and `throw::Error`. `throw::Error` stores a
single `original_error` variable which it is created from, and then a list of `ErrorPoint`s
which starts out with the original point of creation with `throw!()`, and is added to every
time you propagate the error upwards with `up!()`.

`throw!()` and `up!()` provide strictly less functionality than alternative crates such as
[backtrace], but they come with the advantage of performance. Using compiler-provided macros,
these functions embed the line number and filename they're used in into the ErrorPoint they
construct, and build the stacktrace piece-by-piece at each point. You additionally won't get
irrelevant stacktrace lines above where the error is handled! This may be good or bad,
depending on your use case.

Throw also only works if you actually use the macros- which is quite a disadvantage. [backtrace]
is most likely what you want if you don't have a strict performance requirement.

Throw in practice: instead of this:

```text
IO Error: failed to lookup address information: Name or service not known
```

you'll get this:

```text
Error: IO Error: failed to lookup address information: Name or service not known
    at 79:17 in zaldinar::startup (src/startup.rs)
    at 104:4 in zaldinar::startup (src/startup.rs)
    at 28:17 in zaldinar_irclib (/home/daboross/Projects/Rust/zaldinar/zaldinar-irclib/src/lib.rs)
```

# `no_std`

`throw` supports building without std, but it will still depend on `alloc` and use `alloc::Vec`. This can be enabled when using nightly rust with `default-features = false`:

```toml
throw = { version = "0.1", default-features = false }
```

[backtrace]: https://crates.io/crates/backtrace
[crates-io-page]: https://crates.io/crates/throw
[travis-image]: https://travis-ci.org/daboross/rust-throw.svg?branch=master
[travis-builds]: https://travis-ci.org/daboross/rust-throw
[cratesio-badge]: http://meritbadge.herokuapp.com/throw
