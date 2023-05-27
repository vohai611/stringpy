use crate::apply_utf8;
use crate::arrow_in;
use crate::atomic;
use crate::error::StringpyErr;
use crate::utils;
use arrow2::array::Int32Array;
use arrow2::array::ListArray;
use arrow2::array::Utf8Array;
use arrow2::datatypes::{DataType, Field};
use arrow2::offset::{Offsets, OffsetsBuffer};
use core::panic;
use itertools::izip;
use itertools::Itertools;
use pyo3::{prelude::*, types::PyTuple};
use regex::escape;
use regex::Regex;
use std::borrow::Cow;
use std::iter::zip;

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
fn str_count(array: PyObject, pattern: &str) -> Result<PyObject, StringpyErr> {
    let pat = Regex::new(pattern)?;

    fn count(x: Option<&str>, pat: &Regex) -> Option<i32> {
        let x = x?;
        return Option::Some(pat.find_iter(x).count() as i32);
    }
    utils::apply_utf8_i32!(array; count; &pat)
}

#[pyfunction]
fn str_replace(array: PyObject, pattern: &str, replace: &str) -> Result<PyObject, StringpyErr> {
    let pat = Regex::new(pattern)?;

    fn replace_one<'a>(x: Option<&'a str>, pat: &Regex, replace: &str) -> Option<Cow<'a, str>> {
        let x = x?;
        return Some(Cow::from(pat.replace(x, replace)));
    }

    apply_utf8!(array; replace_one; &pat, replace)
}

#[pyfunction]
fn str_replace_all(array: PyObject, pattern: &str, replace: &str) -> Result<PyObject, StringpyErr> {
    let pat = &Regex::new(pattern)?;

    fn replace_all<'a>(x: Option<&'a str>, pat: &Regex, replace: &str) -> Option<Cow<'a, str>> {
        let x = x?;
        return Some(Cow::from(pat.replace_all(x, replace)));
    }

    apply_utf8!(array; replace_all; &pat, replace)
}

#[pyfunction]
fn str_squish(ob: PyObject) -> Result<PyObject, StringpyErr> {
    fn squish(x: Option<&str>) -> Option<Cow<str>> {
        let x = x?;
        let a: Vec<_> = x.split_whitespace().collect();
        return Option::Some(Cow::from(a.join(" ")));
    }
    utils::apply_utf8!(ob; squish;)
}

#[pyfunction]
fn str_trim(array: PyObject, side: &str) -> Result<PyObject, StringpyErr> {
    if !["left", "right", "both"].contains(&side) {
        return Err(StringpyErr::new_value_err(
            "side must be one of 'left', 'right', 'both'".to_string(),
        ));
    }

    fn trim<'a>(x: Option<&'a str>, side: &str) -> Option<Cow<'a, str>> {
        let x = x?;
        let out = match side {
            "left" => x.trim_start(),
            "right" => x.trim_end(),
            "both" => x.trim(),
            _ => return None,
        };
        return Some(Cow::from(out));
    }
    apply_utf8!(array; trim; side)
}

#[pyfunction]
fn str_detect(array: PyObject, pattern: &str) -> Result<PyObject, StringpyErr> {
    let pat = Regex::new(pattern)?;

    fn detect(x: Option<&str>, pat: &Regex) -> Option<bool> {
        let x = x?;
        return Some(pat.is_match(x));
    }

    utils::apply_utf8_bool!(array; detect; &pat)
}

#[pyfunction]
fn str_remove_ascent(array: PyObject) -> Result<PyObject, StringpyErr> {
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
fn str_trunc(
    array: PyObject,
    width: usize,
    side: &str,
    ellipsis: &str,
) -> Result<PyObject, StringpyErr> {
    fn truncate<'a>(
        x: Option<&'a str>,
        width: usize,
        side: &str,
        ellipsis: &str,
    ) -> Option<Cow<'a, str>> {
        let x = x?;
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
    }

    apply_utf8!(array; truncate; width, side, ellipsis)
}

#[pyfunction]
fn str_remove(array: PyObject, pattern: &str) -> Result<PyObject, StringpyErr> {
    str_replace(array, pattern, "")
}

#[pyfunction]
fn str_remove_all(array: PyObject, pattern: &str) -> Result<PyObject, StringpyErr> {
    str_replace_all(array, pattern, "")
}

