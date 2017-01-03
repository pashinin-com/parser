

use cpython::{Python, ToPyObject, PyObject, PythonObject, PyTuple, PyString, PyResult};
use std::borrow::Cow;
// use generator::{Html};
pub mod parser;
pub mod node;
use self::parser::{parse};
use nom::{IResult};
use std::fmt;
use self::node::{Node, NodeClass};
use std::cell;


py_class!(class Article |py| {
    data src: cell::RefCell<PyString>;
    data tree: cell::RefCell<Node>;

    def __new__(_cls, src: PyString) -> PyResult<Article> {
        let n = match parse(src.to_string_lossy(py).as_bytes()) {
            IResult::Done(_, root) => root,
            _ => Node{
                children: None,
                params: None,
                class: NodeClass::Root,
            }
        };
        Article::create_instance(py, cell::RefCell::new(src), cell::RefCell::new(n))
    }
    def load(&self, src: PyString) -> PyResult<PyObject> {
        *self.src(py).borrow_mut() = src;
        *self.tree(py).borrow_mut() =
            match parse(self.src(py).borrow().to_string_lossy(py).clone().as_bytes()) {
                IResult::Done(_, root) => {root}
                _ => Node{
                    children: None,
                    params: None,
                    class: NodeClass::Root,
                }
            };
        Ok(py.None().into_object())
    }

    def source(&self) -> PyResult<PyObject> {
        let ref s = *self.src(py).borrow();
        Ok(PyString::new(py, s.to_string(py).unwrap().as_ref()).into_object())
    }

    def render(&self) -> PyResult<PyString> {
        Ok(PyString::new(py, &*self.tree(py).borrow().to_string()))
    }
});
