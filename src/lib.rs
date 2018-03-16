#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(core, alloc))]
#![deny(missing_docs)]
#![doc(html_root_url = "https://docs.rs/throw/0.1.3")]
//! Throw!
//! ------
//!
//! Throw is a new experimental rust error handling library, meant to assist and build on existing
//! error handling systems.
//!
//! Throw exports two structs, `throw::ErrorPoint` and `throw::Error`. `throw::Error` stores a
//! single `original_error` variable which it is created from, and then a list of `ErrorPoint`s
//! which starts out with the original point of creation with `throw!()`, and is added to every
//! time you propagate the error upwards with `up!()`.
//!
//! *Throw does not replace existing error handling systems*. The `throw::Error` type has a type
//! parameter `E` which represents an internal error type stored. `throw::Error` just wraps your
//! error type and stores ErrorPoints alongside it.
//!
//! Throw helps you better keep track of your errors. Instead of seeing a generic "No such file or
//! directory" message, you get a stack trace of functions which propagated the error as well.
//!
//! Instead of:
//!
//! ```text
//! IO Error: failed to lookup address information: Name or service not known
//! ```
//!
//! Get:
//!
//! ```text
//! Error: IO Error: failed to lookup address information: Name or service not known
//!     at 79:17 in zaldinar::startup (src/startup.rs)
//!     at 104:4 in zaldinar::startup (src/startup.rs)
//!     at 28:17 in zaldinar_irclib (/home/daboross/Projects/Rust/zaldinar/zaldinar-irclib/src/lib.rs)
//! ```
//!
//! ---
//!
//! Using throw!
//! ---
//!
//! The main way you use throw is through two macros, `throw!()` and `up!()`. `throw!()` is used
//! when you have a regular (non-throw) result coming from some library function that you want to
//! propagate upwards in case of an error. `up!()` is used when you have an error which was
//! created using `throw!()` in a sub-function which you want to add an error point to and
//! propagate upwards.
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
//!     let log_contents = up!(read_log());
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
//! #       let err = e.to_string();
//! #       assert!(err.starts_with("Error: No such file or directory (os error 2)\
//! #       \n\tat "), "mangled error message: {}", err);
//!     }
//! }
//! ```
//!
//! This simple program behaves exactly as if `Result<_, io::Error>` directly when it functions
//! correctly. When the program encounters is when throw really shines.  This will result in an
//! error message:
//!
//! ```text
//! Error: No such file or directory (os error 2)
//!    at 16:23 in main (src/main.rs)
//!    at 9:19 in main (src/main.rs)
//! ```
//!
//! These stack traces are stored inside throw::Error, and are recorded automatically when
//! `throw!()` or `up!()` returns an Err value.
//!
//! In each `at` line, the `16:23` represents `line_num:column_num`, the `main` represents the
//! module path (for example `my_program::sub_module`), and `src/main.rs` represents the path of
//! the file in which `throw!()` was used in.
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
//! #   let err = possibly_fails().unwrap_err().to_string();
//! #   assert!(err.starts_with("Error: oops\n\tat "), "mangled error message: {}",  err);
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
//!
//! ---
//!
//! `no_std`
//! ---
//!
//! Throw offers support for `no_std`, with the caveat that a dependency on `alloc` is still
//! required for `Vec` support. (`throw` uses a Vec to store error points within an error.)
//!
//! To use this feature, depend on `throw` with `default-features = false`:
//!
//! ```toml
//! [dependencies]
//! throw = { version = "0.1", default-features = "false" }
//! ```

#[cfg(feature = "std")]
mod core {
    pub use std::fmt;
    pub use std::result;
}

use core::fmt;

#[cfg(not(feature = "std"))]
#[macro_use]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::Vec;
#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(not(feature = "std"))]
use alloc::borrow::ToOwned;
#[macro_use]
extern crate serde_derive;

extern crate serde;

use serde::ser::{SerializeStruct, SerializeSeq, Serialize, Serializer};