#[pyfunction]
fn str_extract(
    array: PyObject,
    pattern: &str,
    group: Option<usize>,
) -> Result<PyObject, StringpyErr> {
    let pat = Regex::new(pattern)?;

    if group.is_some() {
        if group.unwrap() >= pat.captures_len() {
            return Err(StringpyErr::new_value_err(format!(
                "Group {} does not exist in `{}`",
                group.unwrap(),
                pattern
            )));
        }
    }

    fn extract<'a>(x: Option<&'a str>, pat: &Regex, group: Option<usize>) -> Option<Cow<'a, str>> {
        let x = x?;
        if let Some(grp) = group {
            return pat
                .captures(x)
                .map(|x| Cow::from(x.get(grp).unwrap().as_str()));
        } else {
            return pat.find(x).map(|x| Cow::from(x.as_str()));
        }
    }

    apply_utf8!(array; extract; &pat, group)
}

//Vec<Option<Vec<Option<String>>>>
#[pyfunction]
fn str_extract_all(
    array: PyObject,
    pattern: &str,
    group: Option<usize>,
) -> Result<PyObject, StringpyErr> {
    let pat = Regex::new(pattern)?;

    if group.is_some() {
        if group.unwrap() >= pat.captures_len() {
            return Err(StringpyErr::new_value_err(format!(
                "Group {} does not exist in `{}`",
                group.unwrap(),
                pattern
            )));
        }
    }

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
        let array = arrow_in::to_rust_array(array, py)?;
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
    Ok(result?)
}

#[pyfunction]
fn str_split(array: PyObject, pattern: &str, n: Option<usize>) -> Result<PyObject, StringpyErr> {
    let pat = Regex::new(pattern)?;
    let n = n.unwrap_or(usize::MAX);

    fn split<'a>(x: Option<&'a str>, pat: &Regex, n: usize) -> Option<Vec<Option<String>>> {
        let x = x?;
        let a = pat.splitn(x, n).map(|i| Some(i.to_string())).collect();
        Some(a)
    }

    let result = Python::with_gil(|py| {
        let array = arrow_in::to_rust_array(array, py)?;
        let array = array.as_any();
        let mut array: Vec<Option<Vec<Option<String>>>> = array
            .downcast_ref::<Utf8Array<i32>>()
            .unwrap()
            .iter()
            .map(|i| split(i, &pat, n))
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
    Ok(result?)
}

#[pyfunction]
fn str_starts(array: PyObject, pattern: &str, negate: bool) -> Result<PyObject, StringpyErr> {
    let pattern = escape(pattern);
    let pattern = format!("^{}", pattern);
    let pat = Regex::new(pattern.as_str())?;

    utils::apply_utf8_bool!(array; atomic::detect; &pat, negate)
}

#[pyfunction]
fn str_ends(array: PyObject, pattern: &str, negate: bool) -> Result<PyObject, StringpyErr> {
    let pattern = escape(pattern);
    let pattern = format!("{}$", pattern);
    let pat = Regex::new(pattern.as_str())?;

    utils::apply_utf8_bool!(array; atomic::detect; &pat, negate)
}

#[pyfunction]
fn str_subset(array: PyObject, pattern: &str, negate: bool) -> Result<PyObject, StringpyErr> {
    let pat = Regex::new(pattern)?;

    // if match return x, esle return None
    fn subset<'a>(x: Option<&'a str>, pat: &Regex, negate: bool) -> Option<&'a str> {
        let x = x?;
        let a = pat.is_match(x);
        if negate {
            (!a).then(|| x)
        } else {
            a.then(|| x)
        }
    }

    let result = Python::with_gil(|py| {
        let array = arrow_in::to_rust_array(array, py)?;
        let array = array.as_any();
        let array: Vec<Option<&str>> = array
            .downcast_ref::<Utf8Array<i32>>()
            .unwrap_or_else(|| panic!("Expected String Array"))
            .iter()
            .filter(|x| subset(*x, &pat, negate).is_some())
            .collect();

        let result = arrow2::array::Utf8Array::<i32>::from(array);
        let result = Box::new(result);
        Ok(arrow_in::to_py_array(result, py)?)
    });
    result
}

#[pyfunction]
fn str_which(array: PyObject, pattern: &str, negate: bool) -> Result<PyObject, StringpyErr> {
    //let pat = Regex::new(pattern).map_err(|_| PyValueError::new_err(format!("Invalid regex pattern: {}", pattern)))?;
    let pat = Regex::new(pattern)?;

    let result = Python::with_gil(|py| {
        let array = arrow_in::to_rust_array(array, py)?;
        let len = array.len();
        let index: Vec<usize> = (0..len).collect();

        let array = array.as_any();
        let array: Vec<Option<bool>> = array
            .downcast_ref::<Utf8Array<i32>>()
            .unwrap()
            .iter()
            .map(|x| atomic::detect(x, &pat, negate))
            .collect();

        let array = zip(index, array)
            .filter(|(_, y)| y.unwrap_or(false))
            .map(|(x, _)| Some(x as i32))
            .collect::<Vec<Option<i32>>>();

        let result = arrow2::array::Int32Array::from(array);
        let result = Box::new(result);
        Ok(arrow_in::to_py_array(result, py)?)
    });
    result
}

