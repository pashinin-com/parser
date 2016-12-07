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
// use nom::{IResult, space, alpha, alphanumeric, digit};
// use nom::{IResult};
// use nom::IResult::*;
use cpython::ToPyObject;

pub use self::parser::*;
pub mod parser;
pub use self::generator::*;
pub mod generator;

mod paragraph;
mod article;
mod node;
use self::article::*;


// PyTuple, PyDict, ToPyObject, PythonObject
use cpython::{PyObject, PyResult, Python, PyString, PyTuple};

fn run(py: Python) -> PyResult<PyObject> {
    println!("Rust says: Hello Python!");
    Ok(py.None())
}


fn ast(py: Python, input_str: PyString) -> PyResult<PyString> {
    // println!("Rust says: {}", s.to_string(py));
    // let res = parse(&input);
    // let a = Article::new(input_str);

    match input_str.to_string(py) {
        // Ok(input) => {
        Ok(s) => {
            // let greetings = format!("Rust says: Greetings {} !", input);
            // Value::Svalue(ScalarValue::Integer32(3))]
            // let greetings = format!("Rust says: Greetings {} !", input);
            // let v: Vec<Box<ToHtml>> = parse(input.to_string());


            // let res = category(ini_file);
            // let res = category(&input);
            // let res = command(&input);
            // // println!("Object: {:?}", res);
            // match res {
            //     IResult::Done(_, o) => {
            //         // println!("i: {} | o: {:?}", i, o);
            //         return Ok(PyString::new(py, &o));
            //     },
            //     // IResult::Incomplete(x) => println!("incomplete: {:?}", x),
            //     // IResult::Error(e) => println!("error: {:?}", e)
            //     _ => println!("error")
            // }

            // let r = generate_html(&v);
            // Ok(PyString::new(py, &r))

            // string
            // let output = PyString::new(py, &greetings);
            // Ok(output)
            let a = Article::new(s);

            // Ok(PyTuple::empty(py))
            Ok(
                a.to_py_object(py)
                    // .into_object()
                    // .to_py_object(py)
                // PyTuple::new(py, &Vec::new())
                // &PyString::new(py, &greetings)
                // Value::Svalue(ScalarValue::Integer32(1)),
                // Value::Svalue(ScalarValue::Integer32(2))
            )
            // Ok(py.None())
        }
        Err(e) => Err(e)
    }
}

fn html(py: Python, input_str: PyString) -> PyResult<PyString> {
    match input_str.to_string(py) {
        Ok(s) => {
            let a = Article::new(s);
            Ok(a.to_py_object(py))
        }
        Err(e) => Err(e)
    }
}

py_module_initializer!(parser, initparser, PyInit_parser, |py, m| {
    // try!(module.add(py, "add_two", py_fn!(add_two)));
    try!(m.add(py, "__doc__", "Module documentation string"));
    try!(m.add(py, "run", py_fn!(py, run())));
    try!(m.add(py, "ast", py_fn!(py, ast(input: PyString))));
    try!(m.add(py, "html", py_fn!(py, html(input: PyString))));
    Ok(())
});
