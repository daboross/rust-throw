Throw!
------

Throw is a new experimental rust error handling library, meant to assist and build on existing
error handling systems.

Throw exports two structs, `throw::ErrorPoint` and `throw::Error`. `throw::Error` stores a
single `original_error` variable which it is created from, and then a list of `ErrorPoint`s
which starts out with the original point of creation with `throw!()`, and is added to every
time you propagate the error upwards with `up!()`.

*Throw does not replace existing error handling systems*. The `throw::Error` type has a type
parameter `E` which represents an internal error type stored. `throw::Error` just wraps your
error type and stores ErrorPoints alongside it.

Throw helps you better keep track of your errors. Instead of seeing a generic "No such file or
directory" message, you get a stack trace of functions which propagated the error as well.

Instead of:

```text
IO Error: failed to lookup address information: Name or service not known
```

Get:

```text
Error: IO Error: failed to lookup address information: Name or service not known
    at 79:17 in zaldinar::startup (src/startup.rs)
    at 104:4 in zaldinar::startup (src/startup.rs)
    at 28:17 in zaldinar_irclib (/home/daboross/Projects/Rust/zaldinar/zaldinar-irclib/src/lib.rs)
```

---

- API Documentation, full usage instructions: https://dabo.guru/rust/throw/
- Travis CI builds: https://travis-ci.org/daboross/rust-throw
- Cargo crates.io page: http://crates.io/crates/throw
