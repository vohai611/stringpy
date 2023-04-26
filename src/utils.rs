use crate::arrow_in;
use pyo3::prelude::*;
use arrow2::array::Utf8Array;

/// Converts a Python list of strings to a Utf8Array
pub fn list_array(ob: PyObject, py: Python) -> Utf8Array<i32> {

    let array = arrow_in::to_rust_array(ob, py).unwrap();
    let array = array.as_any();
    let array = array
        .downcast_ref::<Utf8Array<i32>>()
        .unwrap();
    array.to_owned()

}