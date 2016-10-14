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
use nom::{IResult};


pub use self::parser::*;
pub mod parser;
pub use self::generator::*;
pub mod generator;


// PyTuple, PyDict, ToPyObject, PythonObject
use cpython::{PyObject, PyResult, Python, PyString};

fn run(py: Python) -> PyResult<PyObject> {
    println!("Rust says: Hello Python!");
    Ok(py.None())
}


fn html(py: Python, input_str: PyString) -> PyResult<PyString> {
    // let mut v: Vec<Box<ToHtml>> = Vec::with_capacity(100);

    // let mut v = Vec::with_capacity(100);
    // let el = Node{text:"asd".to_string()};
    // v.push(Box::new("test".to_string()));
    // name_parser("hello Kimberly".as_bytes()) {
    //     IResult::Done(rest, output) => {
    //         println!("OK!");
    //     }
    // }


    // println!("Rust says: {}", s.to_string(py));
    // println!("Rust says: {}", r);

    match input_str.to_string(py) {
        Ok(input) => {
            let greetings = format!("Rust says: Greetings {} !", input);
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
            let output = PyString::new(py, &greetings);
            Ok(output)
            // Ok(py.None())
        }
        Err(e) => Err(e)
    }


    // Ok(py.None())
}

// fn add_two(py: Python, args: &PyTuple, _: Option<&PyDict>) -> PyResult<PyObject> {
//     match args.as_slice() {
//         [ref a_obj, ref b_obj] => {
//             let a = a_obj.extract::<i32>(py).unwrap();
//             let b = b_obj.extract::<i32>(py).unwrap();
//             let mut acc:i32 = 0;

//             for _ in 0..1000 {
//                 acc += a + b;
//             }

//             Ok(acc.to_py_object(py).into_object())
//         },
//         _ => Ok(py.None())
//     }
// }

py_module_initializer!(example, initexample, PyInit_example, |py, m| {
    // try!(module.add(py, "add_two", py_fn!(add_two)));
    try!(m.add(py, "__doc__", "Module documentation string"));
    try!(m.add(py, "run", py_fn!(py, run())));
    try!(m.add(py, "html", py_fn!(py, html(input: PyString))));
    Ok(())
});

// py_module_initializer!(example, initexample, PyInit_example, |py, m| {
//     try!(m.add(py, "hello", py_fn!(py, hello())));
//     Ok(())
// });
