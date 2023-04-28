use crate::arrow_in;
use crate::utils;
use arrow2::array::{BooleanArray, Utf8Array};
use pyo3::{prelude::*, types::PyTuple};
use regex::Regex;
use unidecode::unidecode;

#[pyfunction]
fn str_c(array: PyObject, collapse: Option<&str>) -> PyResult<String> {
    let collapse = collapse.unwrap_or("");
    let mut result = Python::with_gil(|py| {
        let array = arrow_in::to_rust_array(array, py).unwrap();
        let array = array.as_any();
        let array = array.downcast_ref::<Utf8Array<i32>>().unwrap();

        array
            .iter()
            .map(|i| {
                let mut val = i.unwrap_or("").to_string();
                val.push_str(collapse);
                val
            })
            .reduce(|x, y| x + &y)
            .unwrap()
    });
    let len = collapse.len();
    for _ in 0..len {
        result.pop();
    }

    Ok(result)
}

#[pyfunction]
#[pyo3(signature = (*py_args, sep))]
fn str_combine(py_args: &PyTuple, sep: Option<&str>) -> PyResult<Vec<String>> {
    let sep = sep.unwrap_or("");
    println!("sep: {}", sep);

    // FIXME  consider using a macro to avoid this boilerplate
    let mut a: Vec<Utf8Array<i32>> = Python::with_gil(|py| {
        py_args
            .into_iter()
            .map(|ob| utils::list_array(ob.to_object(py), py))
            .collect()
    });

    let last_element = a.pop().unwrap();

    let rs: Vec<String> = vec![String::with_capacity(50); a[0].len()];

    fn combine_two(mut x1: Vec<String>, x2: &Utf8Array<i32>, sep: &str) -> Vec<String> {
        x1.iter_mut().zip(x2.iter()).for_each(|(v1, v2)| {
            v1.push_str(v2.unwrap());
            v1.push_str(sep);
        });
        x1
    }

    let rs = a.iter().fold(rs, |x1, x2| combine_two(x1, x2, sep));
    let rs = combine_two(rs, &last_element, "");

    Ok(rs)
}

#[pyfunction]
fn str_count(array: PyObject, pattern: &str) -> PyResult<PyObject> {
    let pat = Regex::new(pattern).unwrap_or_else(|_| panic!("not a valid regex"));

    fn count(x: &str, pat: &Regex) -> Option<i32> {
        Option::Some(pat.find_iter(x).count() as i32)
    }

    let result = Python::with_gil(|py| {
        let array = arrow_in::to_rust_array(array, py).unwrap();
        let rs_cap = array.len();

        let mut rs: Vec<Option<i32>> = Vec::with_capacity(rs_cap);
        let array = array
            .as_any()
            .downcast_ref::<Utf8Array<i32>>()
            .unwrap_or_else(|| panic!("Not a string array"))
            .values_iter();

        for i in array {
            rs.push(count(i, &pat));
        }

        let result = arrow2::array::Int32Array::from(rs);
        arrow_in::to_py_array(result.boxed(), py)
    });

    return result;
}

#[pyfunction]
fn str_replace(array: PyObject, pattern: &str, replace: &str) -> PyResult<PyObject> {
    let pat = Regex::new(pattern).unwrap();

    fn replace_one(x: &str, pat: &Regex, replace: &str) -> Option<String> {
        Option::Some(pat.replace(x, replace).to_string())
    }

    let result = Python::with_gil(|py| {
        let array = arrow_in::to_rust_array(array, py).unwrap();
        let array: Vec<Option<String>> = array
            .as_any()
            .downcast_ref::<Utf8Array<i32>>()
            .unwrap()
            .values_iter()
            .map(|i| replace_one(i, &pat, &replace))
            .collect();

        let result = arrow2::array::Utf8Array::<i32>::from(array);
        let result = Box::new(result);
        arrow_in::to_py_array(result, py)
    });

    return result;
}