#[pyfunction]
fn str_dup(array: PyObject, times: Vec<usize>) -> Result<PyObject, StringpyErr> {
    fn repeat(x: Option<&str>, times: usize) -> Option<Cow<str>> {
        let x = x?;
        Some(Cow::Owned(x.repeat(times)))
    }
    utils::apply_utf8!(array; repeat; times;)
}

#[pyfunction]
fn str_length(array: PyObject) -> Result<PyObject, StringpyErr> {
    fn length(x: Option<&str>) -> Option<i32> {
        let x = x?;
        Some(unidecode::unidecode(x).len() as i32)
    }

    utils::apply_utf8_i32!(array; length;)
}

#[pyfunction]
fn str_unique(array: PyObject) -> Result<PyObject, StringpyErr> {
    let result = Python::with_gil(|py| {
        let array = arrow_in::to_rust_array(array, py)?;
        let array = array.as_any();
        let array: Vec<Option<&str>> = array
            .downcast_ref::<Utf8Array<i32>>()
            .unwrap()
            .iter()
            .unique()
            .collect();

        let result = arrow2::array::Utf8Array::<i32>::from(array);
        let result = Box::new(result);
        Ok(arrow_in::to_py_array(result, py)?)
    });
    result
}

#[pyfunction]
fn str_to_upper(array: PyObject) -> Result<PyObject, StringpyErr> {
    fn to_upper(x: Option<&str>) -> Option<Cow<str>> {
        let x = x?;
        Some(Cow::Owned(x.to_uppercase()))
    }
    utils::apply_utf8!(array; to_upper;)
}

#[pyfunction]
fn str_to_lower(array: PyObject) -> Result<PyObject, StringpyErr> {
    fn to_lower(x: Option<&str>) -> Option<Cow<str>> {
        let x = x?;
        Some(Cow::Owned(x.to_lowercase()))
    }
    utils::apply_utf8!(array; to_lower;)
}

#[pyfunction]
fn str_to_title(array: PyObject) -> Result<PyObject, StringpyErr> {
    utils::apply_utf8!(array; atomic::to_upper; " ")
}

#[pyfunction]
fn str_to_sentence(array: PyObject) -> Result<PyObject, StringpyErr> {
    utils::apply_utf8!(array ; atomic::to_upper; ". ")
}

#[pyfunction]
fn str_pad(
    array: PyObject,
    width: Vec<i32>,
    side: Vec<&str>,
    pad: Vec<char>,
) -> Result<PyObject, StringpyErr> {
    // if ! ["left", "right", "both"].contains(&side) {
    //     return Err(StringpyErr::new_value_err(format!("Invalid side: `{}`. Must be one of [left, right, both]", side)));
    // }
    let width = width
        .into_iter()
        .map(|x| x as usize)
        .collect::<Vec<usize>>();

    fn padding<'a>(
        x: Option<&'a str>,
        width: usize,
        side: &str,
        pad: char,
    ) -> Option<Cow<'a, str>> {
        let x = x?;
        let lenth = x.len();
        if width < lenth {
            return Some(Cow::Borrowed(x));
        } else {
            let pad = pad.to_string().repeat(width - lenth);
            match side {
                "left" => Some(Cow::Owned(pad + x)),
                "right" => Some(Cow::Owned(x.to_string() + &pad.as_str())),
                "both" => {
                    let pad_left = pad.chars().take(pad.len() / 2).collect::<String>();
                    let pad_right = pad.chars().skip(pad.len() / 2).collect::<String>();
                    Some(Cow::Owned(pad_left + x + &pad_right))
                }
                _ => Some(Cow::Borrowed(x)),
            }
        }
    }
    apply_utf8!(array ; padding; width, side , pad;)
}

