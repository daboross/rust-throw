use std::error;
use std::fmt;

pub type BoxedStdError = Box<error::Error + Sync + Send>;

pub struct ErrorPoint {
    pub line: u32,
    pub column: u32,
    pub module: &'static str,
    pub file: &'static str,
}

pub struct Error {
    points: Vec<ErrorPoint>,
    original_error: BoxedStdError,
}

impl Error {
    pub fn points(&self) -> &[ErrorPoint] {
        &self.points
    }

    pub fn original_error(&self) -> &BoxedStdError {
        &self.original_error
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(write!(fmt, "Error: {}", self.original_error.description()));
        let mut cause: &error::Error = &*self.original_error;
        while let Some(e) = cause.cause() {
            cause = e;
            try!(write!(fmt, "\n\t (reported cause: {})", e.description()));
        }
        for point in self.points.iter().rev() {
            try!(write!(fmt, "\n\tat {}:{} in {} ({})", point.line, point.column,
                point.module, point.file));
        }

        Ok(())
    }
}

pub trait ThrowBehavior {
    type OkResult;

    fn handle_throw(self, point: ErrorPoint) -> Result<Self::OkResult, Error>;
}

impl<T, E> ThrowBehavior for Result<T, E> where E: Into<BoxedStdError> {
    type OkResult = T;

    fn handle_throw(self, point: ErrorPoint) -> Result<T, Error> {
        match self {
            Ok(x) => Ok(x),
            Err(e) => {
                Err(Error {
                    points: vec![point],
                    original_error: e.into(),
                })
            }
        }
    }
}

impl<T> ThrowBehavior for Result<T, Error> {
    type OkResult = T;

    fn handle_throw(self, point: ErrorPoint) -> Result<T, Error> {
        match self {
            Ok(x) => Ok(x),
            Err(mut e) => {
                e.points.push(point);
                Err(e)
            }
        }
    }
}

impl ThrowBehavior for &'static str {
    type OkResult = ();

    fn handle_throw(self, point: ErrorPoint) -> Result<(), Error> {
        Err(Error {
            points: vec![point],
            original_error: self.to_owned().into(),
        })
    }
}

#[macro_export]
macro_rules! throw {
    ($e:expr) => (
        match $crate::ThrowBehavior::handle_throw($e, $crate::ErrorPoint {
            line: line!(),
            column: column!(),
            module: module_path!(),
            file: file!(),
        }) {
            Ok(v) => v,
            Err(e) => return Err(e),
        }
    )
}
