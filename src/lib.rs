#![deny(missing_docs)]
#![doc(html_root_url = "https://dabo.guru/rust/throw/throw/")]
//! Throw!
//! ------
//!
//! Throw is a new experimental rust error handling library, meant to assist and build on existing
//! error handling systems.
//!
//! Throw exports two structs, `throw::ErrorPoint` and `throw::Error`. `throw::Error` stores a
//! single `original_error` variable which it is created from, and then a list of `ErrorPoint`s
//! which is added to every time you `throw!()` the error to a higher function.
//!
//! *Throw does not replace existing error handling systems*. Instead, you will want to replace
//! functions which return `Result<T, MyError>` with `Result<T, throw::Error<MyError>>` for full
//! affect.
//!
//! The main way you use throw is through the `throw!()` macro, which acts exactly like `try!()`
//! except specifically for functions returning a `Result<T, throw::Error<E>>` rather than any
//! `Result<T, E>`. You can `throw!()` a regular `Result<T, E>`, but the function within which you
//! use `throw!()` must return `Result<T, throw::Error<E>>`.
//!
//! Here's an example of throw in action:
//!
//! ```rust
//! #[macro_use]
//! extern crate throw;
//!
//! use std::io::prelude::*;
//! use std::io;
//! use std::fs::File;
//!
//! fn read_log() -> Result<String, throw::Error<io::Error>> {
//!     let mut file = throw!(File::open("some_file.log"));
//!     let mut buf = String::new();
//!     throw!(file.read_to_string(&mut buf));
//!     Ok((buf))
//! }
//!
//! fn do_things() -> Result<(), throw::Error<io::Error>> {
//!     let log_contents = throw!(read_log());
//!     println!("Log contents: {}", log_contents);
//!
//!     Ok(())
//! }
//!
//! fn main() {
//!     let result = do_things();
//!     if let Err(e) = result {
//! #       /*
//!         panic!("{}", e);
//! #       */
//! #       assert_eq!(format!("{}", e), "Error: No such file or directory (os error 2)\
//! #       \n\tat 16:23 in rust_out (<anon>)\
//! #       \n\tat 9:19 in rust_out (<anon>)");
//!     }
//! }
//! ```
//!
//! This simple program behaves exactly as if `Result<_, io::Error>` directly when it functions
//! correctly. When the program encounters is when throw really shines. Instead of a simple `No
//! such file or directory` message, you get:
//!
//! ```text
//! Error: No such file or directory (os error 2)
//!    at 16:23 in main (src/main.rs)
//!    at 9:19 in main (src/main.rs)
//! ```
//!
//! These stack traces are stored inside throw::Error, and are recorded automatically when
//! `throw!()` returns an Err value.
//!
//! In each `at` line, the `16:23` represents `line_num:column_num`, the `main` represents the
//! module path (for example `my_program::sub_module`), and `src/main.rs` represents the path of
//! the file which `throw!()` was used in.
//!
//! ---
//!
//! Throwing directly from a function is also supported, using `throw_new!()`:
//!
//! ```
//! # #[macro_use]
//! # extern crate throw;
//! fn possibly_fails() -> Result<(), throw::Error<&'static str>> {
//!     if true {
//!         // throw_new!() will always return directly
//!         throw_new!("oops");
//!     }
//!
//!     Ok(())
//! }
//!
//! fn main() {
//! #   /*
//!     possibly_fails().unwrap()
//! #   */
//! #   assert_eq!(format!("{}", possibly_fails().unwrap_err()), "Error: oops\
//! #   \n\tat 6:8 in rust_out (<anon>)")
//! }
//! ```
//!
//! ```text
//! called `Result::unwrap()` on an `Err` value: Error: "oops"
//!    at 6:8 in main (src/main.rs)
//! ```
//!
//! `throw_new!()` differs from `throw!()` in that it takes a parameter directly to pass to a
//! `throw::Error`, rather than a `Result<>` to match on. `throw_new!()` will always return
//! directly from the function.

use std::fmt;

/// Represents a location at which an error was thrown via throw!()
pub struct ErrorPoint {
    /// The line throw!() occurred at, retrieved by line!()
    pub line: u32,
    /// The column throw!() occurred at, retrieved by column!()
    pub column: u32,
    /// The module throw!() occurred in, retrieved by module_path!()
    pub module: &'static str,
    /// The file throw!() occurred in, retrieved by file!()
    pub file: &'static str,
}

/// Represents an error. Stores an original error of type E, and any number of ErrorPoints at which the error was `throw!()`n.
pub struct Error<E> {
    points: Vec<ErrorPoint>,
    original_error: E,
}

impl<E> Error<E> {
    #[doc(hidden)]
    pub fn __push_point(&mut self, point: ErrorPoint) {
        self.points.push(point);
    }

    /// Gets all ErrorPoints where this Error was thrown. These are in reverse order, with the
    /// first time it was thrown first and the latest time it was thrown last.
    pub fn points(&self) -> &[ErrorPoint] {
        &self.points
    }

    /// Gets the original error which this Error was constructed with.
    pub fn original_error(&self) -> &E {
        &self.original_error
    }
}

impl<E> fmt::Display for Error<E> where E: fmt::Display {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(write!(fmt, "Error: {}", self.original_error));
        for point in self.points.iter().rev() {
            try!(write!(fmt, "\n\tat {}:{} in {} ({})", point.line, point.column,
                point.module, point.file));
        }

        Ok(())
    }
}

impl<E> fmt::Debug for Error<E> where E: fmt::Debug {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(write!(fmt, "Error: {:?}", self.original_error));
        for point in self.points.iter().rev() {
            try!(write!(fmt, "\n\tat {}:{} in {} ({})", point.line, point.column,
                point.module, point.file));
        }

        Ok(())
    }
}

impl <E> From<E> for Error<E> {
    fn from(error: E) -> Error<E> {
        Error {
            points: Vec::new(),
            original_error: error,
        }
    }
}

#[macro_export]
macro_rules! throw {
    ($e:expr) => (
        match $e {
            Ok(v) => v,
            Err(e) => throw_new!(e),
        }
    )
}

#[macro_export]
macro_rules! throw_new {
    ($e:expr) => ({
        let mut e = $crate::Error::from($e);
        e.__push_point($crate::ErrorPoint {
            line: line!(),
            column: column!(),
            module: module_path!(),
            file: file!(),
        });
        return Err(e);
    })
}
