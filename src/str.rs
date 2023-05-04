use crate::apply_utf8;
use crate::arrow_in;
use crate::utils;
use arrow2::array::ListArray;
use arrow2::array::{BooleanArray, Utf8Array};
use arrow2::datatypes::{DataType, Field};
use arrow2::offset::{Offsets, OffsetsBuffer};
use pyo3::{prelude::*, types::PyTuple};
use regex::Regex;
use std::borrow::Cow;

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

    fn count(x: Option<&str>, pat: &Regex) -> Option<i32> {
        if let Some(x) = x {
            return Option::Some(pat.find_iter(x).count() as i32);
        } else {
            None
        }
    }

    let result = Python::with_gil(|py| {
        let array = arrow_in::to_rust_array(array, py).unwrap();
        let rs_cap = array.len();

        let mut rs: Vec<Option<i32>> = Vec::with_capacity(rs_cap);
        let array = array
            .as_any()
            .downcast_ref::<Utf8Array<i32>>()
            .unwrap_or_else(|| panic!("Not a string array"))
            .iter();

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

    fn replace_one<'a>(x: Option<&'a str>, pat: &Regex, replace: &str) -> Option<Cow<'a, str>> {
        if let Some(x) = x {
            return Some(Cow::from(pat.replace(x, replace)));
        }
        None
    }

    apply_utf8!(array; replace_one; &pat, replace,)
}

#[pyfunction]
fn str_replace_all(array: PyObject, pattern: &str, replace: &str) -> PyResult<PyObject> {
    let pat = &Regex::new(pattern).unwrap();

    fn replace_all<'a>(x: Option<&'a str>, pat: &Regex, replace: &str) -> Option<Cow<'a, str>> {
        if let Some(x) = x {
            return Some(Cow::from(pat.replace_all(x, replace)));
        }
        None
    }

    apply_utf8!(array; replace_all; &pat, replace,)
}

#[pyfunction]
fn str_squish(ob: PyObject) -> PyResult<PyObject> {
    fn squish(x: Option<&str>) -> Option<Cow<str>> {
        if let Some(x) = x {
            let a: Vec<_> = x.split_whitespace().collect();
            return Option::Some(Cow::from(a.join(" ")));
        } else {
            None
        }
    }
    utils::apply_utf8!(ob; squish;)
}

#[pyfunction]
fn str_trim(array: PyObject, side: &str) -> PyResult<PyObject> {
    fn trim<'a>(x: Option<&'a str>, side: &str) -> Option<Cow<'a, str>> {
        if let Some(i) = x {
            let out = match side {
                "left" => i.trim_start(),
                "right" => i.trim_end(),
                "both" => i.trim(),
                _ => panic!("Not a valid side, side must be ['left', 'right', 'both'] "),
            };
            return Some(Cow::from(out));
        } else {
            None
        }
    }
    apply_utf8!(array; trim; side,)
}

#[pyfunction]
fn str_detect(array: PyObject, pattern: &str) -> PyResult<PyObject> {
    let pat = Regex::new(pattern).unwrap();

    fn detect(x: Option<&str>, pat: &Regex) -> Option<bool> {
        if let Some(x) = x {
            return Some(pat.is_match(x));
        } else {
            None
        }
    }

    let rs = Python::with_gil(|py| {
        let array = arrow_in::to_rust_array(array, py).unwrap();
        let array = array.as_any();
        let array: Vec<Option<bool>> = array
            .downcast_ref::<Utf8Array<i32>>()
            .unwrap()
            .iter()
            .map(|x| detect(x, &pat))
            .collect();

        let rs = BooleanArray::from(array).boxed();

        arrow_in::to_py_array(rs, py)
    });

    rs
}

#[pyfunction]
fn str_remove_ascent(array: PyObject) -> PyResult<PyObject> {
    let remove_ascent = |x: Option<&str>| {
        if let Some(x) = x {
            Some(Cow::from(unidecode::unidecode(x)))
        } else {
            None
        }
    };
    utils::apply_utf8!(array; remove_ascent;)
}

#[pyfunction]
fn str_trunc(array: PyObject, width: usize, side: &str, ellipsis: &str) -> PyResult<PyObject> {
    fn truncate<'a>(
        x: Option<&'a str>,
        width: usize,
        side: &str,
        ellipsis: &str,
    ) -> Option<Cow<'a, str>> {
        if let Some(x) = x {
            let len_x = x.len();
            if len_x < width {
                return Some(Cow::from(x));
            }

            let a = match side {
                "left" => format!("{}{}", &x[..width], ellipsis),
                "right" => format!("{}{}", ellipsis, &x[(len_x - width)..]),
                "center" => {
                    let middle = (width / 2) as f32;
                    let first = middle.round() as usize;
                    let tail = width - middle as usize;
                    let first = &x[..first];
                    let tail = &x[(len_x - tail)..];
                    format!("{}{}{}", first, ellipsis, tail)
                }
                _ => panic!("Not a valid side, side must be ['left', 'right', 'center']"),
            };
            Some(Cow::from(a))
        } else {
            None
        }
    }

    apply_utf8!(array; truncate; width, side, ellipsis,)
}

