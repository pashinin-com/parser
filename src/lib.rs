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
pub use self::parser::*;
mod common;
pub mod parser;
mod article;
mod node;
use self::article::*;
use cpython::PythonObject;


use cpython::{PyResult, Python, PyString, PyObject};

// fn ast2(py: Python, input_str: PyString) -> PyResult<PyString> {
//     // println!("Rust says: {}", s.to_string(py));
//     // let res = parse(&input);
//     // let a = Article::new(input_str);

//     match input_str.to_string(py) {
//         // Ok(input) => {
//         Ok(s) => {
//             // let greetings = format!("Rust says: Greetings {} !", input);
//             // Value::Svalue(ScalarValue::Integer32(3))]
//             // let greetings = format!("Rust says: Greetings {} !", input);
//             // let v: Vec<Box<ToHtml>> = parse(input.to_string());


//             // let res = category(ini_file);
//             // let res = category(&input);
//             // let res = command(&input);
//             // // println!("Object: {:?}", res);
//             // match res {
//             //     IResult::Done(_, o) => {
//             //         // println!("i: {} | o: {:?}", i, o);
//             //         return Ok(PyString::new(py, &o));
//             //     },
//             //     // IResult::Incomplete(x) => println!("incomplete: {:?}", x),
//             //     // IResult::Error(e) => println!("error: {:?}", e)
//             //     _ => println!("error")
//             // }

//             // let r = generate_html(&v);
//             // Ok(PyString::new(py, &r))

//             // string
//             // let output = PyString::new(py, &greetings);
//             // Ok(output)
//             let a = Article::new(s);

//             // Ok(PyTuple::empty(py))
//             Ok(a.to_py_object(py))
//             // Ok(py.None())
//         }
//         Err(e) => Err(e)
//     }
// }


// use cpython::{PyDict};

py_class!(class AST |py| {
    data number: i32;

    def __new__(_cls, arg: i32) -> PyResult<AST> {
        AST::create_instance(py, arg)
    }
    def half(&self) -> PyResult<i32> {
        println!("half() was called with self={:?}", self.number(py));
        Ok(self.number(py) / 2)
    }
});

/// Render article to HTML
fn article(py: Python, input: PyString) -> PyResult<PyString> {
    match input.to_string(py) {
        Ok(s) => {
            let a = Article::new(s);
            // Ok(a.to_py_object(py))
            Ok(PyString::new(py, &format!("{}", a.to_string())))
        }
        Err(e) => Err(e)
    }
}


fn article_ast(py: Python, input: PyString) -> PyResult<PyObject> {
    match input.to_string(py) {
        Ok(s) => {Ok(Article::new(s).to_py_object(py).into_object())}
        Err(e) => Err(e)
    }
}

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
    try!(m.add(py, "article_ast", py_fn!(py, article_ast(input: PyString))));
    try!(m.add(py, "article", py_fn!(py, article(input: PyString))));
    Ok(())
});