/// Types allowed to be value in the context vector
#[derive(Debug, Clone)]
pub enum ThrowContextValues {
    ///Boolean
    Bool(bool),
    ///Int8
    Int8(i8),
    ///Uint8
    Uint8(u8),
    ///Int16
    Int16(i16),
    ///Uint16
    Uint16(u16),
    ///Int32
    Int32(i32),
    ///Uint32
    Uint32(u32),
    ///Int64
    Int64(i64),
    ///Uint64
    Uint64(u64),
    ///Float32
    Float32(f32),
    ///Float64
    Float64(f64),
    ///String
    String(String),
}

impl Serialize for ThrowContextValues {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        match self {
            &ThrowContextValues::Bool(ref x) => { return Ok(serializer.serialize_bool(*x)?); }
            &ThrowContextValues::Int8(ref x) => { return Ok(serializer.serialize_i8(*x)?); }
            &ThrowContextValues::Uint8(ref x) => { return Ok(serializer.serialize_u8(*x)?); }
            &ThrowContextValues::Int16(ref x) => { return Ok(serializer.serialize_i16(*x)?); }
            &ThrowContextValues::Uint16(ref x) => { return Ok(serializer.serialize_u16(*x)?); }
            &ThrowContextValues::Int32(ref x) => { return Ok(serializer.serialize_i32(*x)?); }
            &ThrowContextValues::Uint32(ref x) => { return Ok(serializer.serialize_u32(*x)?); }
            &ThrowContextValues::Int64(ref x) => { return Ok(serializer.serialize_i64(*x)?); }
            &ThrowContextValues::Uint64(ref x) => { return Ok(serializer.serialize_u64(*x)?); }
            &ThrowContextValues::Float32(ref x) => { return Ok(serializer.serialize_f32(*x)?); }
            &ThrowContextValues::Float64(ref x) => { return Ok(serializer.serialize_f64(*x)?); }
            &ThrowContextValues::String(ref x) => { return Ok(serializer.serialize_str(x)?); }
        };
    }
}


impl fmt::Display for ThrowContextValues {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ThrowContextValues::Bool(ref x) => { write!(f, "{}", x) }
            &ThrowContextValues::Int8(ref x) => { write!(f, "{}", x) }
            &ThrowContextValues::Uint8(ref x) => { write!(f, "{}", x) }
            &ThrowContextValues::Int16(ref x) => { write!(f, "{}", x) }
            &ThrowContextValues::Uint16(ref x) => { write!(f, "{}", x) }
            &ThrowContextValues::Int32(ref x) => { write!(f, "{}", x) }
            &ThrowContextValues::Uint32(ref x) => { write!(f, "{}", x) }
            &ThrowContextValues::Int64(ref x) => { write!(f, "{}", x) }
            &ThrowContextValues::Uint64(ref x) => { write!(f, "{}", x) }
            &ThrowContextValues::Float32(ref x) => { write!(f, "{}", x) }
            &ThrowContextValues::Float64(ref x) => { write!(f, "{}", x) }
            &ThrowContextValues::String(ref x) => { write!(f, "{}", x) }
        }
    }
}

impl Into<ThrowContextValues> for u8 {
    fn into(self) -> ThrowContextValues {
        ThrowContextValues::Uint8(self)
    }
}

impl Into<ThrowContextValues> for i8 {
    fn into(self) -> ThrowContextValues {
        ThrowContextValues::Int8(self)
    }
}

impl Into<ThrowContextValues> for u16 {
    fn into(self) -> ThrowContextValues {
        ThrowContextValues::Uint16(self)
    }
}

impl Into<ThrowContextValues> for i16 {
    fn into(self) -> ThrowContextValues {
        ThrowContextValues::Int16(self)
    }
}

impl Into<ThrowContextValues> for u32 {
    fn into(self) -> ThrowContextValues {
        ThrowContextValues::Uint32(self)
    }
}

impl Into<ThrowContextValues> for i32 {
    fn into(self) -> ThrowContextValues {
        ThrowContextValues::Int32(self)
    }
}

impl Into<ThrowContextValues> for u64 {
    fn into(self) -> ThrowContextValues {
        ThrowContextValues::Uint64(self)
    }
}

impl Into<ThrowContextValues> for i64 {
    fn into(self) -> ThrowContextValues {
        ThrowContextValues::Int64(self)
    }
}

