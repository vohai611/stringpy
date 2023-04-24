use crate::arrow_in;
use arrow2::array::Utf8Array;
use pyo3::{prelude::*, types::PyTuple};

#[pyfunction]
fn str_c(ob: PyObject,  collapse: Option<&str>) -> PyResult<String> {
    let collapse = collapse.unwrap_or(""); 
    let mut result = Python::with_gil(|py| {
        let array = arrow_in::to_rust_array(ob, py).unwrap();
        let array = array.as_any();
        let array = array
            .downcast_ref::<Utf8Array<i32>>()
            .unwrap();


        array.iter().map(|i| {
                let mut val = i.unwrap_or("").to_string();
                val.push_str(collapse);
                val
                })
            .reduce(|x, y| x + &y)
            .unwrap()
    
    });
    // let result_len = result.len();
    // let end =  result_len - sep.len();
    // let result = result[0..end].to_owned();
    let len = collapse.len();
    for _ in 0..len {
        result.pop();
    }

    Ok(result)
}

#[pyfunction]
#[pyo3(signature = (*py_args, sep))]
fn str_combine(py_args : &PyTuple, sep: Option<&str>) -> PyResult<Vec<String>> {
    let sep = sep.unwrap_or(""); 

        fn list_array(ob: PyObject, py: Python) -> Utf8Array<i32> {

            let array = arrow_in::to_rust_array(ob, py).unwrap();
            let array = array.as_any();
            let array = array
                .downcast_ref::<Utf8Array<i32>>()
                .unwrap();
            array.to_owned()

        }

        let mut a: Vec<Utf8Array<i32>> = Python::with_gil(|py| {
         py_args
        .into_iter()
        .map(|ob| list_array(ob.to_object(py), py))
        .collect()
        });

        let last_element = a.pop().unwrap();

        let rs: Vec<String> = vec![String::with_capacity(50); a[0].len()];

        fn combine_two(mut x1:  Vec<String> ,x2: &Utf8Array<i32>, sep: &str) -> Vec<String> {
           x1.iter_mut().zip(x2.iter()).for_each(|(v1, v2)| {
                v1.push_str(v2.unwrap());
                v1.push_str(sep);
            });
            x1
        }

    let rs = a.iter().fold(rs, |x1,x2| {combine_two(x1, x2, sep)});
    let rs = combine_two(rs,&last_element, "");


    Ok(rs)



}

#[pymodule]
fn _stringpy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(str_c, m)?)?;
    m.add_function(wrap_pyfunction!(str_combine, m)?)?;
    Ok(())
}
