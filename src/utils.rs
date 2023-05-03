use crate::arrow_in;
use arrow2::array::Utf8Array;
use pyo3::prelude::*;

/// Converts a Python list of strings to a Utf8Array
pub fn list_array(ob: PyObject, py: Python) -> Utf8Array<i32> {
    let array = arrow_in::to_rust_array(ob, py).unwrap();
    let array = array.as_any();
    let array = array.downcast_ref::<Utf8Array<i32>>().unwrap();
    array.to_owned()
}

/// Apply a function to a Utf8Array and return a new Utf8Array.
/// This function must take one element of input and return one element of output
#[macro_export]
macro_rules! apply_utf8 {
    ($ob:expr; $func:expr; $($args:expr,)* ) => {
        {

    let result = Python::with_gil(|py| {
        let array = arrow_in::to_rust_array($ob, py).unwrap();
        let array = array.as_any();
        let array: Vec<Option<Cow<str>>> = array
            .downcast_ref::<Utf8Array<i32>>()
            .unwrap()
            .iter()
            .map(|i| $func(i, $($args),*))
            .collect();

        let result = arrow2::array::Utf8Array::<i32>::from(array);
        let result = Box::new(result);
        arrow_in::to_py_array(result, py)
    });
    result

    }
}
}

pub(crate) use apply_utf8;
