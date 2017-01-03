//! lib.rs is the main file which has `py_module_initializer!` macro.
//!
//! It defines an entry point in .SO library with exported python
//! functions.

// #![deny(missing_docs,
//         missing_debug_implementations, missing_copy_implementations,
//         trivial_casts, trivial_numeric_casts,
//         unsafe_code,
//         unstable_features,
//         unused_import_braces, unused_qualifications)]
// #[macro_use]
// extern crate itertools;

#[macro_use]
extern crate cpython;

#[macro_use]
extern crate nom;
use cpython::ToPyObject;

mod common;
pub mod article;

use nom::{IResult};
use self::article::node::{Node, NodeClass};
use self::article::parser::{parse};
use self::article::Article;
use cpython::PythonObject;
use std::cell;
use cpython::{PyResult, PyString, PyObject};
use std::borrow::Cow;


py_class!(class Markdown |py| {
    data src: cell::RefCell<PyString>;
    data tree: cell::RefCell<Node>;
    // self::article::node::Node

    def __new__(_cls, src: PyString) -> PyResult<Markdown> {
        let n = match parse(src.to_string_lossy(py).as_bytes()) {
            IResult::Done(_, root) => root,
            _ => Node{
                children: None,
                params: None,
                class: NodeClass::Root,
            }
        };
        Markdown::create_instance(py, cell::RefCell::new(src), cell::RefCell::new(n))
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


///
/// Main Python lib init function
///
/// Docs about this macros:
/// http://dgrunwald.github.io/rust-cpython/doc/cpython/macro.py_module_initializer.html
///
/// Arguments:
///
/// 1. name: The module name as a Rust identifier.
/// 2. py2_init: "init" + $name. Necessary because macros can't use concat_idents!().
/// 3. py3_init: "PyInit_" + $name. Necessary because macros can't use concat_idents!().
///
py_module_initializer!(librparser, initlibrparser, PyInit_librparser, |py, m| {
    // try!(module.add(py, "add_two", py_fn!(add_two)));
    try!(m.add(py, "__doc__", "Module documentation string"));
    // try!(m.add(py, "AST", py_fn!(py, ASTree())));
    try!(m.add_class::<Article>(py));
    try!(m.add_class::<Markdown>(py));
    // try!(m.add(py, "article_ast", py_fn!(py, article_ast(input: PyString))));
    // try!(m.add(py, "article", py_fn!(py, article(input: PyString))));
    Ok(())
});
