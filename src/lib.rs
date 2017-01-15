extern crate cpython;
extern crate cpython_json;
extern crate serde_json;

pub use cpython::{PyResult, PyObject};
pub use serde_json::value::Value;

pub type LambdaResult = Result<Value, Box<std::error::Error>>;

use cpython::{Python, PyUnicode, PyErr, PythonObject, PythonObjectWithTypeObject};
use cpython_json::{from_json, to_json};

macro_rules! pyerr {
    ($py:expr, $x:expr) => { $x.or_else(|e| Err(e.to_pyerr($py))) }
}

pub fn handler<F>(py: Python, f: F, py_event: PyObject, _: PyObject) -> PyResult<PyObject>
    where F: Fn(Value) -> LambdaResult
{
    let event = pyerr!(py, to_json(py, &py_event))?;
    let result = match f(event) {
        Ok(r) => r,
        Err(e) => {
            return Err(PyErr {
                ptype: cpython::exc::RuntimeError::type_object(py).into_object(),
                pvalue: Some(PyUnicode::new(py, &format!("{:?}", e)).into_object()),
                ptraceback: None,
            })
        }
    };
    pyerr!(py, from_json(py, result))
}

#[macro_export]
macro_rules! lambda {
    ($f:expr) => {
        py_module_initializer!(liblambda, initliblambda, PyInit_liblambda, |py, m| {
            try!(m.add(py, "handler",
                       py_fn!(py, x(event: $crate::PyObject,
                                    context: $crate::PyObject)
                                    -> $crate::PyResult<$crate::PyObject> {
                           $crate::handler(py, $f, event, context)
                       })));
            Ok(())
        });
    }
}