#[pyfunction]
fn str_replace_all(array: PyObject, pattern: &str, replace: &str) -> PyResult<PyObject> {
    let pat = &Regex::new(pattern).unwrap();

    let result = Python::with_gil(|py| {
        let array = arrow_in::to_rust_array(array, py).unwrap();
        let mut rs: Vec<Option<String>> = Vec::with_capacity(array.len());

        let array = array.as_any().downcast_ref::<Utf8Array<i32>>().unwrap();

        array
            .values_iter()
            .for_each(|i| rs.push(Some(pat.replace_all(i, replace).into_owned())));

        let result = arrow2::array::Utf8Array::<i32>::from(rs);
        arrow_in::to_py_array(result.boxed(), py)
    });

    return result;
}

#[pyfunction]
fn str_squish(ob: PyObject) -> PyResult<PyObject> {
    fn squish(x: &str) -> Option<String> {
        let a: Vec<_> = x.split_whitespace().collect();
        Option::Some(a.join(" "))
    }

    let result = Python::with_gil(|py| {
        let array = arrow_in::to_rust_array(ob, py).unwrap();
        let array = array.as_any();
        let array: Vec<Option<String>> = array
            .downcast_ref::<Utf8Array<i32>>()
            .unwrap()
            .iter()
            .map(|i| squish(i.unwrap()))
            .collect();

        let result = arrow2::array::Utf8Array::<i32>::from(array);
        let result = Box::new(result);
        arrow_in::to_py_array(result, py)
    });

    return result;
}

#[pyfunction]
fn str_trim(array: PyObject, side: &str) -> PyResult<PyObject> {

    fn func<'a> (x: &'a str, side: &str ) -> &'a str {

     match side {
        "left" => x.trim_start(),
        "right" => x.trim_end(),
        "both" => x.trim(),
        _ => panic!("Not a valid side, side must be ['left', 'right', 'both'] "),
    }
}

    let rs = Python::with_gil(|py| {
        let array = arrow_in::to_rust_array(array, py).unwrap();
        let array = array.as_any();
        let array: Vec<Option<&str>> = array
            .downcast_ref::<Utf8Array<i32>>()
            .unwrap()
            .iter()
            .map(|x| Some(func(x.unwrap(), side)))
            .collect();

        let rs = Utf8Array::<i32>::from(array).boxed();

        arrow_in::to_py_array(rs, py)
    });

    rs
}

#[pyfunction]
fn str_detect(array: PyObject, pattern: &str) -> PyResult<PyObject> {
    let rs = Python::with_gil(|py| {
        let array = arrow_in::to_rust_array(array, py).unwrap();
        let array = array.as_any();
        let array: Vec<Option<bool>> = array
            .downcast_ref::<Utf8Array<i32>>()
            .unwrap()
            .iter()
            .map(|x| Some(Regex::new(pattern).unwrap().is_match(x.unwrap())))
            .collect();

        let rs = BooleanArray::from(array).boxed();

        arrow_in::to_py_array(rs, py)
    });

    rs
}

#[pyfunction]
fn str_remove_ascent(array: PyObject) -> PyResult<Vec<String>> {
    let rs = Python::with_gil(|py| {
        let array = arrow_in::to_rust_array(array, py).unwrap();
        let array = array.as_any();
        let array: Vec<String> = array
            .downcast_ref::<Utf8Array<i32>>()
            .unwrap()
            .iter()
            .map(|x| unidecode(x.unwrap()))
            .collect();
        array
    });

    Ok(rs)
}

#[pymodule]
fn _stringpy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(str_c, m)?)?;
    m.add_function(wrap_pyfunction!(str_combine, m)?)?;
    m.add_function(wrap_pyfunction!(str_count, m)?)?;
    m.add_function(wrap_pyfunction!(str_replace, m)?)?;
    m.add_function(wrap_pyfunction!(str_replace_all, m)?)?;
    m.add_function(wrap_pyfunction!(str_remove_ascent, m)?)?;
    m.add_function(wrap_pyfunction!(str_squish, m)?)?;
    m.add_function(wrap_pyfunction!(str_trim, m)?)?;
    m.add_function(wrap_pyfunction!(str_detect, m)?)?;
    Ok(())
}
