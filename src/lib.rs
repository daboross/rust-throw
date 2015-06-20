/// Throw!
/// ------
///
/// Throw is a new experimental rust error handling library, meant to assist and build on existing
/// error handling systems.
///
/// Throw exports two structs, `throw::ErrorPoint` and `throw::Error`. `throw::Error` stores a
/// single `original_error` variable which it is created from, and then a list of `ErrorPoint`s
/// which is added to every time you `throw!()` the error to a higher function.
///
/// *Throw does not replace existing error handling systems*. Instead, you will want to replace
/// functions which return `Result<T, MyError>` with `Result<T, throw::Error<MyError>>` for full
/// affect.
///
/// The main way you use throw is through the `throw!()` macro, which acts exactly like `try!()`
/// except specifically for functions returning a `Result<T, throw::Error<E>>` rather than any
/// `Result<T, E>`. You can `throw!()` a regular `Result<T, E>`, but the function within which you
/// use `throw!()` must return `Result<T, throw::Error<E>>`.
///
/// Here's an example of throw in action:
///
/// ```rust
/// #[macro_use]
/// extern crate throw;
///
/// use std::io::prelude::*;
/// use std::io;
/// use std::fs::File;
///
/// fn read_log() -> Result<String, throw::Error<io::Error>> {
///     let mut file = throw!(File::open("some_file.log"));
///     let mut buf = String::new();
///     throw!(file.read_to_string(&mut buf));
///     Ok((buf))
/// }
///
/// fn do_things() -> Result<(), throw::Error<io::Error>> {
///     let log_contents = throw!(read_log());
///     println!("Log contents: {}", log_contents);
///
///     Ok(())
/// }
///
/// fn main() {
///     let result = do_things();
///     if let Err(e) = result {
/// #       /*
///         panic!("{}", e);
/// #       */
/// #       assert_eq!(format!("{}", e), "Error: No such file or directory (os error 2)\
/// #       \n\tat 16:23 in rust_out (<anon>)\
/// #       \n\tat 9:19 in rust_out (<anon>)");
///     }
/// }
/// ```
///
/// This simple program behaves exactly as if `Result<_, io::Error>` directly when it functions
/// correctly. When the program encounters is when throw really shines. Instead of a simple `No
/// such file or directory` message, you get:
///
/// ```text
/// Error: No such file or directory (os error 2)
///    at 16:23 in main (src/main.rs)
///    at 9:19 in main (src/main.rs)
/// ```
///
/// These stack traces are stored inside throw::Error, and are recorded automatically when
/// `throw!()` returns an Err value.
///
/// ---
///
/// Throwing directly from a function is also supported, using `throw_new!()`:
///
/// ```
/// # #[macro_use]
/// # extern crate throw;
/// fn possibly_fails() -> Result<(), throw::Error<&'static str>> {
///     if true {
///         // throw_new!() will always return directly
///         throw_new!("oops");
///     }
///
///     Ok(())
/// }
///
/// fn main() {
/// #   /*
///     possibly_fails().unwrap()
/// #   */
/// #   assert_eq!(format!("{}", possibly_fails().unwrap_err()), "Error: oops\
/// #   \n\tat 6:8 in rust_out (<anon>)")
/// }
/// ```
///
/// ```text
/// called `Result::unwrap()` on an `Err` value: Error: "oops"
///    at 6:8 in main (src/main.rs)
/// ```
///
/// `throw!()` can also be used in place of `throw_new!()` *if the argument provided is &str or
/// String. `throw!()` is special cased so that if you provide it a string literal rather than a
/// Result, it will throw the string literal directly.

use std::fmt;

pub struct ErrorPoint {
    pub line: u32,
    pub column: u32,
    pub module: &'static str,
    pub file: &'static str,
}

pub struct Error<E> {
    points: Vec<ErrorPoint>,
    original_error: E,
}

impl<E> Error<E> {
    #[doc(hidden)]
    pub fn __push_point(&mut self, point: ErrorPoint) {
        self.points.push(point);
    }

    pub fn points(&self) -> &[ErrorPoint] {
        &self.points
    }

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

pub trait ThrowBehavior<E> {
    type OkType;

    fn handle_throw(self) -> Result<Self::OkType, Error<E>>;
}

impl<T, OE, NE> ThrowBehavior<NE> for Result<T, OE> where OE: Into<Error<NE>> {
    type OkType = T;

    fn handle_throw(self) -> Result<T, Error<NE>> {
        match self {
            Ok(x) => Ok(x),
            Err(e) => {
                Err(e.into())
            }
        }
    }
}

impl<'a, E> ThrowBehavior<E> for &'a str where &'a str: Into<E> {
    type OkType = ();

    fn handle_throw(self) -> Result<(), Error<E>> {
        Err(Error {
            points: Vec::new(),
            original_error: self.into(),
        })
    }
}
impl<E> ThrowBehavior<E> for String where String: Into<E> {
    type OkType = ();

    fn handle_throw(self) -> Result<(), Error<E>> {
        Err(Error {
            points: Vec::new(),
            original_error: self.into(),
        })
    }
}

#[macro_export]
macro_rules! throw {
    ($e:expr) => (
        match $crate::ThrowBehavior::handle_throw($e) {
            Ok(v) => v,
            Err(mut e) => {
                e.__push_point($crate::ErrorPoint {
                    line: line!(),
                    column: column!(),
                    module: module_path!(),
                    file: file!(),
                });
                return Err(e);
            },
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