impl Into<ThrowContextValues> for f32 {
    fn into(self) -> ThrowContextValues {
        ThrowContextValues::Float32(self)
    }
}

impl Into<ThrowContextValues> for f64 {
    fn into(self) -> ThrowContextValues {
        ThrowContextValues::Float64(self)
    }
}

impl<'a> Into<ThrowContextValues> for &'a str {
    fn into(self) -> ThrowContextValues {
        ThrowContextValues::String(self.to_owned())
    }
}

impl Into<ThrowContextValues> for String {
    fn into(self) -> ThrowContextValues {
        ThrowContextValues::String(self)
    }
}

/// Result alias for a result containing a throw::Error.
pub type Result<T, E> = core::result::Result<T, Error<E>>;

/// Represents a location at which an error was thrown via throw!()
#[derive(Serialize, Debug)]
pub struct ErrorPoint {
    line: u32,
    column: u32,
    module_path: &'static str,
    file: &'static str,
}


impl ErrorPoint {
    /// The line throw!() occurred at, retrieved by line!()
    #[inline]
    pub fn line(&self) -> u32 {
        self.line
    }

    /// The column throw!() occurred at, retrieved by column!()
    #[inline]
    pub fn column(&self) -> u32 {
        self.column
    }

    /// The module throw!() occurred in, retrieved by module_path!()
    #[inline]
    pub fn module_path(&self) -> &'static str {
        self.module_path
    }

    /// The file throw!() occurred in, retrieved by file!()
    #[inline]
    pub fn file(&self) -> &'static str {
        self.file
    }

    #[doc(hidden)]
    pub fn __construct(
        line: u32,
        column: u32,
        module_path: &'static str,
        file: &'static str,
    ) -> ErrorPoint {
        ErrorPoint {
            line: line,
            column: column,
            module_path: module_path,
            file: file,
        }
    }
}


/// represent a key-value pair
#[derive(Serialize, Debug, Clone)]
pub struct KvPair {
    key: &'static str,
    value: ThrowContextValues,
}

impl KvPair {
    fn new(key: &'static str, value: ThrowContextValues) -> KvPair {
        KvPair { key, value }
    }
    ///key
    pub fn key(&self) -> &'static str {
        self.key
    }
    ///value
    pub fn value(&self) -> &ThrowContextValues {
        &self.value
    }
}


/// Represents an error. Stores an original error of type E, and any number of ErrorPoints at
/// which the error was propagated.
pub struct Error<E> {
    points: Vec<ErrorPoint>,
    context: Vec<KvPair>,
    error: E,
}

struct VecErrorPoint<'a>(&'a [ErrorPoint]);

impl<'a> Serialize for VecErrorPoint<'a>
{
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for e in self.0 {
            seq.serialize_element(e)?;
        }
        seq.end()
    }
}

struct VecKvPair<'a>(&'a [KvPair]);

impl<'a> Serialize for VecKvPair<'a>
{
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for e in self.0 {
            seq.serialize_element(e)?;
        }
        seq.end()
    }
}


impl<E: fmt::Display> Serialize for Error<E> {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut state = serializer.serialize_struct("Error", 3)?;

        state.serialize_field("points", &VecErrorPoint(&self.points))?;
        state.serialize_field("context", &VecKvPair(&self.context))?;
        state.serialize_field::<&str>("error", &format!("{}", self.error).as_str())?;
        state.end()
    }
}


impl<E> Error<E> {
    /// Creates a new Error with no ErrorPoints
    pub fn new(error: E) -> Error<E> {
        Error {
            points: Vec::new(),
            context: Vec::new(),
            error: error,
        }
    }

    /// get context
    pub fn get_context(&self) -> &[KvPair] {
        self.context.as_slice()
    }

    /// For macro use only
    #[doc(hidden)]
    pub fn __add_context<V: Into<ThrowContextValues>>(&mut self, key: &'static str, value: V) {
        self.context.push(KvPair::new(key, value.into()))
    }

    /// For macro use only
    #[doc(hidden)]
    pub fn __push_point(&mut self, point: ErrorPoint) {
        self.points.push(point);
    }

