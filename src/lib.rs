extern crate cpython;
extern crate cpython_json;
extern crate serde_json;

pub use cpython::{PyResult, PyObject};
pub use serde_json::value::Value;

pub type LambdaResult = Result<Value, Box<std::error::Error>>;

use cpython::{Python, PyUnicode, PyTuple, PyErr, PythonObject, PythonObjectWithTypeObject,
              ObjectProtocol};
use cpython_json::{from_json, to_json};

pub struct LambdaContext<'a> {
    py: &'a Python<'a>,
    py_context: &'a PyObject,
    string_storage: [String; 7],
}

macro_rules! str_getters {
    (@step $_idx:expr,) => {};

    (@step $idx:expr, $x:ident, $($xs:ident,)*) => {
        pub fn $x(&self) -> &str {
            &self.string_storage[$idx]
        }

        str_getters!(@step $idx + 1usize, $($xs,)*);
    };

    ($($xs:ident),*) => {
        str_getters!(@step 0usize, $($xs,)*);
    };
}

impl<'a> LambdaContext<'a> {
    fn new(py: &'a Python, py_context: &'a PyObject) -> PyResult<LambdaContext<'a>> {
        macro_rules! str_attr {
            ($x:expr) => {
                py_context.getattr(*py, $x)?.extract::<String>(*py)?;
            }
        }

        let string_storage: [String; 7] = [str_attr!("function_name"),
                                           str_attr!("function_version"),
                                           str_attr!("invoked_function_arn"),
                                           str_attr!("memory_limit_in_mb"),
                                           str_attr!("aws_request_id"),
                                           str_attr!("log_group_name"),
                                           str_attr!("log_stream_name")];

        Ok(LambdaContext {
            py: py,
            py_context: py_context,
            string_storage: string_storage,
        })
    }

    str_getters!(function_name,
                 function_version,
                 invoked_function_arn,
                 memory_limit_in_mb,
                 aws_request_id,
                 log_group_name,
                 log_stream_name);

    pub fn get_remaining_time_in_millis(&self) -> Result<u64, ContextError> {
        self.py_context
            .call_method(*self.py,
                         "get_remaining_time_in_millis",
                         PyTuple::new(*self.py, &[]),
                         None)
            .and_then(|x| x.extract::<u64>(*self.py))
            .map_err(|_| ContextError::GetRemainingTimeFailed)
    }
}

#[derive(Debug)]
pub enum ContextError {
    GetRemainingTimeFailed,
}

impl std::fmt::Display for ContextError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ContextError::GetRemainingTimeFailed => {
                write!(f, "failed to call get_remaining_time_in_millis")
            }
        }
    }
}

impl std::error::Error for ContextError {
    fn description(&self) -> &str {
        match *self {
            ContextError::GetRemainingTimeFailed => "failed to call get_remaining_time_in_millis",
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        None
    }
}

pub fn handler<F>(py: Python, f: F, py_event: PyObject, py_context: PyObject) -> PyResult<PyObject>
    where F: Fn(Value, LambdaContext) -> LambdaResult
{
    let event = to_json(py, &py_event).or_else(|e| Err(e.to_pyerr(py)))?;
    let result = match f(event, LambdaContext::new(&py, &py_context)?) {
        Ok(r) => r,
        Err(e) => {
            return Err(PyErr {
                ptype: cpython::exc::RuntimeError::type_object(py).into_object(),
                pvalue: Some(PyUnicode::new(py, &format!("{:?}", e)).into_object()),
                ptraceback: None,
            })
        }
    };
    from_json(py, result).or_else(|e| Err(e.to_pyerr(py)))
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