#[pyfunction]
fn str_sub(array: PyObject, start: Vec<i32>, end: Vec<i32>) -> Result<PyObject, StringpyErr> {
    fn sub(x: Option<&str>, start: i32, end: i32) -> Option<Cow<str>> {
        let x = x?;
        let len = x.len();

        let start = if start >= len as i32 {
            len
        } else if (start >= 0) & (start < len as i32) {
            start as usize
        } else if (start < 0) & (start > -(len as i32)) {
            len - (-start as usize)
        } else {
            0
        };

        let end = if end >= len as i32 {
            len
        } else if (end >= 0) & (end < len as i32) {
            end as usize
        } else if (end < 0) & (end > -(len as i32)) {
            len - (-end as usize)
        } else {
            0
        };

        Some(Cow::Owned(x[start..end].to_string()))
    }
    utils::apply_utf8!(array; sub; start, end;)
}

#[pyfunction]
fn str_match(array: PyObject, pattern: &str) -> Result<PyObject, StringpyErr> {
    let pat = Regex::new(pattern)?;

    fn _match<'a>(x: Option<&'a str>, pat: &Regex) -> Option<Vec<Option<String>>> {
        let x = x?;
        let mut result = Vec::new();
        for i in pat.captures(x)?.iter().skip(1) {
            // skip group 0 which is implicit group
            // of whole match
            result.push(Some(i.unwrap().as_str().to_string()))
        }

        Some(result)
    }

    let result = Python::with_gil(|py| {
        let array = arrow_in::to_rust_array(array, py)?;
        let array = array.as_any();
        let mut array: Vec<Option<Vec<Option<String>>>> = array
            .downcast_ref::<Utf8Array<i32>>()
            .unwrap()
            .iter()
            .map(|i| _match(i, &pat))
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
    Ok(result?)
}

#[pyfunction]
fn str_locate(array: PyObject, pattern: &str) -> Result<PyObject, StringpyErr> {
    let pat = Regex::new(pattern).unwrap();

    fn find_loc<'a>(x: Option<&'a str>, pat: &Regex) -> Option<Vec<i32>> {
        let x = x?;
        let mut locs = pat.capture_locations();
        let _out = pat.captures_read(&mut locs, x)?;
        let (first, end) = locs.get(0)?;
        let out = Some(vec![first as i32, end as i32]);
        return out;
    }

    let result = Python::with_gil(|py| {
        let array = arrow_in::to_rust_array(array, py)?;
        let array = array.as_any();
        let array: Vec<Option<Vec<i32>>> = array
            .downcast_ref::<Utf8Array<i32>>()
            .unwrap()
            .iter()
            .map(|i| find_loc(i, &pat))
            .collect();

        let length_each: Vec<usize> = array
            .iter()
            .map(|_| 2) // always length 2
            .collect();

        // Vec<Option<Vec<usize>> to Vec<Option<usize>>
        let mut array2: Vec<Option<i32>> = Vec::with_capacity(length_each.iter().sum());
        array.iter().for_each(|x| {
            if let Some(x) = x {
                array2.push(Some(x[0]));
                array2.push(Some(x[1]));
            } else {
                array2.push(None);
                array2.push(None);
            }
        });

        let _field = Box::new(Field::new("_", DataType::Int32, true));
        let _list = DataType::List(_field);

        let offset = Offsets::try_from_iter(length_each.into_iter()).unwrap();
        let offset_buf = OffsetsBuffer::from(offset);
        let ar2 = Int32Array::from(array2);
        let b2: ListArray<i32> = ListArray::new(_list, offset_buf, Box::new(ar2), None);
        arrow_in::to_py_array(b2.boxed(), py)
    });

    Ok(result?)
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
    m.add_function(wrap_pyfunction!(str_split, m)?)?;
    m.add_function(wrap_pyfunction!(str_starts, m)?)?;
    m.add_function(wrap_pyfunction!(str_ends, m)?)?;
    m.add_function(wrap_pyfunction!(str_subset, m)?)?;
    m.add_function(wrap_pyfunction!(str_which, m)?)?;
    m.add_function(wrap_pyfunction!(str_dup, m)?)?;
    m.add_function(wrap_pyfunction!(str_length, m)?)?;
    m.add_function(wrap_pyfunction!(str_unique, m)?)?;
    m.add_function(wrap_pyfunction!(str_to_upper, m)?)?;
    m.add_function(wrap_pyfunction!(str_to_lower, m)?)?;
    m.add_function(wrap_pyfunction!(str_to_title, m)?)?;
    m.add_function(wrap_pyfunction!(str_to_sentence, m)?)?;
    m.add_function(wrap_pyfunction!(str_pad, m)?)?;
    m.add_function(wrap_pyfunction!(str_sub, m)?)?;
    m.add_function(wrap_pyfunction!(str_match, m)?)?;
    m.add_function(wrap_pyfunction!(str_locate, m)?)?;
    Ok(())
}
