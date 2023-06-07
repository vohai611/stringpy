use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[derive(Debug)]
pub enum StringpyErr {
    RegexErr(regex::Error),
    PyErr(PyErr),
}

impl StringpyErr {
    pub fn new_value_err<S: Into<String>>(message: S) -> Self {
        StringpyErr::PyErr(PyValueError::new_err(message.into()))
    }
}

impl From<regex::Error> for StringpyErr {
    fn from(err: regex::Error) -> Self {
        StringpyErr::RegexErr(err)
    }
}

impl From<StringpyErr> for PyErr {
    fn from(string_err: StringpyErr) -> Self {
        match string_err {
            // handle regex parse error
            StringpyErr::RegexErr(err) => PyValueError::new_err(err.to_string()),
            // handle pyo3 error
            StringpyErr::PyErr(err) => err,
        }
    }
}

impl From<PyErr> for StringpyErr {
    fn from(err: PyErr) -> Self {
        StringpyErr::PyErr(err)
    }
}
