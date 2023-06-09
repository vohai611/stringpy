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
///
#[macro_export]
macro_rules! apply_utf8 {
    ($ob:expr; $func:expr; $($args:expr),* ) => {
        {

    let result = Python::with_gil(|py| {
        let array = arrow_in::to_rust_array($ob, py).unwrap();
        let array = array.as_any();
        let array: Vec<Option<Cow<str>>> = array
            .downcast_ref::<Utf8Array<i32>>()
            .unwrap_or_else(|| panic!("Expect string array"))
            .iter()
            .map(|i| $func(i, $($args),*))
            .collect();

        let result = arrow2::array::Utf8Array::<i32>::from(array);
        let result = Box::new(result);
        arrow_in::to_py_array(result, py)
    });
    Ok(result?)

    }
};
($ob:expr ;  $func:expr ; $($ob2:ident),* ; $($args:expr),* ) => {
    {

let result = Python::with_gil(|py| {
    let array = arrow_in::to_rust_array($ob, py).unwrap();
    let array = array.as_any();
    let array= array
        .downcast_ref::<Utf8Array<i32>>()
        .unwrap_or_else(|| panic!("Expect string array"));

    $(let $ob2 =  if $ob2.len() == 1 {
        vec![$ob2[0]; array.len()]
     } else {$ob2};)*

    let array: Vec<Option<Cow<str>>> = izip!(array, $($ob2),*)
        .map(|(i1,   $($ob2),*) | $func(i1,  $($ob2),* ,  $($args),*))
        //.map(|(i1, width, side)| $func(i1,width, side,  $($args),*))
        .collect();

    let result = arrow2::array::Utf8Array::<i32>::from(array);
    let result = Box::new(result);
    arrow_in::to_py_array(result, py)
});
Ok(result?)

}
}
}

#[macro_export]
macro_rules!  apply_utf8_bool {
    ($ob:expr; $func:expr; $($args:expr),* ) => {
        {

    let result = Python::with_gil(|py| {
        let array = arrow_in::to_rust_array($ob, py)?;
        let array = array.as_any();
        let array: Vec<Option<bool>> = array
            .downcast_ref::<Utf8Array<i32>>()
            .unwrap()
            .iter()
            .map(|i| $func(i, $($args),*))
            .collect();

        let result = arrow2::array::BooleanArray::from(array);
        let result = Box::new(result);
        arrow_in::to_py_array(result, py)
    });
    Ok(result?)
    }
}
}

#[macro_export]
macro_rules! apply_utf8_i32 {
    ($ob:expr; $func:expr; $($args:expr),* ) => {
        {

    let result = Python::with_gil(|py| {
        let array = arrow_in::to_rust_array($ob, py).unwrap();
        let array = array.as_any();
        let array: Vec<Option<i32>> = array
            .downcast_ref::<Utf8Array<i32>>()
            .unwrap()
            .iter()
            .map(|i| $func(i, $($args),*))
            .collect();

        let result = arrow2::array::Int32Array::from(array);
        let result = Box::new(result);
        arrow_in::to_py_array(result, py)
    });
    Ok(result?)

    }
}
}

pub(crate) use apply_utf8;
pub(crate) use apply_utf8_bool;
pub(crate) use apply_utf8_i32;