    /// Gets all ErrorPoints where this Error was thrown. These are in reverse order, with the
    /// first time it was thrown first and the latest time it was thrown last.
    #[inline]
    pub fn points(&self) -> &[ErrorPoint] {
        &self.points
    }

    /// Gets the original error which this Error was constructed with.
    #[deprecated = "use `error` instead."]
    #[inline]
    pub fn original_error(&self) -> &E {
        self.error()
    }

    /// Gets the original error which this Error was constructed with.
    #[inline]
    pub fn error(&self) -> &E {
        &self.error
    }

    /// Move the original error out.
    #[inline]
    pub fn into_origin(self) -> E {
        self.into_error()
    }

    /// Take out the original error and transform into another type
    /// where the original error can transform into that type.
    #[inline]
    pub fn into_error<N>(self) -> N
        where
            E: Into<N>,
    {
        self.error.into()
    }

    /// Transforms this Error<OldError> into Error<NewError>. This isn't implemented as an Into or
    /// From implementation because it would conflict with the blanket implementations in stdlib.
    pub fn transform<NE>(self) -> Error<NE>
        where
            E: Into<NE>,
    {
        Error {
            points: self.points,
            context: self.context,
            error: self.error.into(),
        }
    }
}

impl<E> fmt::Display for Error<E>
    where
        E: fmt::Display,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(fmt, "Error: {}", self.error));

        for kv in self.context.iter().rev() {
            try!(write!(
                fmt,
                "\n\t{}: {}",
                kv.key(),
                kv.value(),
            ));
        }

        for point in self.points.iter().rev() {
            try!(write!(
                fmt,
                "\n\tat {}:{} in {} ({})",
                point.line(),
                point.column(),
                point.module_path(),
                point.file()
            ));
        }

        Ok(())
    }
}

impl<E> fmt::Debug for Error<E>
    where
        E: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(fmt, "Error: {:?}", self.error));
        for kv in self.context.iter().rev() {
            try!(write!(
                fmt,
                "\n\t{}: {}",
                kv.key(),
                kv.value(),
            ));
        }
        for point in self.points.iter().rev() {
            try!(write!(
                fmt,
                "\n\tat {}:{} in {} ({})",
                point.line(),
                point.column(),
                point.module_path(),
                point.file()
            ));
        }

        Ok(())
    }
}

#[macro_export]
macro_rules! up {
    ($e:expr) => (
        match $e {
            Ok(v) => v,
            Err(e) => {
                // re-assignment for a better error message if up!() is used incorrectly
                return Err(__with_new_errorpoint!(e.transform()));
            },
        }
    );
    ($e:expr, $($key:expr, $value:expr), *) => (
        match $e {
            Ok(v) => v,
            Err(e) => {
                // re-assignment for a better error message if up!() is used incorrectly
                let mut me = __with_new_errorpoint!(e.transform());
                $(
                    me.__add_context($key, $value);
                )*
                return Err(me);
            },
        }
    )
}

#[doc(hidden)]
#[macro_export]
macro_rules! __with_new_errorpoint {
    ($e:expr) => ({
        let mut e = $e;
        e.__push_point($crate::ErrorPoint::__construct(
            line!(),
            column!(),
            module_path!(),
            file!(),
        ));
        e
    })
}

#[macro_export]
macro_rules! throw {
    ($e:expr) => (
        match $e {
            Ok(v) => v,
            Err(e) => throw_new!(e),
        }
    );

     ($e:expr, $($key:expr, $value:expr), *) => ({
         match $e {
            Ok(v) => v,
            Err(e) => throw_new!(e, $($key:expr, $value:expr)*),
        }
    })
}

#[macro_export]
macro_rules! throw_new {
  ($e:expr) => ({
        return Err(__with_new_errorpoint!($crate::Error::new($e.into())));
    });

  ($e:expr, $($key:expr, $value:expr), *) => ({
        let mut me = $crate::Error::new($e.into());
        $(
            me.__add_context($key, $value);
        )*
        return Err(__with_new_errorpoint!(me));

    })
}
