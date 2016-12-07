

use cpython::{PyObject, PyResult, Python, PyString, PyTuple, ToPyObject};
use std::borrow::Cow;
use generator::{Html};
use parser::{parse};
use nom::{IResult};
// use core::slice::SliceExt;

// mod paragraph;
use paragraph::Paragraph;

// pub use self::article::Article;
// pub mod generator;

#[derive(PartialEq,Eq,Debug)]
pub struct Article<'a> {
    // pub struct Paragraph {
    // pub elements: Vec<Box<Node<'a>+'a>>,
    // pub elements: Option<Vec<Box<Node>>>,
    // pub elements: Box<Vec<Box<Node>>>,
    // pub children: PyTuple,
    // pub name: &'a str,
    // pub src: &'a str,
    // html: Cow<'a, str>,
    paragraphs: Vec<Paragraph<'a> >,
    src: Cow<'a, str>,
}
impl<'a> Article<'a> {
    pub fn new<S>(src: S) -> Article<'a>
        where S: Into<Cow<'a, str>>
    {
        Article {
            src: src.into(),
            paragraphs: vec![],
            // html: "".into(),
        }
    }

    fn ast(py: Python, input_str: PyString) -> PyResult<PyTuple> {
        match input_str.to_string(py) {
            Ok(_) => {
                Ok(PyTuple::new(py, &Vec::new()))
                // Ok(PyTuple::new(py, &PyString::new(py, "asd")))
            }
            Err(e) => Err(e)
        }
        // Ok(PyTuple::new(py, &[input_str]))
    }

}

///
/// Convert article to a python string (PyString)
///
impl<'a> ToPyObject for Article<'a>{
    type ObjectType = PyString;
    // type ObjectType = Article<'a>;

    #[inline]
    fn to_py_object(&self, py: Python) -> PyString {
        // PyString::new(py, &self.src)
        PyString::new(py, &self.html())
    }
}


impl<'a> Html for Article<'a>{
    fn html(&self) -> String {
        // impl Html for Vec<Command> {
        //     fn html(&self) -> String {
        //         self.iter().fold("".to_string(),
        //                          |mut i,j| {i.push_str(&*j.html()); i})     // &*j.html()
        //     }
        // }

        let s = self.src.to_string();
        let parsed = parse(s.as_bytes());
        match parsed {
            IResult::Done(_, node) => {
                // println!("i: {} | o: {:?}", i, o);
                // return Ok(PyString::new(py, &o));


                // prev:
                // v.iter().fold("".to_string(),
                //               |mut i,j| {i.push_str(&*j.to_string()); i})     // &*j.html()
                node.to_string()

            },
            IResult::Incomplete(x) => format!("incomplete, rest: {:?}!", x).to_string(),
            IResult::Error(e) => format!("Parsing error: {:?}", e).to_string(),
            // println!("Parsing error: {:?}", e)
            // IResult::Error(e) => "parsing error".to_string(),
            // _ => "WTF: article to html".to_string()
        }

        // v.len().to_string()

        // self.iter().fold("".to_string(),
        //                  |mut i,j| {i.push_str(&*j.html()); i})     // &*j.html()

        // self.src.to_string()
        // PyString::new(&self.src)
        // PyString::new(py, "art")
    }
}