#[pyfunction]
fn str_remove(array: PyObject, pattern: &str) -> PyResult<PyObject> {
    str_replace(array, pattern, "")
}

#[pyfunction]
fn str_remove_all(array: PyObject, pattern: &str) -> PyResult<PyObject> {
    str_replace_all(array, pattern, "")
}

#[pyfunction]
fn str_extract(array: PyObject, pattern: &str, group: Option<usize>) -> PyResult<PyObject> {
    let pat = Regex::new(pattern).unwrap_or_else(|_| panic!("Invalid regex pattern: {}", pattern));

    fn extract<'a>(x: Option<&'a str>, pat: &Regex, group: Option<usize>) -> Option<Cow<'a, str>> {
        if let Some(grp) = group {
            if let Some(x) = x {
                return pat
                    .captures(x)
                    .map(|x| Cow::from(x.get(grp).unwrap().as_str()));
            }
        } else {
            if let Some(x) = x {
                return pat.find(x).map(|x| Cow::from(x.as_str()));
            }
        }
        None
    }

    apply_utf8!(array; extract; &pat, group,)
}

//Vec<Option<Vec<Option<String>>>>
#[pyfunction]
fn str_extract_all(array: PyObject, pattern: &str, group: Option<usize>) -> PyResult<PyObject> {
    let pat = Regex::new(pattern).unwrap_or_else(|_| panic!("Invalid regex pattern: {}", pattern));

    fn extract_all<'a>(
        x: Option<&'a str>,
        pat: &Regex,
        group: Option<usize>,
    ) -> Option<Vec<Option<String>>> {
        if let Some(grp) = group {
            if let Some(x) = x {
                return pat
                    .captures_iter(x)
                    .map(|x| Some(x.get(grp).unwrap().as_str().to_string()))
                    .collect::<Vec<_>>()
                    .into();
            }
        } else {
            if let Some(x) = x {
                return pat
                    .find_iter(x)
                    .map(|x| Some(x.as_str().to_string()))
                    .collect::<Vec<_>>()
                    .into();
            }
        }
        None
    }

    let result = Python::with_gil(|py| {
        let array = arrow_in::to_rust_array(array, py).unwrap();
        let array = array.as_any();
        let mut array: Vec<Option<Vec<Option<String>>>> = array
            .downcast_ref::<Utf8Array<i32>>()
            .unwrap()
            .iter()
            .map(|i| extract_all(i, &pat, group))
            .collect();

        let length_each: Vec<usize> = array
            .iter()
            .map(|x| if let Some(x) = x { x.len() } else { 1 }) // None still take length 1
            .collect();

        let array2 = array
            .iter_mut()
            .reduce(|x, y| {
                if let Some(x_in) = x {
                    if let Some(y) = y {
                        x_in.append(y);
                    } else {
                        x_in.push(None)
                    }  
                } else {
                    if let Some(y) = y {
                        let mut tmp = vec![None];
                        tmp.append(y);
                        *x = Some(tmp);
                    } else {
                        *x = Some(vec![None, None])
                    }
                }
                x
            })
            .unwrap()
            .as_ref()
            .unwrap();

        let _field = Box::new(Field::new("_", DataType::Utf8, true));
        let _list = DataType::List(_field);

        let offset = Offsets::try_from_iter(length_each.into_iter()).unwrap();
        let offset_buf = OffsetsBuffer::from(offset);
        let ar2 = Utf8Array::<i32>::from(array2);
        let b2: ListArray<i32> = ListArray::new(_list, offset_buf, Box::new(ar2), None);
        arrow_in::to_py_array(b2.boxed(), py)
    });

    result
}

#[pymodule]
fn _stringpy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(str_c, m)?)?;
    m.add_function(wrap_pyfunction!(str_combine, m)?)?;
    m.add_function(wrap_pyfunction!(str_count, m)?)?;
    m.add_function(wrap_pyfunction!(str_replace, m)?)?;
    m.add_function(wrap_pyfunction!(str_replace_all, m)?)?;
    m.add_function(wrap_pyfunction!(str_remove, m)?)?;
    m.add_function(wrap_pyfunction!(str_remove_all, m)?)?;
    m.add_function(wrap_pyfunction!(str_remove_ascent, m)?)?;
    m.add_function(wrap_pyfunction!(str_squish, m)?)?;
    m.add_function(wrap_pyfunction!(str_trim, m)?)?;
    m.add_function(wrap_pyfunction!(str_detect, m)?)?;
    m.add_function(wrap_pyfunction!(str_trunc, m)?)?;
    m.add_function(wrap_pyfunction!(str_extract, m)?)?;
    m.add_function(wrap_pyfunction!(str_extract_all, m)?)?;
    Ok(())
}
